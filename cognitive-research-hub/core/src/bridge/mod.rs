//! Chromatic–spectral bridge for deterministic hue/frequency conversions.
//!
//! Specification references:
//! - `cognitive-research-hub/spec.md`
//! - `cognitive-research-hub/core/spec.md`
//! - `cognitive-research-hub/core/src/bridge/spec.md`
//!
//! The bridge links the chromatic tensor representation with the spectral
//! tensor used during diagnostics and sonic encoding. The implementation keeps
//! the mapping deterministic and reversible within the Δ ≤ 1e-3 tolerance
//! envelope described in the specification.

use std::f32::consts::PI;

use crate::{tensor::*, Fx, HUE_CATEGORIES};

/// Base frequency (Hz) used for hue→frequency mapping.
const BASE_FREQUENCY: Fx = 27.5; // A0 reference
/// Octave span encoded by the bridge.
const OCTAVE_SPAN: Fx = 5.0;
/// Minimal Gaussian width in Hz for luminance→sigma mapping.
const MIN_SIGMA: Fx = 4.0;
/// Maximal Gaussian width in Hz for luminance→sigma mapping.
const MAX_SIGMA: Fx = 48.0;
/// Round-trip tolerance for Δ components.
const ROUND_TRIP_TOLERANCE: Fx = 1e-3;
/// Epsilon used to guard against divisions by zero.
const EPSILON: Fx = 1e-6;

fn ratio_per_bin() -> Fx {
    let steps = (HUE_CATEGORIES as Fx - 1.0).max(1.0);
    2f32.powf(OCTAVE_SPAN / steps)
}

fn map_luminance_to_sigma(l: Fx) -> Fx {
    let l_clamped = l.clamp(0.0, 1.0);
    MIN_SIGMA + (1.0 - l_clamped) * (MAX_SIGMA - MIN_SIGMA)
}

fn sigma_to_luminance(sigma: Fx) -> Fx {
    let sigma_clamped = sigma.clamp(MIN_SIGMA, MAX_SIGMA);
    1.0 - (sigma_clamped - MIN_SIGMA) / (MAX_SIGMA - MIN_SIGMA)
}

fn hue_to_bin_weights(hue: Fx) -> (usize, Fx, usize, Fx) {
    let hue_norm = normalize_hue(hue);
    let span = 2.0 * PI;
    let scaled = (hue_norm / span) * HUE_CATEGORIES as Fx;
    let base_idx = scaled.floor() as usize;
    let frac = scaled - base_idx as Fx;
    let idx_a = base_idx.min(HUE_CATEGORIES - 1);
    let idx_b = (idx_a + 1) % HUE_CATEGORIES;
    let weight_a = 1.0 - frac;
    let weight_b = frac;
    (idx_a, weight_a, idx_b, weight_b)
}

fn dominant_bin(spec: &SpectralTensor) -> usize {
    let mut max_idx = 0;
    let mut max_val = f32::MIN;
    for (idx, &amp) in spec.bins.iter().enumerate() {
        if amp > max_val {
            max_val = amp;
            max_idx = idx;
        }
    }
    max_idx
}

fn hue_from_frequency(freq: Fx) -> Fx {
    let ratio = (freq / BASE_FREQUENCY).max(EPSILON);
    let hue = 2.0 * PI * (ratio.log2() / OCTAVE_SPAN);
    normalize_hue(hue)
}

fn mean_hsl(chromatic: &ChromaticTensor) -> (Fx, Fx, Fx) {
    let mut sum_cos = 0.0;
    let mut sum_sin = 0.0;
    let mut sum_s = 0.0;
    let mut sum_l = 0.0;
    let mut count = 0.0;
    for row in 0..chromatic.shape.h {
        for col in 0..chromatic.shape.w {
            let rgb = chromatic.rgb_at(row, col);
            let (h, s, l) = rgb_to_hsl(rgb[0], rgb[1], rgb[2]);
            sum_cos += h.cos();
            sum_sin += h.sin();
            sum_s += s;
            sum_l += l;
            count += 1.0;
        }
    }
    if count <= 0.0 {
        return (0.0, 0.0, 0.5);
    }
    let avg_h = sum_sin.atan2(sum_cos);
    let hue = normalize_hue(avg_h);
    (hue, sum_s / count, sum_l / count)
}

/// Encodes a chromatic tensor into its spectral representation.
pub fn encode_to_spectral(chromatic: &ChromaticTensor) -> SpectralTensor {
    let mut bins = vec![0.0; HUE_CATEGORIES];
    let mut sigma = vec![0.0; HUE_CATEGORIES];
    let mut counts = vec![0.0; HUE_CATEGORIES];
    let cell_count = chromatic.shape.cell_count().max(1) as Fx;

    for row in 0..chromatic.shape.h {
        for col in 0..chromatic.shape.w {
            let rgb = chromatic.rgb_at(row, col);
            let (h, s, l) = rgb_to_hsl(rgb[0], rgb[1], rgb[2]);
            let (idx_a, w_a, idx_b, w_b) = hue_to_bin_weights(h);
            let sigma_value = map_luminance_to_sigma(l);

            bins[idx_a] += s * w_a;
            sigma[idx_a] += sigma_value * w_a;
            counts[idx_a] += w_a;

            bins[idx_b] += s * w_b;
            sigma[idx_b] += sigma_value * w_b;
            counts[idx_b] += w_b;
        }
    }

    for (i, count) in counts.iter().enumerate() {
        if *count > 0.0 {
            bins[i] /= cell_count;
            sigma[i] /= *count;
        } else {
            sigma[i] = map_luminance_to_sigma(0.5);
            bins[i] = 0.0;
        }
    }

    SpectralTensor::new(bins, Some(sigma), BASE_FREQUENCY, ratio_per_bin(), true)
}

/// Decodes a spectral tensor back into a chromatic tensor (1×1 pixel).
pub fn decode_to_chromatic(spectral: &SpectralTensor) -> ChromaticTensor {
    let total_energy: Fx = spectral.bins.iter().map(|v| v.max(0.0)).sum();
    let weighted_index = spectral
        .bins
        .iter()
        .enumerate()
        .fold(0.0, |acc, (idx, amp)| acc + idx as Fx * amp.max(0.0));
    let mean_index = if total_energy > EPSILON {
        weighted_index / total_energy
    } else {
        0.0
    };
    let hue_fraction = (mean_index / HUE_CATEGORIES as Fx).clamp(0.0, 1.0);
    let freq = BASE_FREQUENCY * 2f32.powf(hue_fraction * OCTAVE_SPAN);
    let hue = hue_from_frequency(freq);

    let saturation = total_energy.clamp(0.0, 1.0);

    let sigma_value = spectral
        .sigma
        .as_ref()
        .map_or(map_luminance_to_sigma(0.5), |sig| {
            if total_energy > EPSILON {
                let weighted = sig
                    .iter()
                    .zip(spectral.bins.iter())
                    .fold(0.0, |acc, (s, amp)| acc + *s * amp.max(0.0));
                (weighted / total_energy).clamp(MIN_SIGMA, MAX_SIGMA)
            } else {
                map_luminance_to_sigma(0.5)
            }
        });
    let luminance = sigma_to_luminance(sigma_value);

    let (r, g, b) = hsl_to_rgb(hue, saturation, luminance);
    let shape = Shape2D::new(1, 1);
    let rgb = vec![r, g, b];
    let coherence = if total_energy > EPSILON {
        let idx = dominant_bin(spectral);
        let amp = spectral.bins[idx].max(0.0);
        Some(vec![(amp / total_energy).clamp(0.0, 1.0)])
    } else {
        Some(vec![0.0])
    };
    ChromaticTensor::new(shape, rgb, coherence)
}

/// Computes seam-aware weights for a hue near the wrap-around boundary.
pub fn record_seam_weights(hue: Fx, epsilon: Fx) -> (Fx, Fx) {
    assert!(epsilon > 0.0, "epsilon must be positive");
    let hue_norm = normalize_hue(hue);
    let span = 2.0 * PI;

    if hue_norm < epsilon {
        let ratio = (epsilon - hue_norm) / epsilon;
        (ratio.clamp(0.0, 1.0), 1.0 - ratio.clamp(0.0, 1.0))
    } else if span - hue_norm < epsilon {
        let ratio = (epsilon - (span - hue_norm)) / epsilon;
        ((1.0 - ratio).clamp(0.0, 1.0), ratio.clamp(0.0, 1.0))
    } else {
        let (_, w_a, _, w_b) = hue_to_bin_weights(hue_norm);
        (w_a.clamp(0.0, 1.0), w_b.clamp(0.0, 1.0))
    }
}

/// Validates that encode→decode round-trips stay within tolerance.
pub fn validate_round_trip(chromatic: &ChromaticTensor) -> bool {
    let mean = mean_hsl(chromatic);
    let spectral = encode_to_spectral(chromatic);
    let decoded = decode_to_chromatic(&spectral);
    let rgb = decoded.rgb_at(0, 0);
    let reconstructed = rgb_to_hsl(rgb[0], rgb[1], rgb[2]);
    let (dh, ds, dl) = delta_hsl(mean, reconstructed);
    dh.abs() <= ROUND_TRIP_TOLERANCE
        && ds.abs() <= ROUND_TRIP_TOLERANCE
        && dl.abs() <= ROUND_TRIP_TOLERANCE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn luminance_sigma_roundtrip() {
        for &l in &[0.0, 0.25, 0.5, 0.75, 1.0] {
            let sigma = map_luminance_to_sigma(l);
            let recovered = sigma_to_luminance(sigma);
            assert!((recovered - l).abs() < 1e-6);
        }
    }

    #[test]
    fn hue_mapping_consistent() {
        let freq = BASE_FREQUENCY * 2f32.powf(OCTAVE_SPAN * 0.5);
        let hue = hue_from_frequency(freq);
        let (_, w_a, _, w_b) = hue_to_bin_weights(hue);
        assert!((w_a + w_b - 1.0).abs() < 1e-6);
    }
}
