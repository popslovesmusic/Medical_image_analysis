//! Spectral tensor representation for deterministic frequency analysis.
//!
//! Specification references:
//! - `cognitive-research-hub/spec.md`
//! - `cognitive-research-hub/core/spec.md`
//! - `cognitive-research-hub/core/src/tensor/spec.md`

use crate::Fx;

/// Spectral tensor capturing frequency-domain amplitudes and optional bandwidths.
#[derive(Clone, Debug)]
pub struct SpectralTensor {
    pub bins: Vec<Fx>,
    pub sigma: Option<Vec<Fx>>,
    pub f_min: Fx,
    pub f_res: Fx,
    pub log_scale: bool,
}

impl SpectralTensor {
    /// Constructs a spectral tensor validating buffer lengths.
    pub fn new(
        bins: Vec<Fx>,
        sigma: Option<Vec<Fx>>,
        f_min: Fx,
        f_res: Fx,
        log_scale: bool,
    ) -> Self {
        assert!(
            !bins.is_empty(),
            "spectral tensor requires at least one bin"
        );
        if let Some(ref sigma_buf) = sigma {
            assert_eq!(sigma_buf.len(), bins.len(), "sigma buffer length mismatch");
        }
        Self {
            bins,
            sigma,
            f_min,
            f_res,
            log_scale,
        }
    }

    /// Returns the energy of the spectrum via deterministic accumulation.
    pub fn energy(&self) -> Fx {
        self.bins.iter().fold(0.0, |acc, &v| acc + v.abs())
    }
}
