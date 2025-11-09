# Module: core/src/meta/logger/tests/
# Spec Version: 1.0

## Purpose

This directory contains the unit tests for the `logger/` module.

The tests must verify the functionality specified in the parent `logger/spec.md`, ensuring that the logging macros and backends (console, file) function correctly.

## Scope

* **Macro Tests (`test_macros.rs`):**
    * Verify that `log_info!`, `log_warn!`, and `log_error!` macros compile.
    * Test that the macros correctly format messages.
* **Backend Tests (`test_backends.rs`):**
    * Test initialization of the console and file backends.
    * Verify that backends (using mock interfaces) receive the correct log levels and messages.
    * Ensure file backend handles I/O errors gracefully.

## Deterministic Guarantees

* Tests must not rely on specific system time.
* Tests must mock file I/O to avoid writing to disk during `cargo test`.
* Tests must not interfere with the global logger state.