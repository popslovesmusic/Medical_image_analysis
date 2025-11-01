//! Diagnostics module providing deterministic metric computation and continuity checks.
//!
//! Specification references:
//! - `cognitive-research-hub/spec.md`
//! - `cognitive-research-hub/core/spec.md`
//! - `cognitive-research-hub/core/src/diagnostics/spec.md`

pub mod metrics;

use metrics::continuity_from_history;

/// Validates that the provided history of cycles maintains deterministic continuity bounds.
pub fn validate_continuity(history: &[CycleRecord]) -> bool {
    if history.len() < 3 {
        return true;
    }
    let metrics = continuity_from_history(history);
    let slope_ok = metrics.slope.abs() <= 0.05;
    let stdev_ok = metrics.stdev <= 0.1;
    let oscillation_ok = metrics.oscillation_index <= 0.35;
    slope_ok && stdev_ok && oscillation_ok
}

/// Convenience re-export for consumers requiring the continuity metrics structure.
pub use metrics::ContinuityMetrics;
/// Convenience re-export for consumers requiring the cycle record representation.
pub use metrics::CycleRecord;
