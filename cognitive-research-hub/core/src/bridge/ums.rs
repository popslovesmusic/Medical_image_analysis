use std::f32::consts::PI;

use crate::{
    tensor::{rgb_to_hsl, spectral_energy, ChromaticTensor, SpectralTensor},
    Fx, HUE_CATEGORIES, UMS_CHROMATIC_BANDS, UMS_CHROMATIC_OFFSET, UMS_DIM, UMS_SPECTRAL_BANDS,
    UMS_TEMPORAL_BANDS, UMS_TEMPORAL_OFFSET,
};

use super::{hue_to_bin_weights, map_luminance_to_sigma, mean_hsl, normalize_hue};

const SPECTRAL_AMPLITUDE_BANDS: usize = UMS_SPECTRAL_BANDS / 2;
const SPECTRAL_SIGMA_OFFSET: usize = SPECTRAL_AMPLITUDE_BANDS;

/// Unified Modality Space (UMS) vector storing spectral, chromatic, and temporal features.
#[derive(Clone, Debug, PartialEq)]
pub struct UnifiedModalitySpace {
    data: [Fx; UMS_DIM],
}

impl UnifiedModalitySpace {
    /// Creates a zero-initialised UMS vector.
    pub fn new() -> Self {
        Self {
            data: [0.0; UMS_DIM],
        }
    }

    /// Returns the raw slice backing the UMS vector.
    pub fn as_slice(&self) -> &[Fx] {
        &self.data
    }

    /// Returns a mutable slice to the underlying data for in-place updates.
    pub fn as_mut_slice(&mut self) -> &mut [Fx] {
        &mut self.data
    }

    /// Returns the spectral amplitude segment (0..SPECTRAL_AMPLITUDE_BANDS).
    pub fn spectral_amplitudes(&self) -> &[Fx] {
        &self.data[..SPECTRAL_AMPLITUDE_BANDS]
    }

    /// Returns the spectral sigma segment.
    pub fn spectral_bandwidths(&self) -> &[Fx] {
        let end = SPECTRAL_SIGMA_OFFSET.saturating_add(SPECTRAL_AMPLITUDE_BANDS);
        &self.data[SPECTRAL_SIGMA_OFFSET..end]
    }

    /// Returns the chromatic histogram slice (first HUE_CATEGORIES slots of the chromatic band).
    pub fn chromatic_histogram(&self) -> &[Fx] {
        let start = UMS_CHROMATIC_OFFSET;
        let end = start
            .saturating_add(HUE_CATEGORIES)
            .min(UMS_CHROMATIC_OFFSET.saturating_add(UMS_CHROMATIC_BANDS));
        &self.data[start..end]
    }

    /// Returns the entire chromatic band slice.
    pub fn chromatic_slice(&self) -> &[Fx] {
        let start = UMS_CHROMATIC_OFFSET;
        let end = start.saturating_add(UMS_CHROMATIC_BANDS).min(UMS_DIM);
        &self.data[start..end]
    }
}

impl Default for UnifiedModalitySpace {
    fn default() -> Self {
        Self::new()
    }
}

/// Projects chromatic and spectral tensors into the Unified Modality Space.
pub fn project_to_ums(
    chromatic: &ChromaticTensor,
    spectral: &SpectralTensor,
) -> UnifiedModalitySpace {
    assert!(
        SPECTRAL_AMPLITUDE_BANDS > 0,
        "UMS spectral band configuration invalid"
    );
    assert!(
        UMS_CHROMATIC_BANDS >= HUE_CATEGORIES,
        "UMS chromatic band insufficient for hue histogram",
    );
    assert!(
        UMS_TEMPORAL_OFFSET < UMS_DIM,
        "UMS temporal offset must lie within the vector",
    );
    let mut ums = UnifiedModalitySpace::new();
    populate_spectral(&mut ums, spectral);
    populate_chromatic(&mut ums, chromatic);
    populate_temporal(&mut ums, spectral);
    ums
}

/// Reconstructs the mean chromatic representation encoded in the UMS vector.
pub fn reconstruct_chromatic_from_ums(ums: &UnifiedModalitySpace) -> (Fx, Fx, Fx) {
    let slice = ums.chromatic_slice();
    if slice.len() <= HUE_CATEGORIES {
        return (0.0, 0.0, 0.5);
    }
    let cos_slot = slice[HUE_CATEGORIES];
    let sin_slot = if slice.len() > HUE_CATEGORIES + 1 {
        slice[HUE_CATEGORIES + 1]
    } else {
        0.0
    };
    let mut hue = sin_slot.atan2(cos_slot);
    if hue < 0.0 {
        hue += 2.0 * PI;
    }
    let saturation = if slice.len() > HUE_CATEGORIES + 2 {
        slice[HUE_CATEGORIES + 2].clamp(0.0, 1.0)
    } else {
        0.0
    };
    let luminance = if slice.len() > HUE_CATEGORIES + 3 {
        slice[HUE_CATEGORIES + 3].clamp(0.0, 1.0)
    } else {
        0.5
    };
    (normalize_hue(hue), saturation, luminance)
}

/// Reconstructs spectral amplitudes and bandwidths from the UMS vector.
pub fn reconstruct_spectral_from_ums(
    ums: &UnifiedModalitySpace,
    bins: usize,
) -> (Vec<Fx>, Vec<Fx>) {
    if bins == 0 {
        return (Vec::new(), Vec::new());
    }
    let mut amplitudes = vec![0.0; bins];
    let mut sigmas = vec![map_luminance_to_sigma(0.5); bins];
    let amp_src = ums.spectral_amplitudes();
    let sigma_src = ums.spectral_bandwidths();
    if bins <= SPECTRAL_AMPLITUDE_BANDS {
        for idx in 0..bins {
            amplitudes[idx] = amp_src[idx];
            sigmas[idx] = sigma_src[idx];
        }
        return (amplitudes, sigmas);
    }
    for (idx, amp_dst) in amplitudes.iter_mut().enumerate() {
        let mut start = idx
            .saturating_mul(SPECTRAL_AMPLITUDE_BANDS)
            .saturating_div(bins.max(1));
        let mut end = (idx
            .saturating_add(1)
            .saturating_mul(SPECTRAL_AMPLITUDE_BANDS))
        .saturating_div(bins.max(1));
        if start >= SPECTRAL_AMPLITUDE_BANDS {
            start = SPECTRAL_AMPLITUDE_BANDS.saturating_sub(1);
        }
        end = end.min(SPECTRAL_AMPLITUDE_BANDS);
        if end <= start {
            end = start.saturating_add(1).min(SPECTRAL_AMPLITUDE_BANDS);
        }
        let slice = &amp_src[start..end];
        let len = slice.len() as Fx;
        if len > 0.0 {
            let sum = slice.iter().fold(0.0, |acc, &v| acc + v);
            *amp_dst = sum / len;
        }
    }

    for (idx, sigma_dst) in sigmas.iter_mut().enumerate() {
        let mut start = idx
            .saturating_mul(SPECTRAL_AMPLITUDE_BANDS)
            .saturating_div(bins.max(1));
        let mut end = (idx
            .saturating_add(1)
            .saturating_mul(SPECTRAL_AMPLITUDE_BANDS))
        .saturating_div(bins.max(1));
        if start >= SPECTRAL_AMPLITUDE_BANDS {
            start = SPECTRAL_AMPLITUDE_BANDS.saturating_sub(1);
        }
        end = end.min(SPECTRAL_AMPLITUDE_BANDS);
        if end <= start {
            end = start.saturating_add(1).min(SPECTRAL_AMPLITUDE_BANDS);
        }
        let slice = &sigma_src[start..end];
        let len = slice.len() as Fx;
        if len > 0.0 {
            let sum = slice.iter().fold(0.0, |acc, &v| acc + v);
            *sigma_dst = sum / len;
        }
    }
    (amplitudes, sigmas)
}

fn populate_spectral(ums: &mut UnifiedModalitySpace, spectral: &SpectralTensor) {
    let bins = spectral.bins.len();
    assert!(bins > 0, "spectral tensor requires at least one bin");
    if bins <= SPECTRAL_AMPLITUDE_BANDS {
        for idx in 0..bins {
            ums.data[idx] = spectral.bins[idx];
        }
        for idx in bins..SPECTRAL_AMPLITUDE_BANDS {
            ums.data[idx] = 0.0;
        }
        if let Some(sigmas) = spectral.sigma.as_ref() {
            for idx in 0..bins {
                let offset = SPECTRAL_SIGMA_OFFSET.saturating_add(idx);
                ums.data[offset] = sigmas[idx];
            }
            let default_sigma = map_luminance_to_sigma(0.5);
            for idx in bins..SPECTRAL_AMPLITUDE_BANDS {
                let offset = SPECTRAL_SIGMA_OFFSET.saturating_add(idx);
                ums.data[offset] = default_sigma;
            }
        } else {
            let default_sigma = map_luminance_to_sigma(0.5);
            for idx in 0..SPECTRAL_AMPLITUDE_BANDS {
                let offset = SPECTRAL_SIGMA_OFFSET.saturating_add(idx);
                ums.data[offset] = default_sigma;
            }
        }
        return;
    }

    for target_idx in 0..SPECTRAL_AMPLITUDE_BANDS {
        let start = target_idx.saturating_mul(bins) / SPECTRAL_AMPLITUDE_BANDS;
        let end = (target_idx.saturating_add(1).saturating_mul(bins) / SPECTRAL_AMPLITUDE_BANDS)
            .min(bins);
        let end = end.max(start.saturating_add(1).min(bins));
        let slice = &spectral.bins[start..end];
        let sum = slice.iter().fold(0.0, |acc, &v| acc + v);
        ums.data[target_idx] = sum / slice.len() as Fx;
    }
    if let Some(sigmas) = spectral.sigma.as_ref() {
        for target_idx in 0..SPECTRAL_AMPLITUDE_BANDS {
            let start = target_idx.saturating_mul(sigmas.len()) / SPECTRAL_AMPLITUDE_BANDS;
            let end = (target_idx.saturating_add(1).saturating_mul(sigmas.len())
                / SPECTRAL_AMPLITUDE_BANDS)
                .min(sigmas.len());
            let end = end.max(start.saturating_add(1).min(sigmas.len()));
            let slice = &sigmas[start..end];
            let sum = slice.iter().fold(0.0, |acc, &v| acc + v);
            let offset = SPECTRAL_SIGMA_OFFSET.saturating_add(target_idx);
            ums.data[offset] = sum / slice.len() as Fx;
        }
    } else {
        let default_sigma = map_luminance_to_sigma(0.5);
        for target_idx in 0..SPECTRAL_AMPLITUDE_BANDS {
            let offset = SPECTRAL_SIGMA_OFFSET.saturating_add(target_idx);
            ums.data[offset] = default_sigma;
        }
    }
}

fn populate_chromatic(ums: &mut UnifiedModalitySpace, chromatic: &ChromaticTensor) {
    let mut histogram = vec![0.0; HUE_CATEGORIES];
    let mut total = 0.0;
    for row in 0..chromatic.shape.h {
        for col in 0..chromatic.shape.w {
            let rgb = chromatic.rgb_at(row, col);
            let (hue, _, _) = rgb_to_hsl(rgb[0], rgb[1], rgb[2]);
            let (idx_a, w_a, idx_b, w_b) = hue_to_bin_weights(hue);
            histogram[idx_a] += w_a;
            histogram[idx_b] += w_b;
            total += 1.0;
        }
    }
    if total > 0.0 {
        for value in histogram.iter_mut() {
            *value /= total;
        }
    }

    let chrom_offset = UMS_CHROMATIC_OFFSET;
    let chrom_end = chrom_offset
        .saturating_add(UMS_CHROMATIC_BANDS)
        .min(UMS_DIM);
    let slice = &mut ums.data[chrom_offset..chrom_end];
    for (idx, value) in histogram.iter().enumerate() {
        if idx < slice.len() {
            slice[idx] = *value;
        }
    }

    let (mean_h, mean_s, mean_l) = mean_hsl(chromatic);
    let mut cursor = HUE_CATEGORIES;
    if cursor < slice.len() {
        slice[cursor] = mean_h.cos();
        cursor = cursor.saturating_add(1);
    }
    if cursor < slice.len() {
        slice[cursor] = mean_h.sin();
        cursor = cursor.saturating_add(1);
    }
    if cursor < slice.len() {
        slice[cursor] = mean_s.clamp(0.0, 1.0);
        cursor = cursor.saturating_add(1);
    }
    if cursor < slice.len() {
        slice[cursor] = mean_l.clamp(0.0, 1.0);
    }
}

fn populate_temporal(ums: &mut UnifiedModalitySpace, spectral: &SpectralTensor) {
    if UMS_TEMPORAL_BANDS == 0 {
        return;
    }
    let energy = spectral_energy(spectral);
    let start = UMS_TEMPORAL_OFFSET.min(UMS_DIM.saturating_sub(1));
    ums.data[start] = energy;
}
