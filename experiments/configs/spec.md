# Module: experiments/configs/
# Spec Version: 1.0

## Purpose

This directory stores the canonical `.toml` configuration files for all named experiments.

Each file in this directory represents a single, reproducible experimental run. These files are the **only** thing that should differ between runs to ensure deterministic comparison.

## Scope

* Contains `.toml` files that adhere to the schema defined in `trainer/config/spec.md`.
* Files should be named descriptively (e.g., `exp_001_baseline.toml`, `exp_002_increased_lr.toml`).
* These configs are read by the `trainer` binary at launch (e.g., `cargo run -- --config experiments/configs/exp_001_baseline.toml`).

## File Layout