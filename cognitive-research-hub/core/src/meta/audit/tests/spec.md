# Module: core/src/meta/audit/tests/
# Spec Version: 1.0

## Purpose

This directory contains unit tests for the `audit/` module.

The tests must verify:
* **`hashcheck.rs`**: That the hashing function is deterministic.
* **`comparer.rs`**: That the `compare_runs` function correctly identifies identical and mismatched (mock) `Chronicle` files.
* **`mod.rs`**: That the `run_replay_audit` function can orchestrate a mock audit and return a correct `AuditReport`.

## File Layout

* `test_audit.rs`: Contains all unit tests for the `audit` module.