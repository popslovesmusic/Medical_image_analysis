# Module: core/src/meta/context/tests/
# Spec Version: 1.0

## Purpose

This directory contains unit tests for the `context/` module.

The tests must verify that the `create_frame` function correctly:
* Creates a new `ContextFrame`.
* Accurately inherits the `run_id`, `config_hash`, and `environment` from the parent.
* Correctly sets the new `module` name.

## File Layout

* `test_context.rs`: Contains all unit tests for the `context` module.