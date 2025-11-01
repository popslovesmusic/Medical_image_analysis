//! Tensor module implementing deterministic chromatic and spectral operations.
//!
//! Specification references:
//! - `cognitive-research-hub/spec.md`
//! - `cognitive-research-hub/core/spec.md`
//! - `cognitive-research-hub/core/src/tensor/spec.md`

mod chromatic;
mod layout;
mod ops;
mod spectral;

pub use chromatic::{delta_hsl, hsl_to_rgb, normalize_hue, rgb_to_hsl, ChromaticTensor};
pub use layout::{Shape2D, Stride2D};
<<<<<<< ours
pub use ops::{add_rgb, map_rgb_inplace, mask_inject, mean_rgb, mix_rgb, sum_fixed_rgb};
pub use spectral::SpectralTensor;
=======
pub use ops::{
    add_rgb, grad_hsl_loss, grad_mix, map_rgb_inplace, mask_inject, mean_rgb, mix_rgb,
    sum_fixed_rgb, GradRGB,
};
pub use spectral::{
    add_gaussian_kernel, bin_freq, spectral_centroid, spectral_energy, SpectralTensor,
};
>>>>>>> theirs

/// Shared helper for computing deterministic RGB index offsets.
#[inline]
fn channel_offset(stride: Stride2D, row: usize, col: usize, channel: usize) -> usize {
    let row_offset = row.saturating_mul(stride.row);
    let col_offset = col.saturating_mul(stride.col);
    row_offset
        .saturating_add(col_offset)
        .saturating_add(channel)
}
