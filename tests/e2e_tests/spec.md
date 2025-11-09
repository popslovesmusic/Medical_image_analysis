# Module: tests/e2e_tests/
# Spec Version: 1.0

## Purpose

This directory contains the actual Rust source code for the project-wide end-to-end (E2E) tests, as defined in the parent `tests/spec.md`.

These tests are configured as a Rust integration test suite. `cargo test` will automatically discover and run them.

## Scope

* **`test_e2e_determinism.rs`**: Implements the test case for a deterministic re-run.
* **`test_e2e_checkpoint_replay.rs`**: Implements the test case for verifying checkpoint and replay.
* **`mod.rs`**: Registers the test modules (though for integration tests, this is often not strictly necessary, it's good practice).

## Status

* **Spec Version:** 1.0
* **Phase Alignment:** Continuous (Phases 1-9)
* **Readiness:** ✅ Approved