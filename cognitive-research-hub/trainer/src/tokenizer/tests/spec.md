# Module: trainer/src/tokenizer/tests/
# Spec Version: 1.0

## Purpose

This directory contains unit tests for the `tokenizer/` module.

The tests must verify:
* **Tokenization:** That the `encode` / `decode` round-trip is lossless (or within acceptable tolerance).
* **Dataloader:** That the `DataLoader` produces identical batches in the same order when given the same seed.
* **Augmentation:** That all augmentation functions are deterministic when given the same seed.

## File Layout

* `test_tokenizer.rs`: Contains all unit tests for the `tokenizer` module.