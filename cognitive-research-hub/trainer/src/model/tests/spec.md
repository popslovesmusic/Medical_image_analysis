# Module: trainer/src/model/tests/
# Spec Version: 1.0

## Purpose

This directory contains unit tests for the `model/` module.

The tests must verify:
* **Determinism:** That initializing the same model with the same seed produces bitwise identical weights.
* **Forward Pass:** That a forward pass with a fixed input produces a deterministic output.
* **Serialization:** That a `save_weights` / `load_weights` round-trip results in an identical model.

## File Layout

* `test_model_determinism.rs`: Contains all unit tests for the `model` module.