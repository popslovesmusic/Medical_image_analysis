//! Chromatic Core library implementing deterministic tensor primitives.
//!
//! Specification references:
//! - `cognitive-research-hub/spec.md`
//! - `cognitive-research-hub/core/spec.md`
//! - `cognitive-research-hub/core/src/tensor/spec.md`

pub mod tensor;

/// Core tensor dimensions enforced across the system (3×12×12×3).
pub const CSA_SHAPE: (usize, usize, usize, usize) = (3, 12, 12, 3);

/// Number of chromatic hue categories supported by the archive.
pub const HUE_CATEGORIES: usize = 12;

/// Number of spectral channels per chromatic field sample.
pub const SPECTRAL_CHANNELS: usize = 3;

/// Convenience alias for floating point operations within the core.
pub type Fx = f32;

/// Fixed-point accumulator type for deterministic reductions.
pub type Qx = i32;
