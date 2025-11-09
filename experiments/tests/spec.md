# Module: tests/ (Project Root)
# Spec Version: 1.0

## Purpose

This directory defines the project-wide testing strategy and provides the top-level integration and end-to-end (E2E) test harness for the entire `Medical_Image_Analysis` monorepo.

Its purpose is to verify that the individually-tested crates (`core`, `trainer`) integrate correctly and that the final application fulfills all deterministic guarantees (ZAGs) from `AGENTS.md`.

## Testing Philosophy & Tooling

* **Toolchain:** All tests MUST be written in Rust 2021. The one and only test runner for this project is **`cargo test`**. All references to `pytest` or other non-Rust test runners in any spec are superseded by this one.
* **Unit Tests:** Reside *within* each crate (e.g., `core/src/tensor/tests/`). They test a single module in isolation.
* **Integration Tests:** Reside *within* each crate's `tests/` directory (e.g., `core/tests/`). They test the crate's public API.
* **End-to-End (E2E) Tests (This Directory):** Reside here. These tests build and run the *entire compiled binary* (e.g., `tiny-agent-trainer`) as a black box, verifying its behavior against a known dataset and configuration.

## Scope

* **E2E Tests:** Contains the top-level `cargo test` integration suite that runs the compiled binaries.
* **Test Data (`fixtures/`):** Stores small, static data files (e.g., sample images, mock configs, "golden" output files) used by the tests.
* **Test Logs (`logs/`):** Stores the output logs generated *by* the test runs themselves (e.g., `test_e2e_run.log`). This directory should be in `.gitignore`.

## Key E2E Test Cases

* **`test_e2e_deterministic_run`**:
    1.  Runs the `trainer` binary with `experiments/configs/exp_001_baseline.toml`.
    2.  Runs the `trainer` binary *again* with the exact same config.
    3.  Uses the `core/meta/audit` module to perform a hash comparison of the two `chronicle.cmeta` files.
    4.  **Pass Condition:** The hashes MUST be bit-for-bit identical.
* **`test_e2e_checkpoint_replay`**:
    1.  Runs the `trainer` for 2 epochs and stops.
    2.  Runs the `trainer` *again*, loading from the epoch 2 checkpoint, for 2 more epochs (total 4).
    3.  Runs a *separate* `trainer` from scratch for 4 epochs continuously.
    4.  **Pass Condition:** The final model weights and `metrics.json` from both runs MUST be identical.

## File Layout 🛠️