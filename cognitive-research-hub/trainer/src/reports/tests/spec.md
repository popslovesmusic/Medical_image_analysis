# Module: trainer/src/reports/tests/
# Spec Version: 1.0

## Purpose

This directory contains unit tests for the `reports/` module.

The tests must verify:
* **JSON Determinism:** That `generate_json_report` produces a bitwise-identical string for the same input (requires mock `serde_json` or fixed-order maps).
* **MD Determinism:** That `generate_md_report` produces an identical string for the same input (requires fixed float precision).
* **Hashing:** That the `hasher` module correctly computes a known SHA-256 hash for a given input string.

## File Layout

* `test_report_determinism.rs`: Contains all unit tests for the `reports` module.