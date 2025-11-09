# Module: experiments/results/
# Spec Version: 1.1
# Purpose

This directory is the **output target** for all experiment artifacts.

When an experiment is run (e.g., from `experiments/configs/exp_001_baseline.toml`), the `trainer` will create a new, unique sub-directory here (e.g., `exp_001_baseline_20251109T120000/`) and save all outputs into it.

The `comparisons/` sub-directory is used to store explicit A/B test summaries.

## Scope

* Contains timestamped output directories for each experimental run.
* Contains a `comparisons/` directory for diff reports.
* This directory (excluding `spec.md` and `comparisons/spec.md`) should be added to `.gitignore`.

## Directory Structure (Example)