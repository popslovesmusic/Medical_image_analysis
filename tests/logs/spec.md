# Module: tests/logs/
# Spec Version: 1.0

## Purpose

This directory is the **output target** for log files generated *by the test harness itself* (e.g., `cargo test > tests/logs/test_run.log`).

This is separate from the `experiments/results/` directory, which is an *artifact* of the `trainer` binary. This directory is an artifact of the `cargo test` command.

## Scope

* Contains `.log` files from test runs, primarily for debugging CI/CD failures.
* This entire directory (except this `spec.md` file) MUST be in the root `.gitignore` file. Log files should never be committed to source control.

## File Layout