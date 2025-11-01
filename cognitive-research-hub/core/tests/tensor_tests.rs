//! Integration tests for deterministic tensor operations.
//!
//! Specification references:
//! - `cognitive-research-hub/spec.md`
//! - `cognitive-research-hub/core/spec.md`
//! - `cognitive-research-hub/core/src/tensor/spec.md`

use chromatic_core::tensor::{
<<<<<<< ours
<<<<<<< ours
<<<<<<< ours
    add_rgb, delta_hsl, hsl_to_rgb, map_rgb_inplace, mask_inject, mean_rgb, mix_rgb, normalize_hue,
    rgb_to_hsl, sum_fixed_rgb, ChromaticTensor, Shape2D,
=======
    add_gaussian_kernel, add_rgb, bin_freq, delta_hsl, grad_hsl_loss, grad_mix, hsl_to_rgb,
    map_rgb_inplace, mask_inject, mean_rgb, mix_rgb, normalize_hue, rgb_to_hsl, spectral_centroid,
    spectral_energy, sum_fixed_rgb, ChromaticTensor, Shape2D, SpectralTensor,
>>>>>>> theirs
=======
=======
>>>>>>> theirs
    add_gaussian_kernel, add_rgb, bin_freq, delta_hsl, dequantize_scalar, grad_hsl_loss, grad_mix,
    hsl_to_rgb, map_rgb_inplace, mask_inject, mean_rgb, mix_rgb, normalize_hue, quantize_scalar,
    rgb_to_hsl, spectral_centroid, spectral_energy, sum_fixed_rgb, ChromaticTensor,
    FixedAccumulator, Shape2D, SpectralTensor, DEFAULT_FIXED_SCALE,
<<<<<<< ours
>>>>>>> theirs
=======
>>>>>>> theirs
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
<<<<<<< ours
<<<<<<< ours
<<<<<<< ours
=======
=======
>>>>>>> theirs
=======
>>>>>>> theirs

#[test]
fn spectral_utilities_behave() {
    assert!((bin_freq(0, 10.0, 5.0, false) - 10.0).abs() < 1e-6);
    assert!((bin_freq(2, 10.0, 5.0, false) - 20.0).abs() < 1e-6);
    assert!((bin_freq(2, 10.0, 2.0, true) - 40.0).abs() < 1e-4);

    let mut spec = SpectralTensor::new(vec![0.0; 5], None, 10.0, 5.0, false);
    add_gaussian_kernel(&mut spec, 20.0, 5.0, 1.0);
    assert!(spec.bins[2] > spec.bins[1] && spec.bins[2] > spec.bins[3]);
    let energy = spectral_energy(&spec);
    assert!(energy > 0.0);
    let centroid = spectral_centroid(&spec);
    assert!((centroid - 20.0).abs() < 2.0);
}

#[test]
fn grad_mix_matches_expectation() {
    let a = (0.8, 0.3, 0.2);
    let b = (0.2, 0.7, 0.4);
    let d_out = (1.0, 2.0, 3.0);
    let (grad_a, grad_b, d_alpha) = grad_mix(a, b, 0.25, d_out);
    assert!((grad_a.dr - 0.25).abs() < 1e-6);
    assert!((grad_a.dg - 0.5).abs() < 1e-6);
    assert!((grad_a.db - 0.75).abs() < 1e-6);
    assert!((grad_b.dr - 0.75).abs() < 1e-6);
    assert!((grad_b.dg - 1.5).abs() < 1e-6);
    assert!((grad_b.db - 2.25).abs() < 1e-6);
    let expected_alpha = d_out.0 * (a.0 - b.0) + d_out.1 * (a.1 - b.1) + d_out.2 * (a.2 - b.2);
    assert!((d_alpha - expected_alpha).abs() < 1e-6);
}

#[test]
fn grad_hsl_loss_vanishes_at_target() {
    let target_hsl = (std::f32::consts::PI / 3.0, 0.6, 0.4);
    let target_rgb = hsl_to_rgb(target_hsl.0, target_hsl.1, target_hsl.2);
    let grad = grad_hsl_loss(target_rgb, target_hsl);
    assert!(grad.dr.abs() < 1e-3);
    assert!(grad.dg.abs() < 1e-3);
    assert!(grad.db.abs() < 1e-3);
}
<<<<<<< ours
<<<<<<< ours
>>>>>>> theirs
=======
=======
>>>>>>> theirs

#[test]
fn quantization_roundtrip_within_tolerance() {
    let scale = DEFAULT_FIXED_SCALE;
    let value = 0.347_812_5;
    let quantized = quantize_scalar(value, scale);
    let restored = dequantize_scalar(quantized, scale);
    let tolerance = 1.0 / (scale as f32);
    assert!((restored - value).abs() <= tolerance);
}

#[test]
fn fixed_accumulator_is_order_invariant() {
    let scale = DEFAULT_FIXED_SCALE;
    let values = [0.1_f32, -0.25, 0.4, -0.05, 0.2];
    let mut acc_forward = FixedAccumulator::new(scale);
    for &v in &values {
        acc_forward.accumulate(v);
    }

    let mut acc_reverse = FixedAccumulator::new(scale);
    for &v in values.iter().rev() {
        acc_reverse.accumulate(v);
    }

    assert_eq!(
        acc_forward.finish_quantized(),
        acc_reverse.finish_quantized()
    );
    assert!((acc_forward.finish() - acc_reverse.finish()).abs() < 1e-6);

    let mut acc_merge = FixedAccumulator::new(scale);
    let (left, right) = values.split_at(values.len() / 2);
    let mut acc_left = FixedAccumulator::new(scale);
    acc_left.accumulate_iter(left.iter().copied());
    let mut acc_right = FixedAccumulator::new(scale);
    acc_right.accumulate_iter(right.iter().copied());
    acc_merge.merge(&acc_left);
    acc_merge.merge(&acc_right);

    assert_eq!(acc_forward.finish_quantized(), acc_merge.finish_quantized());
}
<<<<<<< ours
>>>>>>> theirs
=======
>>>>>>> theirs
