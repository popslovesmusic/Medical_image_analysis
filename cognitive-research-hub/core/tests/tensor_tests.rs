//! Integration tests for deterministic tensor operations.
//!
//! Specification references:
//! - `cognitive-research-hub/spec.md`
//! - `cognitive-research-hub/core/spec.md`
//! - `cognitive-research-hub/core/src/tensor/spec.md`

use chromatic_core::tensor::{
    add_rgb, delta_hsl, hsl_to_rgb, map_rgb_inplace, mask_inject, mean_rgb, mix_rgb, normalize_hue,
    rgb_to_hsl, sum_fixed_rgb, ChromaticTensor, Shape2D,
};

fn sample_tensor(value: f32) -> ChromaticTensor {
    let shape = Shape2D::new(3, 3);
    let rgb = vec![value; shape.rgb_len()];
    ChromaticTensor::new(shape, rgb, None)
}

#[test]
fn hue_normalization_wraps() {
    let two_pi = std::f32::consts::PI * 2.0;
    assert!((normalize_hue(two_pi + 0.25) - 0.25).abs() < 1e-6);
    assert!((normalize_hue(-0.5) - (two_pi - 0.5)).abs() < 1e-6);
}

#[test]
fn rgb_hsl_roundtrip_is_consistent() {
    let (h, s, l) = rgb_to_hsl(0.25, 0.5, 0.75);
    let (r, g, b) = hsl_to_rgb(h, s, l);
    let dr = (r - 0.25).abs();
    let dg = (g - 0.5).abs();
    let db = (b - 0.75).abs();
    assert!(dr < 1e-4, "red channel diff {}", dr);
    assert!(dg < 1e-4, "green channel diff {}", dg);
    assert!(db < 1e-4, "blue channel diff {}", db);
}

#[test]
fn delta_hsl_is_seam_aware() {
    let a = (0.0, 0.5, 0.5);
    let b = (std::f32::consts::PI * 1.75, 0.55, 0.45);
    let (dh, ds, dl) = delta_hsl(a, b);
    assert!(dh.abs() < 1.0);
    assert!((ds - 0.05).abs() < 1e-6);
    assert!((dl + 0.05).abs() < 1e-6);
}

#[test]
fn mix_and_add_are_deterministic() {
    let shape = Shape2D::new(2, 2);
    let mut out = ChromaticTensor::new(shape, vec![0.0; shape.rgb_len()], None);
    let a = ChromaticTensor::new(shape, vec![1.0; shape.rgb_len()], None);
    let b = ChromaticTensor::new(shape, vec![0.0; shape.rgb_len()], None);
    mix_rgb(&mut out, &a, &b, 0.25);
    assert!(out.rgb.iter().all(|&v| (v - 0.25).abs() < 1e-6));

    add_rgb(&mut out, &a, &b);
    assert!(out.rgb.iter().all(|&v| (v - 1.0).abs() < 1e-6));
}

#[test]
fn mask_inject_blends_correctly() {
    let shape = Shape2D::new(1, 2);
    let base = ChromaticTensor::new(shape, vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0], None);
    let inj = ChromaticTensor::new(shape, vec![0.0, 0.0, 1.0, 0.5, 0.5, 0.5], None);
    let mut out = ChromaticTensor::new(shape, vec![0.0; shape.rgb_len()], None);
    let mask = vec![0.0, 1.0];
    mask_inject(&mut out, &base, &inj, &mask);
    assert!((out.rgb[0] - 1.0).abs() < 1e-6);
    assert!((out.rgb[5] - 0.5).abs() < 1e-6);
}

#[test]
fn map_and_mean_behave() {
    let mut tensor = sample_tensor(0.5);
    map_rgb_inplace(&mut tensor, |v| v * v);
    assert!(tensor.rgb.iter().all(|&v| (v - 0.25).abs() < 1e-6));
    let mean = mean_rgb(&tensor);
    assert!(mean.iter().all(|&v| (v - 0.25).abs() < 1e-6));
}

#[test]
fn fixed_point_sum_is_stable() {
    let tensor = sample_tensor(0.125);
    let result = sum_fixed_rgb(&tensor, 1_000);
    let expected = (0.125_f32 * 1000.0_f32).round() as i32 * tensor.shape.cell_count() as i32;
    for channel in result {
        assert_eq!(channel, expected);
    }
}
