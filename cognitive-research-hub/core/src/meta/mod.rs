//! Placeholder for the main `meta` module.
//!
//! This module organizes all sub-modules related to
//! determinism, auditing, logging, and session context,
//! as defined in `meta/spec.md`.

pub mod audit;
pub mod chronicle;
pub mod context;
pub mod logger;
pub mod session;

// (Placeholder re-exports for the `core` crate)
// pub use self::chronicle::CycleRecord;
// pub use self::session::ContextFrame;
// pub use self::audit::AuditReport;