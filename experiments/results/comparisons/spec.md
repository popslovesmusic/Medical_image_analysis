# Module: experiments/results/comparisons/
# Spec Version: 1.0

## Purpose

This directory stores human-readable comparison reports (A/B tests) between two or more experimental runs from the parent `results/` directory.

While the `audit/` module performs a bitwise *determinism check* on a *single* experiment, this module is for *performance comparison* between *different* experiments.

## Scope

* Contains Markdown (`.md`) files.
* Each file should compare key metrics (e.g., final loss, accuracy, F1-score) between two or more runs (e.g., "Baseline vs. 8-Layer CNN").
* These reports are intended to be generated manually or by a simple Rust utility.

## Status

* **Spec Version:** 1.0
* **Phase Alignment:** **Phase 8** (per `IMPLEMENTATION_CHECKLIST.md`)
* **Readiness:** ✅ Approved