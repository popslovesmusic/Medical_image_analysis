//! Deterministic temporal reasoning and trend analysis.
//!
//! This module organizes the sub-modules for trend modeling,
//! oscillation detection, and stability classification, as
//! defined in `continuity/spec.md`.

pub mod classifier;
pub mod oscillation;
pub mod planner;
pub mod trend;

// (Placeholder re-exports)
// pub use self::classifier::{classify_stability, TrendModel};
// pub use self::oscillation::detect_oscillations;
// pub use self::planner::{plan_temporal_action, TemporalAction};
// pub use self::trend::analyze_trends;