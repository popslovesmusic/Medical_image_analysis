//! Spectral tensor representation for deterministic frequency analysis.
//!
//! Specification references:
//! - `cognitive-research-hub/spec.md`
//! - `cognitive-research-hub/core/spec.md`
//! - `cognitive-research-hub/core/src/tensor/spec.md`

use crate::Fx;

<<<<<<< ours
<<<<<<< ours
<<<<<<< ours
<<<<<<< ours
=======
=======
>>>>>>> theirs
=======
>>>>>>> theirs
=======
>>>>>>> theirs
/// Computes the center frequency of the `k`-th bin.
///
/// For linear layouts the frequency advances in constant steps `f_res`.
/// For logarithmic layouts the resolution is treated as a multiplicative
/// ratio applied per-bin. The implementation guards against integer
/// overflow when converting the index for the logarithmic exponent.
pub fn bin_freq(k: usize, f_min: Fx, f_res: Fx, log_scale: bool) -> Fx {
    assert!(f_min > 0.0, "minimum frequency must be positive");
    if log_scale {
        assert!(f_res > 0.0, "log-scale resolution must be positive");
        assert!(k <= i32::MAX as usize, "bin index exceeds supported range");
        let ratio = f_res;
        f_min * ratio.powi(k as i32)
    } else {
        f_min + f_res * k as Fx
    }
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
<<<<<<< ours
<<<<<<< ours
<<<<<<< ours
<<<<<<< ours
        self.bins.iter().fold(0.0, |acc, &v| acc + v.abs())
    }
=======
=======
>>>>>>> theirs
=======
>>>>>>> theirs
=======
>>>>>>> theirs
        spectral_energy(self)
    }
}

/// Adds a Gaussian kernel centred at `f0` with width `sigma` and amplitude `amp`.
pub fn add_gaussian_kernel(spec: &mut SpectralTensor, f0: Fx, sigma: Fx, amp: Fx) {
    assert!(sigma > 0.0, "sigma must be positive");
    let denom = 2.0 * sigma * sigma;
    for (k, bin) in spec.bins.iter_mut().enumerate() {
        let freq = bin_freq(k, spec.f_min, spec.f_res, spec.log_scale);
        let delta = freq - f0;
        let weight = (-(delta * delta) / denom).exp();
        *bin += amp * weight;
    }
}

/// Deterministic spectral energy computed as the L1 norm of the bins.
pub fn spectral_energy(spec: &SpectralTensor) -> Fx {
    spec.bins.iter().fold(0.0, |acc, &v| acc + v.abs())
}

/// Computes the spectral centroid weighted by the absolute amplitude.
pub fn spectral_centroid(spec: &SpectralTensor) -> Fx {
    let total = spectral_energy(spec);
    if total <= f32::EPSILON {
        return bin_freq(0, spec.f_min, spec.f_res, spec.log_scale);
    }
    let mut num = 0.0;
    for (k, &amp) in spec.bins.iter().enumerate() {
        let freq = bin_freq(k, spec.f_min, spec.f_res, spec.log_scale);
        num += freq * amp.abs();
    }
    num / total
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
}
