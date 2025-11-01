//! Layout utilities for deterministic chromatic tensors.
//!
//! Specification references:
//! - `cognitive-research-hub/spec.md`
//! - `cognitive-research-hub/core/spec.md`
//! - `cognitive-research-hub/core/src/tensor/spec.md`

use crate::Fx;

/// Two-dimensional tensor shape describing height and width.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Shape2D {
    pub h: usize,
    pub w: usize,
}

impl Shape2D {
    /// Creates a new shape ensuring both dimensions are non-zero.
    pub fn new(h: usize, w: usize) -> Self {
        assert!(h > 0 && w > 0, "shape dimensions must be non-zero");
        Self { h, w }
    }

    /// Returns the number of cells in the tensor using saturating arithmetic.
    pub fn cell_count(&self) -> usize {
        self.h.saturating_mul(self.w)
    }

    /// Returns the total number of RGB elements given the interleaved layout.
    pub fn rgb_len(&self) -> usize {
        self.cell_count().saturating_mul(3)
    }
}

/// Two-dimensional stride descriptor enforcing row-major iteration order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Stride2D {
    pub row: usize,
    pub col: usize,
}

impl Stride2D {
    /// Constructs a stride descriptor using saturating arithmetic for safety.
    pub fn new(shape: Shape2D) -> Self {
        let col = 3; // RGB interleaved
        let row = shape.w.saturating_mul(col);
        Self { row, col }
    }
}

/// Deterministically clamps a scalar into the \[0, 1\] range.
pub fn clamp_unit(x: Fx) -> Fx {
    x.max(0.0).min(1.0)
}
