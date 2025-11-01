use std::f32::consts::PI;

use crate::{
    tensor::{rgb_to_hsl, spectral_energy, ChromaticTensor, SpectralTensor},
    Fx, HUE_CATEGORIES, UMS_CHROMATIC_BANDS, UMS_CHROMATIC_OFFSET, UMS_DIM, UMS_SPECTRAL_BANDS,
    UMS_TEMPORAL_BANDS, UMS_TEMPORAL_OFFSET,
};

<<<<<<< ours
<<<<<<< ours
<<<<<<< ours
use super::{hue_to_bin_weights, map_luminance_to_sigma, mean_hsl, normalize_hue};
=======
use super::{hue_to_bin_weights, map_luminance_to_sigma, mean_hsl, normalize_hue, EPSILON};
>>>>>>> theirs
=======
use super::{hue_to_bin_weights, map_luminance_to_sigma, mean_hsl, normalize_hue, EPSILON};
>>>>>>> theirs
=======
use super::{hue_to_bin_weights, map_luminance_to_sigma, mean_hsl, normalize_hue, EPSILON};
>>>>>>> theirs

const SPECTRAL_AMPLITUDE_BANDS: usize = UMS_SPECTRAL_BANDS / 2;
const SPECTRAL_SIGMA_OFFSET: usize = SPECTRAL_AMPLITUDE_BANDS;

/// Unified Modality Space (UMS) vector storing spectral, chromatic, and temporal features.
#[derive(Clone, Debug, PartialEq)]
pub struct UnifiedModalitySpace {
    data: [Fx; UMS_DIM],
}

<<<<<<< ours
<<<<<<< ours
<<<<<<< ours
=======
=======
>>>>>>> theirs
=======
>>>>>>> theirs
/// Half-precision (f16) encoded representation of a Unified Modality Space vector.
#[derive(Clone, Debug, PartialEq)]
pub struct CompressedUnifiedModality {
    data: [u16; UMS_DIM],
    mean: Fx,
    std: Fx,
}

const F16_MAX: Fx = 65_504.0;
const F16_MIN: Fx = -65_504.0;

<<<<<<< ours
<<<<<<< ours
>>>>>>> theirs
=======
>>>>>>> theirs
=======
>>>>>>> theirs
impl UnifiedModalitySpace {
    /// Creates a zero-initialised UMS vector.
    pub fn new() -> Self {
        Self {
            data: [0.0; UMS_DIM],
        }
    }

<<<<<<< ours
<<<<<<< ours
<<<<<<< ours
=======
=======
>>>>>>> theirs
=======
>>>>>>> theirs
    /// Creates a UMS vector from an explicit array.
    pub fn from_array(data: [Fx; UMS_DIM]) -> Self {
        Self { data }
    }

<<<<<<< ours
<<<<<<< ours
>>>>>>> theirs
=======
>>>>>>> theirs
=======
>>>>>>> theirs
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
<<<<<<< ours
<<<<<<< ours
<<<<<<< ours
=======
=======
>>>>>>> theirs
=======
>>>>>>> theirs

    /// Returns the fixed dimensionality of the UMS vector.
    pub fn len(&self) -> usize {
        UMS_DIM
    }
<<<<<<< ours
<<<<<<< ours
>>>>>>> theirs
=======
>>>>>>> theirs
=======
>>>>>>> theirs
}

impl Default for UnifiedModalitySpace {
    fn default() -> Self {
        Self::new()
    }
}

<<<<<<< ours
<<<<<<< ours
<<<<<<< ours
=======
=======
>>>>>>> theirs
=======
>>>>>>> theirs
impl CompressedUnifiedModality {
    /// Returns the stored global mean used for μ/σ normalisation.
    pub fn mean(&self) -> Fx {
        self.mean
    }

    /// Returns the stored global standard deviation used for μ/σ normalisation.
    pub fn std(&self) -> Fx {
        self.std
    }

    /// Returns the underlying half-precision payload slice.
    pub fn data(&self) -> &[u16] {
        &self.data
    }

    /// Reconstructs a Unified Modality Space vector from the compressed payload.
    pub fn decompress(&self) -> UnifiedModalitySpace {
        let mut data = [0.0; UMS_DIM];
        for (idx, bits) in self.data.iter().copied().enumerate() {
            data[idx] = f16_bits_to_f32(bits) * self.std + self.mean;
        }
        UnifiedModalitySpace::from_array(data)
    }
}

<<<<<<< ours
<<<<<<< ours
>>>>>>> theirs
=======
>>>>>>> theirs
=======
>>>>>>> theirs
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

<<<<<<< ours
<<<<<<< ours
<<<<<<< ours
=======
=======
>>>>>>> theirs
=======
>>>>>>> theirs
/// Compresses a Unified Modality Space vector using μ/σ normalisation and f16 payloads.
pub fn compress_ums(ums: &UnifiedModalitySpace) -> CompressedUnifiedModality {
    let (mean, std) = compute_moments(ums.as_slice());
    let adjusted_std = std.max(EPSILON);
    let normaliser = 1.0 / adjusted_std;
    let mut data = [0u16; UMS_DIM];
    for (idx, value) in ums.as_slice().iter().copied().enumerate() {
        let mut normalised = (value - mean) * normaliser;
        if normalised.is_nan() || normalised.is_infinite() {
            normalised = 0.0;
        }
        let clamped = normalised.clamp(F16_MIN, F16_MAX);
        data[idx] = f32_to_f16_bits(clamped);
    }
    CompressedUnifiedModality {
        data,
        mean,
        std: adjusted_std,
    }
}

/// Decompresses a Unified Modality Space vector from its μ/σ normalised f16 form.
pub fn decompress_ums(compressed: &CompressedUnifiedModality) -> UnifiedModalitySpace {
    compressed.decompress()
}

<<<<<<< ours
<<<<<<< ours
>>>>>>> theirs
=======
>>>>>>> theirs
=======
>>>>>>> theirs
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
<<<<<<< ours
<<<<<<< ours
<<<<<<< ours
=======
=======
>>>>>>> theirs
=======
>>>>>>> theirs

fn compute_moments(values: &[Fx]) -> (Fx, Fx) {
    if values.is_empty() {
        return (0.0, 0.0);
    }
    let mut sum = 0.0;
    for &value in values {
        sum += value;
    }
    let mean = sum / values.len() as Fx;
    let mut variance_acc = 0.0;
    for &value in values {
        let delta = value - mean;
        variance_acc += delta * delta;
    }
    let variance = variance_acc / values.len() as Fx;
    (mean, variance.sqrt())
}

fn f32_to_f16_bits(value: Fx) -> u16 {
    let bits = value.to_bits();
    let sign = ((bits >> 16) & 0x8000) as u16;
    let exponent = ((bits >> 23) & 0xff) as i32;
    let mantissa = bits & 0x7fffff;

    if exponent == 0xff {
        return if mantissa == 0 {
            sign | 0x7c00
        } else {
            sign | 0x7e00
        };
    }

    let mut exp16 = exponent - 127 + 15;
    if exp16 >= 0x1f {
        return sign | 0x7c00;
    }
    if exp16 <= 0 {
        if exp16 < -10 {
            return sign;
        }
        let mantissa = mantissa | 0x800000;
        let shift = 14 - exp16;
        let mut half = (mantissa >> shift) as u16;
        if ((mantissa >> (shift - 1)) & 0x1) != 0 {
            half = half.saturating_add(1);
        }
        return sign | half;
    }

    let mut half_mant = (mantissa >> 13) as u16;
    if ((mantissa >> 12) & 0x1) != 0 {
        half_mant = half_mant.saturating_add(1);
        if half_mant & 0x0400 != 0 {
            half_mant = 0;
            exp16 += 1;
            if exp16 >= 0x1f {
                return sign | 0x7c00;
            }
        }
    }
    sign | ((exp16 as u16) << 10) | (half_mant & 0x03ff)
}

fn f16_bits_to_f32(bits: u16) -> Fx {
    let sign = (bits >> 15) as u32;
    let exponent = ((bits >> 10) & 0x1f) as i32;
    let mantissa = (bits & 0x03ff) as u32;
    let sign_bits = sign << 31;

    if exponent == 0 {
        if mantissa == 0 {
            return Fx::from_bits(sign_bits);
        }
        let mut mant = mantissa;
        let mut exp = -14;
        while (mant & 0x0400) == 0 {
            mant <<= 1;
            exp -= 1;
        }
        mant &= 0x03ff;
        let exponent_bits = ((exp + 127) as u32) << 23;
        let mantissa_bits = mant << 13;
        return Fx::from_bits(sign_bits | exponent_bits | mantissa_bits);
    }

    if exponent == 0x1f {
        let mantissa_bits = mantissa << 13;
        return Fx::from_bits(sign_bits | 0x7f80_0000 | mantissa_bits);
    }

    let exponent_bits = ((exponent + 112) as u32) << 23;
    let mantissa_bits = mantissa << 13;
    Fx::from_bits(sign_bits | exponent_bits | mantissa_bits)
}
<<<<<<< ours
<<<<<<< ours
>>>>>>> theirs
=======
>>>>>>> theirs
=======
>>>>>>> theirs
