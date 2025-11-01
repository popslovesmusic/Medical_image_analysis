//! Deterministic chromatic tensor operations and reductions.
//!
//! Specification references:
//! - `cognitive-research-hub/spec.md`
//! - `cognitive-research-hub/core/spec.md`
//! - `cognitive-research-hub/core/src/tensor/spec.md`

use super::{channel_offset, layout::clamp_unit, ChromaticTensor};
use crate::{Fx, Qx};

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
    let mut accum = [0i64; 3];
    for row in 0..t.shape.h {
        for col in 0..t.shape.w {
            let offset = channel_offset(t.stride, row, col, 0);
            for channel in 0..3 {
                let value = (t.rgb[offset + channel] * scale as Fx).round() as i64;
                accum[channel] += value;
            }
        }
    }
    accum.map(|v| v.clamp(i32::MIN as i64, i32::MAX as i64) as Qx)
}
