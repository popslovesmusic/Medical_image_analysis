//! Fixed-point quantization utilities and deterministic accumulators.
//!
//! Specification references:
//! - `cognitive-research-hub/spec.md`
//! - `cognitive-research-hub/core/spec.md`
//! - `cognitive-research-hub/core/src/tensor/spec.md`

use crate::{Fx, Qx};

/// Default fixed-point scale (Q12.20) used across deterministic reductions.
pub const DEFAULT_FIXED_SCALE: i32 = 1 << 20;

/// Quantizes an `f32` value into fixed-point representation using `scale`.
pub fn quantize_scalar(value: Fx, scale: i32) -> Qx {
    assert!(scale > 0, "scale must be positive");
    let scaled = (value * scale as Fx).round() as i64;
    scaled.clamp(i32::MIN as i64, i32::MAX as i64) as Qx
}

/// Dequantizes a fixed-point value back into `f32` using `scale`.
pub fn dequantize_scalar(value: Qx, scale: i32) -> Fx {
    assert!(scale > 0, "scale must be positive");
    value as Fx / scale as Fx
}

/// Deterministic accumulator performing fixed-point reductions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FixedAccumulator {
    scale: i32,
    sum: i128,
}

impl FixedAccumulator {
    /// Creates a new accumulator using the provided scale.
    pub fn new(scale: i32) -> Self {
        assert!(scale > 0, "scale must be positive");
        Self { scale, sum: 0 }
    }

    /// Creates an accumulator using the default project-wide scale.
    pub fn with_default_scale() -> Self {
        Self::new(DEFAULT_FIXED_SCALE)
    }

    /// Returns the scale associated with this accumulator.
    pub fn scale(&self) -> i32 {
        self.scale
    }

    /// Returns the raw quantized sum.
    pub fn sum(&self) -> i128 {
        self.sum
    }

    /// Accumulates a floating-point value by quantizing it first.
    pub fn accumulate(&mut self, value: Fx) {
        let quantized = quantize_scalar(value, self.scale) as i128;
        self.sum = self.sum.saturating_add(quantized);
    }

    /// Accumulates a pre-quantized fixed-point value.
    pub fn accumulate_quantized(&mut self, value: Qx) {
        self.sum = self.sum.saturating_add(value as i128);
    }

    /// Accumulates a sequence of floating-point values deterministically.
    pub fn accumulate_iter<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Fx>,
    {
        for value in iter {
            self.accumulate(value);
        }
    }

    /// Merges another accumulator into `self`, requiring identical scales.
    pub fn merge(&mut self, other: &Self) {
        assert_eq!(self.scale, other.scale, "scale mismatch");
        self.sum = self.sum.saturating_add(other.sum);
    }

    /// Returns the accumulated value as `f32`.
    pub fn finish(self) -> Fx {
        (self.sum as Fx) / self.scale as Fx
    }

    /// Returns the accumulated value as a quantized `Qx`.
    pub fn finish_quantized(self) -> Qx {
        self.sum.clamp(i32::MIN as i128, i32::MAX as i128) as Qx
    }

    /// Performs a deterministic sum using the default scale.
    pub fn sum_slice(values: &[Fx]) -> Fx {
        let mut acc = Self::with_default_scale();
        acc.accumulate_iter(values.iter().copied());
        acc.finish()
    }
}
