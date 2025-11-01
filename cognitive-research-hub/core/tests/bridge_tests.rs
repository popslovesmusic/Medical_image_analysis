//! Integration tests for the chromatic↔spectral bridge module.
//!
//! Specification references:
//! - `cognitive-research-hub/spec.md`
//! - `cognitive-research-hub/core/spec.md`
//! - `cognitive-research-hub/core/src/bridge/spec.md`

use chromatic_core::{
<<<<<<< ours
    bridge::{decode_to_chromatic, encode_to_spectral, record_seam_weights, validate_round_trip},
    tensor::{delta_hsl, hsl_to_rgb, rgb_to_hsl, ChromaticTensor, Shape2D},
=======
    bridge::{
        decode_to_chromatic, encode_to_spectral, project_to_ums, reconstruct_chromatic_from_ums,
        reconstruct_spectral_from_ums, record_seam_weights, validate_round_trip,
    },
    tensor::{delta_hsl, hsl_to_rgb, normalize_hue, rgb_to_hsl, ChromaticTensor, Shape2D},
    UMS_TEMPORAL_OFFSET,
>>>>>>> theirs
};

fn make_uniform_tensor(h: f32, s: f32, l: f32, shape: Shape2D) -> ChromaticTensor {
    let (r, g, b) = hsl_to_rgb(h, s, l);
    let mut rgb = Vec::with_capacity(shape.rgb_len());
    for _ in 0..shape.cell_count() {
        rgb.extend_from_slice(&[r, g, b]);
    }
    ChromaticTensor::new(shape, rgb, None)
}

#[test]
fn encode_decode_preserves_hue_within_tolerance() {
    let shape = Shape2D::new(3, 4);
    let original_hsl = (std::f32::consts::PI * 0.75, 0.6, 0.35);
    let chromatic = make_uniform_tensor(original_hsl.0, original_hsl.1, original_hsl.2, shape);

    let spectral = encode_to_spectral(&chromatic);
    let decoded = decode_to_chromatic(&spectral);
    let decoded_rgb = decoded.rgb_at(0, 0);
    let decoded_hsl = rgb_to_hsl(decoded_rgb[0], decoded_rgb[1], decoded_rgb[2]);
    let (dh, ds, dl) = delta_hsl(original_hsl, decoded_hsl);

    assert!(dh.abs() <= 1e-3, "ΔH = {}", dh);
    assert!(ds.abs() <= 1e-3, "ΔS = {}", ds);
    assert!(dl.abs() <= 1e-3, "ΔL = {}", dl);
}

#[test]
fn seam_weights_sum_to_one() {
    let epsilon = 0.05;
    let (w_low, w_high) = record_seam_weights(0.01, epsilon);
    let (w_low_wrap, w_high_wrap) = record_seam_weights(2.0 * std::f32::consts::PI - 0.01, epsilon);

    assert!((w_low + w_high - 1.0).abs() < 1e-6);
    assert!((w_low_wrap + w_high_wrap - 1.0).abs() < 1e-6);
}

#[test]
fn validate_round_trip_accepts_uniform_tensor() {
    let shape = Shape2D::new(2, 2);
    let chromatic = make_uniform_tensor(std::f32::consts::PI / 6.0, 0.4, 0.55, shape);
    assert!(validate_round_trip(&chromatic));
}

#[test]
fn encoding_distributes_energy_across_bins() {
    let shape = Shape2D::new(1, 12);
    let mut rgb = Vec::new();
    for idx in 0..shape.cell_count() {
        let hue = 2.0 * std::f32::consts::PI * (idx as f32 + 0.5) / 12.0;
        let (r, g, b) = hsl_to_rgb(hue, 0.7, 0.45);
        rgb.extend_from_slice(&[r, g, b]);
    }
    let chromatic = ChromaticTensor::new(shape, rgb, None);
    let spectral = encode_to_spectral(&chromatic);

    assert_eq!(spectral.bins.len(), 12);
    assert!(spectral.bins.iter().all(|&v| v >= 0.0));
    assert!(spectral
        .sigma
        .as_ref()
        .expect("sigma present")
        .iter()
        .all(|&sigma| sigma >= 4.0 && sigma <= 48.0));
}
<<<<<<< ours
=======

fn compute_mean_hsl(chromatic: &ChromaticTensor) -> (f32, f32, f32) {
    let mut sum_cos = 0.0f32;
    let mut sum_sin = 0.0f32;
    let mut sum_s = 0.0f32;
    let mut sum_l = 0.0f32;
    let mut count = 0.0f32;
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
    (normalize_hue(avg_h), sum_s / count, sum_l / count)
}

fn make_gradient_tensor(shape: Shape2D) -> ChromaticTensor {
    let mut rgb = Vec::with_capacity(shape.rgb_len());
    let total = shape.cell_count() as f32;
    for idx in 0..shape.cell_count() {
        let hue = 2.0 * std::f32::consts::PI * (idx as f32 / total);
        let sat = 0.35 + 0.25 * ((idx % shape.w) as f32 / shape.w as f32);
        let lum = 0.25 + 0.5 * ((idx / shape.w) as f32 / shape.h as f32);
        let (r, g, b) = hsl_to_rgb(hue, sat, lum);
        rgb.extend_from_slice(&[r, g, b]);
    }
    ChromaticTensor::new(shape, rgb, None)
}

#[test]
fn ums_projection_round_trip_restores_statistics() {
    let shape = Shape2D::new(3, 4);
    let chromatic = make_gradient_tensor(shape);
    let spectral = encode_to_spectral(&chromatic);
    let ums = project_to_ums(&chromatic, &spectral);

    let expected_mean = compute_mean_hsl(&chromatic);
    let reconstructed_mean = reconstruct_chromatic_from_ums(&ums);
    let (dh, ds, dl) = delta_hsl(expected_mean, reconstructed_mean);
    assert!(dh.abs() <= 1e-4, "ΔH = {}", dh);
    assert!(ds.abs() <= 1e-4, "ΔS = {}", ds);
    assert!(dl.abs() <= 1e-4, "ΔL = {}", dl);

    let histogram_sum: f32 = ums.chromatic_histogram().iter().sum();
    assert!((histogram_sum - 1.0).abs() <= 1e-4);

    let temporal_start = UMS_TEMPORAL_OFFSET;
    let energy_delta = (ums.as_slice()[temporal_start] - spectral.energy()).abs();
    assert!(energy_delta <= 1e-6, "ΔEnergy = {}", energy_delta);
}

#[test]
fn ums_projection_recovers_spectral_bins() {
    let shape = Shape2D::new(2, 6);
    let chromatic = make_gradient_tensor(shape);
    let spectral = encode_to_spectral(&chromatic);
    let ums = project_to_ums(&chromatic, &spectral);

    let (amps, sigmas) = reconstruct_spectral_from_ums(&ums, spectral.bins.len());
    for (expected, recovered) in spectral.bins.iter().zip(amps.iter()) {
        assert!((expected - recovered).abs() <= 1e-6);
    }
    if let Some(expected_sigmas) = spectral.sigma.as_ref() {
        for (expected, recovered) in expected_sigmas.iter().zip(sigmas.iter()) {
            assert!((expected - recovered).abs() <= 1e-6);
        }
    }
}
>>>>>>> theirs
