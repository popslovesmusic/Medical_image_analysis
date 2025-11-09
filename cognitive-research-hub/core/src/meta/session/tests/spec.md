# Module: core/src/meta/session/tests/
# Spec Version: 1.0

## Purpose

This directory contains unit tests for the `session/` module.

The tests must verify that the `init_session` function correctly:
* Generates a unique `run_id`.
* Deterministically hashes configuration files (using mock files).
* Correctly identifies the runtime environment.

## File Layout

* `test_session.rs`: Contains all unit tests for the `session` module.