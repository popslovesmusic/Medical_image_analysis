# Module: trainer/src/validator/tests/
# Spec Version: 1.0

## Purpose

This directory contains unit tests for the `validator/` module.

The tests must verify:
* **Metric Determinism:** That all functions in `metrics.rs` (e.g., `calculate_accuracy`) return the exact same floating-point value for the same inputs every time.
* **Report Aggregation:** That the `run_validation` loop correctly aggregates metrics using fixed-order summation.

## File Layout

* `test_metrics.rs`: Contains all unit tests for the `validator` module.