# Module: cognitive-research-hub/scripts/
# Spec Version: 1.1 (ZAG Compliant)
# Purpose

This folder houses utility binaries and orchestration scripts that operate *on* the compiled Rust workspace.
Scripts here handle automation, experiment runs, and environment validation.

They must:

1.  Be written in Rust (as a utility binary) or as simple shell scripts (`.sh`, `.bat`) that invoke `cargo`.
2.  Adhere to all ZAG constraints, especially the OpenCL mandate.

## Directory Layout
cognitive-research-hub/scripts/
├─ spec.md               ← this spec
├─ env_check/
│   ├─ src/main.rs       ← Rust binary to verify OpenCL, CPU features (AVX), etc.
│   └─ Cargo.toml
├─ run_experiment.sh     ← Orchestration script to launch a trainer run
├─ archive_results.sh    ← Orchestration script to archive results
└─ README.md

## Script Categories and Roles
Category	Purpose	Interfaces With
`env_check`	A **Rust binary** that verifies the build and runtime environment. It MUST check for:
* `rustc` version
* `cargo` version
* **OpenCL** (via `ocl` crate) driver and device presence
* Required CPU features (e.to.AVX)
`orchestration`	Shell scripts (`.sh`/`.bat`) that provide a simple interface for launching compiled Rust binaries (e.g., the `trainer`).

## Deterministic Rules

* **No Python:** This directory MUST NOT contain any Python (`.py`) scripts.
* **Environment Lock:** The `env_check` binary exports a manifest of the compiler, OS, and **OpenCL** version.
* **Orchestration:** All experiment execution is handled by running the compiled `tiny-agent-trainer` binary, not by a script.

## Example: Deterministic Run Invocation

Orchestration is performed by invoking the compiled trainer binary directly.

```bash
# The `run_experiment.sh` script is a simple wrapper for this:
cargo run --release --package tiny-agent-trainer -- \
    --config experiments/configs/exp_001_baseline.toml