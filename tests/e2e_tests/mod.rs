//! Project-wide End-to-End (E2E) test suite.
//!
//! This module registers all E2E tests, which verify
//! the deterministic behavior of the compiled application binary.

pub mod test_e2e_checkpoint_replay;
pub mod test_e2e_determinism;