//! Deterministic chromatic tensor operations and reductions.
//!
//! Specification references:
//! - `cognitive-research-hub/spec.md`
//! - `cognitive-research-hub/core/spec.md`
//! - `cognitive-research-hub/core/src/tensor/spec.md`

<<<<<<< ours
<<<<<<< ours
<<<<<<< ours
<<<<<<< ours
<<<<<<< ours
use super::{channel_offset, layout::clamp_unit, ChromaticTensor};
use crate::{Fx, Qx};

=======
use super::chromatic::delta_hsl;
use super::{channel_offset, chromatic::rgb_to_hsl, layout::clamp_unit, ChromaticTensor};
=======
=======
>>>>>>> theirs
=======
>>>>>>> theirs
=======
>>>>>>> theirs
use super::chromatic::delta_hsl;
use super::{
    channel_offset,
    chromatic::rgb_to_hsl,
    layout::clamp_unit,
    quant::{quantize_scalar, FixedAccumulator},
    ChromaticTensor,
};
<<<<<<< ours
<<<<<<< ours
<<<<<<< ours
>>>>>>> theirs
=======
>>>>>>> theirs
=======
>>>>>>> theirs
=======
>>>>>>> theirs
use crate::{Fx, Qx};

/// Gradient components for RGB triples.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GradRGB {
    pub dr: Fx,
    pub dg: Fx,
    pub db: Fx,
}

<<<<<<< ours
<<<<<<< ours
<<<<<<< ours
<<<<<<< ours
>>>>>>> theirs
=======
>>>>>>> theirs
=======
>>>>>>> theirs
=======
>>>>>>> theirs
=======
>>>>>>> theirs
/// Performs a deterministic linear blend between two chromatic tensors.
pub fn mix_rgb(out: &mut ChromaticTensor, a: &ChromaticTensor, b: &ChromaticTensor, alpha: Fx) {
    assert_eq!(out.shape, a.shape);
    assert_eq!(out.shape, b.shape);
    let clamped_alpha = clamp_unit(alpha);
    let inv_alpha = 1.0 - clamped_alpha;
    for row in 0..out.shape.h {
        for col in 0..out.shape.w {
            let offset = channel_offset(out.stride, row, col, 0);
            for channel in 0..3 {
                let idx = offset + channel;
                out.rgb[idx] = clamped_alpha * a.rgb[idx] + inv_alpha * b.rgb[idx];
            }
        }
    }
}

/// Adds two tensors with clamping to the unit interval.
pub fn add_rgb(out: &mut ChromaticTensor, a: &ChromaticTensor, b: &ChromaticTensor) {
    assert_eq!(out.shape, a.shape);
    assert_eq!(out.shape, b.shape);
    for row in 0..out.shape.h {
        for col in 0..out.shape.w {
            let offset = channel_offset(out.stride, row, col, 0);
            for channel in 0..3 {
                let idx = offset + channel;
                out.rgb[idx] = clamp_unit(a.rgb[idx] + b.rgb[idx]);
            }
        }
    }
}

/// Injects values from `inj` into `base` according to `mask`, writing the result to `out`.
pub fn mask_inject(
    out: &mut ChromaticTensor,
    base: &ChromaticTensor,
    inj: &ChromaticTensor,
    mask: &[Fx],
) {
    assert_eq!(out.shape, base.shape);
    assert_eq!(out.shape, inj.shape);
    assert_eq!(mask.len(), out.shape.cell_count());
    for row in 0..out.shape.h {
        for col in 0..out.shape.w {
            let mask_idx = row.saturating_mul(out.shape.w).saturating_add(col);
            let m = clamp_unit(mask[mask_idx]);
            let offset = channel_offset(out.stride, row, col, 0);
            for channel in 0..3 {
                let idx = offset + channel;
                out.rgb[idx] = base.rgb[idx] * (1.0 - m) + inj.rgb[idx] * m;
            }
        }
    }
}

/// Applies an in-place mapping to every RGB channel of the tensor.
pub fn map_rgb_inplace(t: &mut ChromaticTensor, mut f: impl FnMut(Fx) -> Fx) {
    for value in &mut t.rgb {
        *value = clamp_unit(f(*value));
    }
}

/// Computes the deterministic mean RGB triplet.
pub fn mean_rgb(t: &ChromaticTensor) -> [Fx; 3] {
    let mut sums = [0.0; 3];
    for row in 0..t.shape.h {
        for col in 0..t.shape.w {
            let offset = channel_offset(t.stride, row, col, 0);
            for channel in 0..3 {
                sums[channel] += t.rgb[offset + channel];
            }
        }
    }
    let denom = t.shape.cell_count() as Fx;
    [sums[0] / denom, sums[1] / denom, sums[2] / denom]
}

/// Performs a fixed-point deterministic reduction over the RGB channels.
pub fn sum_fixed_rgb(t: &ChromaticTensor, scale: i32) -> [Qx; 3] {
    assert!(scale > 0, "scale must be positive");
<<<<<<< ours
<<<<<<< ours
<<<<<<< ours
<<<<<<< ours
    let mut accum = [0i64; 3];
=======
=======
>>>>>>> theirs
=======
>>>>>>> theirs
=======
>>>>>>> theirs
    let mut accum = [
        FixedAccumulator::new(scale),
        FixedAccumulator::new(scale),
        FixedAccumulator::new(scale),
    ];
<<<<<<< ours
<<<<<<< ours
<<<<<<< ours
>>>>>>> theirs
=======
>>>>>>> theirs
=======
>>>>>>> theirs
=======
>>>>>>> theirs
    for row in 0..t.shape.h {
        for col in 0..t.shape.w {
            let offset = channel_offset(t.stride, row, col, 0);
            for channel in 0..3 {
<<<<<<< ours
<<<<<<< ours
<<<<<<< ours
<<<<<<< ours
                let value = (t.rgb[offset + channel] * scale as Fx).round() as i64;
                accum[channel] += value;
            }
        }
    }
    accum.map(|v| v.clamp(i32::MIN as i64, i32::MAX as i64) as Qx)
}
<<<<<<< ours
=======
=======
=======
>>>>>>> theirs
=======
>>>>>>> theirs
=======
>>>>>>> theirs
                let value = t.rgb[offset + channel];
                let quantized = quantize_scalar(value, scale);
                accum[channel].accumulate_quantized(quantized);
            }
        }
    }
    accum.map(FixedAccumulator::finish_quantized)
}
<<<<<<< ours
<<<<<<< ours
<<<<<<< ours
>>>>>>> theirs
=======
>>>>>>> theirs
=======
>>>>>>> theirs
=======
>>>>>>> theirs

/// Gradients of the mix operation w.r.t. inputs and mixing coefficient.
pub fn grad_mix(
    a: (Fx, Fx, Fx),
    b: (Fx, Fx, Fx),
    alpha: Fx,
    d_out: (Fx, Fx, Fx),
) -> (GradRGB, GradRGB, Fx) {
    let alpha_clamped = clamp_unit(alpha);
    let inv_alpha = 1.0 - alpha_clamped;
    let grad_a = GradRGB {
        dr: alpha_clamped * d_out.0,
        dg: alpha_clamped * d_out.1,
        db: alpha_clamped * d_out.2,
    };
    let grad_b = GradRGB {
        dr: inv_alpha * d_out.0,
        dg: inv_alpha * d_out.1,
        db: inv_alpha * d_out.2,
    };
    let d_alpha = d_out.0 * (a.0 - b.0) + d_out.1 * (a.1 - b.1) + d_out.2 * (a.2 - b.2);
    (grad_a, grad_b, d_alpha)
}

fn hsl_loss(rgb: (Fx, Fx, Fx), target: (Fx, Fx, Fx)) -> Fx {
    let (h, s, l) = rgb_to_hsl(rgb.0, rgb.1, rgb.2);
    let (dh, ds, dl) = delta_hsl((h, s, l), target);
    0.5 * (dh * dh + ds * ds + dl * dl)
}

/// Numerical gradient of the HSL loss with respect to the RGB inputs.
pub fn grad_hsl_loss(a_rgb: (Fx, Fx, Fx), b_hsl: (Fx, Fx, Fx)) -> GradRGB {
    const EPS: Fx = 1e-3;
    let base = [a_rgb.0, a_rgb.1, a_rgb.2];
    let mut grad = [0.0; 3];
    for i in 0..3 {
        let mut plus = base;
        plus[i] = clamp_unit(plus[i] + EPS);
        let mut minus = base;
        minus[i] = clamp_unit(minus[i] - EPS);
        let loss_plus = hsl_loss((plus[0], plus[1], plus[2]), b_hsl);
        let loss_minus = hsl_loss((minus[0], minus[1], minus[2]), b_hsl);
        grad[i] = (loss_plus - loss_minus) / (2.0 * EPS);
    }
    GradRGB {
        dr: grad[0],
        dg: grad[1],
        db: grad[2],
    }
}
<<<<<<< ours
<<<<<<< ours
<<<<<<< ours
<<<<<<< ours
>>>>>>> theirs
=======
>>>>>>> theirs
=======
>>>>>>> theirs
=======
>>>>>>> theirs
=======
>>>>>>> theirs
