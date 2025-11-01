//! Chromatic Core library implementing deterministic tensor primitives.
//!
//! Specification references:
//! - `cognitive-research-hub/spec.md`
//! - `cognitive-research-hub/core/spec.md`
//! - `cognitive-research-hub/core/src/tensor/spec.md`

<<<<<<< ours
<<<<<<< ours
<<<<<<< ours
<<<<<<< ours
=======
pub mod bridge;
>>>>>>> theirs
=======
pub mod bridge;
>>>>>>> theirs
=======
pub mod bridge;
>>>>>>> theirs
=======
pub mod bridge;
pub mod diagnostics;
>>>>>>> theirs
pub mod tensor;

/// Core tensor dimensions enforced across the system (3×12×12×3).
pub const CSA_SHAPE: (usize, usize, usize, usize) = (3, 12, 12, 3);

/// Number of chromatic hue categories supported by the archive.
pub const HUE_CATEGORIES: usize = 12;

/// Number of spectral channels per chromatic field sample.
pub const SPECTRAL_CHANNELS: usize = 3;

<<<<<<< ours
<<<<<<< ours
<<<<<<< ours
=======
=======
>>>>>>> theirs
=======
>>>>>>> theirs
/// Unified Modality Space total dimensionality (spectral + chromatic + temporal).
pub const UMS_DIM: usize = 512;

/// Number of slots reserved for spectral information inside the Unified Modality Space.
pub const UMS_SPECTRAL_BANDS: usize = 256;

/// Starting index for chromatic slots within the Unified Modality Space vector.
pub const UMS_CHROMATIC_OFFSET: usize = UMS_SPECTRAL_BANDS;

/// Number of chromatic slots allocated in the Unified Modality Space.
pub const UMS_CHROMATIC_BANDS: usize = 128;

/// Starting index for temporal slots within the Unified Modality Space vector.
pub const UMS_TEMPORAL_OFFSET: usize = UMS_CHROMATIC_OFFSET + UMS_CHROMATIC_BANDS;

/// Number of temporal slots currently reserved within the Unified Modality Space.
pub const UMS_TEMPORAL_BANDS: usize = UMS_DIM - UMS_TEMPORAL_OFFSET;

<<<<<<< ours
<<<<<<< ours
>>>>>>> theirs
=======
>>>>>>> theirs
=======
>>>>>>> theirs
/// Convenience alias for floating point operations within the core.
pub type Fx = f32;

/// Fixed-point accumulator type for deterministic reductions.
pub type Qx = i32;
