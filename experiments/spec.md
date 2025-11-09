# Module: experiments/
# Spec Version: 1.1
# Purpose

This directory provides the framework for structured experiment orchestration and archival.

Its purpose is to define a clear, reproducible process for running the `trainer` with different configurations (e.g., A/B testing new model layers or hyperparameters) and storing the results in a way that is auditable and compliant with our determinism guarantees.

## Scope

* **Configuration (`configs/`):** Stores all `.toml` configuration files used for specific, named experiments.
* **Execution:** Defines the **Rust-based** method for launching experiments (e.g., `cargo run --release --package tiny-agent-trainer -- --config experiments/configs/exp_001.toml`).
* **Results Storage (`results/`):** Defines the standard output directory and format for all experiment artifacts (logs, models, and reports).
* **Archival (`archive/`):** Defines the process for compressing and storing the results of completed experiments.

## Core Data Structures

This module does not define new data structures, but it orchestrates the flow of:
* `config/TrainerConfig` (Input)
* `reports/ValidationReport` (Output)
* `training/CheckpointState` (Output)
* `meta/chronicle/CycleRecord` (Output)

## Functional Overview

* **`run_experiment(config_path: &Path)`**:
    * This is not a function, but a process:
    1.  The user (or a script) launches the main `tiny-agent-trainer` binary, passing a path to a config file in `experiments/configs/`.
    2.  The trainer runs to completion (training and validation).
    3.  All outputs (`run_summary.md`, `metrics.json`, `.cmeta` log) are saved to a unique, timestamped directory in `experiments/results/`.
    4.  The `config.toml` file used for the run is copied into the results directory for perfect reproducibility.
* **`archive_experiment(result_path: &Path)`**:
    * A utility (or manual process) to compress a completed experiment directory (e.g., into `.tar.gz`) and move it to `experiments/archive/`.

## Deterministic Guarantees

* **Configuration-Driven:** The *only* difference between two experiments must be the `.toml` configuration file.
* **Full Reproducibility:** Any experiment in `results/` or `archive/` MUST be perfectly reproducible by re-running the trainer with the *exact* same config file and a clean environment.
* **Verification:** The `archive/` process should include a verification script (e.g., `cargo run --package verifier`) that re-runs the experiment and performs an audit (using `meta/audit`) to ensure the results are identical.

## File Layout 🛠️