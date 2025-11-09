# Module: experiments/archive/restoration/
# Spec Version: 1.0

## Purpose

This directory defines the **Phase 9 Determinism Audit** process.

Its purpose is to provide the specification and (eventually) the Rust-based scripts to perform a full, bit-for-bit reproducibility audit on an archived experiment.

This module is the final arbiter of the "Zero Ambiguity Guarantee" for determinism.

## Scope

* Defines the `audit.rs` (or similar) utility.
* This utility will be a standalone Rust binary (e.g., `cargo run --package auditor -- <archive_path>`).
* The auditor will perform the following steps, as defined in `IMPLEMENTATION_CHECKLIST.md`:
    1.  **Un-archive:** Decompress a target experiment (e.g., `exp_001.tar.gz`) from the `archive/` directory.
    2.  **Extract Config:** Read the `config.toml` from the un-archived experiment.
    3.  **Re-run:** Launch the `tiny-agent-trainer` binary using that *exact* config, outputting to a new temporary `results/` directory.
    4.  **Compare:** Use the `core/meta/audit` module's functions (e.g., `compare_runs`) to perform a bitwise "diff" between the *original* archived `chronicle.cmeta` and the *newly generated* one.
    5.  **Report:** Output a final `determinism_audit.log` stating PASS or FAIL.

## File Layout