# Module: tests/fixtures/
# Spec Version: 1.0

## Purpose

This directory stores small, static data files used by the project-wide end-to-end (E2E) tests.

These "fixtures" provide known, stable inputs for testing the entire application binary. They are critical for ensuring that tests are reproducible and do not rely on external or changing data.

## Scope

* **Test Configs:** Contains minimal `.toml` files (e.g., `test_config.toml`) that run the trainer for a single epoch or step.
* **Test Data:** Contains a small, representative sample of input data (e.g., `test_image.png`).
* **Golden Files:** May contain "golden" output files (e.g., `golden_metrics.json`) for tests that compare a new run's output against a known, correct baseline.

## Status

* **Spec Version:** 1.0
* **Phase Alignment:** Continuous (Phases 1-9)
* **Readiness:** ✅ Approved