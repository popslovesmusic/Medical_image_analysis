//! Integration tests for the chromatic↔spectral bridge module.
//!
//! Specification references:
//! - `cognitive-research-hub/spec.md`
//! - `cognitive-research-hub/core/spec.md`
//! - `cognitive-research-hub/core/src/bridge/spec.md`

use chromatic_core::{
    bridge::{decode_to_chromatic, encode_to_spectral, record_seam_weights, validate_round_trip},
    tensor::{delta_hsl, hsl_to_rgb, rgb_to_hsl, ChromaticTensor, Shape2D},
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
