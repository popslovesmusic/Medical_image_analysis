# Module: trainer/src/
# Spec Version: 1.0
# Purpose

This directory contains all the core Rust source code for the Trainer Subsystem.

It implements the logic for model architecture, data tokenization, the training loop, and validation, as defined by the parent `trainer/spec.md`. All code herein MUST be Rust 2021 and adhere to the project's deterministic ZAG constraints.

## Scope

* **Model Architectures (`model/`):** Defines the Rust-native neural network structures (e.g., using `Candle` or `burn`).
* **Data Handling (`tokenizer/`):** Implements pre-processing and tokenization for medical image data.
* **Training Loop (`training/`):** Manages the core training, optimization, and checkpointing logic.
* **Validation (`validator/`):** Implements the logic for evaluating model performance against a test set.
* **Reporting (`reports/`):** Generates deterministic JSON and Markdown summaries of training results.

## Architectural Overview
`src/`
├─ spec.md           ← this specification
├─ model/
│   ├─ spec.md
│   └─ ... (Rust source files)
├─ tokenizer/
│   ├─ spec.md
│   └─ ... (Rust source files)
├─ training/
│   ├─ spec.md
│   └─ ... (Rust source files)
├─ validator/
│   ├─ spec.md
│   └─ ... (Rust source files)
├─ reports/
│   ├─ spec.md
│   └─ ... (Rust source files)
└─ lib.rs            ← Main crate entry point (`tiny-agent-trainer`)

## Deterministic Guarantees

* **ML Framework:** All models MUST be implemented using a deterministic-capable Rust framework (e.g., `Candle`, `burn`, or `tch-rs` with strict determinism flags).
* **RNG:** All random operations (e.g., weight init, data shuffling) MUST be seeded from the `base_seed` in `config/`.
* **Data Shuffling:** Data loaders MUST use a deterministic shuffling algorithm (e.g., seeded `rand_chacha::ChaCha8Rng`).
* **Checkpointing:** Serialization MUST be bit-for-bit reproducible.

## Status

* **Spec Version:** 1.0
* **Phase Alignment:** **Phase 6 & 7** (per `IMPLEMENTATION_CHECKLIST.md`)
* **Dependencies:** `chromatic-core`, `candle-core`, `serde`, `rand_chacha`
* **Readiness:** ✅ Approved for implementation