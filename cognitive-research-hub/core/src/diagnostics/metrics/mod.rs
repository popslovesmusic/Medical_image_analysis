//! Deterministic diagnostics metrics for chromatic and spectral tensors.
//!
//! This module acts as the entry point, organizing metric-related
//! logic into specialized sub-modules as defined by the specification.
//!
//! Specification references:
//! - `cognitive-research-hub/spec.md`
//! - `cognitive-research-hub/core/spec.md`
//! - `cognitive-research-hub/core/src/diagnostics/spec.md`
//! - `cognitive-research-hub/core/src/diagnostics/metrics/spec.md`

// Declare the sub-modules required by the spec's File Layout
pub mod continuity;
pub mod hsl;
pub mod spectral;

// Re-export key data structures and functions for easier access
// (Placeholders for when implementation is added)
// pub use self::continuity::{ContinuityMetrics, continuity_from_history};
// pub use self::hsl::{ChromaticDelta, compute_delta_hsl};
// pub use self::spectral::{SpectralStats, spectral_energy_balance, phase_coherence_index};