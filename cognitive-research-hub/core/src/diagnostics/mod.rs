//! Deterministic self-assessment and fault detection.
//!
//! This module organizes all diagnostic sub-systems, including
//! metrics, visualizers, and temporal continuity analysis,
//! as defined in `diagnostics/spec.md`.

pub mod continuity;
pub mod metrics;
pub mod visual;