# Module: trainer/config/tests/
# Spec Version: 1.0

## Purpose

This directory contains unit tests for the `config/` module.

The tests must verify:
* **Loading:** That `load_config` can correctly parse a valid `.toml` file.
* **Validation:** That `load_config` (and `validation.rs`) correctly reject invalid configs (e.g., `batch_size: 0`).
* **Hashing:** That `load_config` produces a deterministic, correct `config_hash`.

## File Layout

* `test_loading.rs`: Contains all unit tests for the `config` module.