# Module: trainer/src/training/tests/
# Spec Version: 1.0

## Purpose

This directory contains unit tests for the `training/` module.

The tests must verify:
* **Checkpointing:** That a `save_checkpoint` / `load_checkpoint` round-trip results in a bitwise identical `CheckpointState`.
* **Optimizer:** That the optimizer state is correctly saved and loaded.
* **Loss Function:** That loss functions are deterministic (return the same value for the same input).
* **Training Loop:** (Mock test) That the `run_training_loop` can successfully orchestrate a single, deterministic step.

## File Layout

* `test_checkpoint.rs`: Contains all unit tests for the `training` module.