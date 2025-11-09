# Module: trainer/docs/
# Spec Version: 1.0

## Purpose

This directory provides internal documentation and experiment notes specifically for the `trainer/` subsystem.

Its goal is to provide a human-readable guide for developers working on the `trainer/` crate, including setup instructions, architectural overviews (specific to the trainer), and records of experimental results.

## Scope

* **Architectural Notes:** Contains Markdown files explaining the design decisions behind the `trainer/`'s Rust-native ML implementation (e.g., "Why Candle vs. Burn").
* **Experiment Records:** A log of training runs, observations, and results.
* **How-To Guides:** Instructions for common tasks, such as "How to add a new model layer" or "How to run a replay audit."

## Relationship to other Docs

* **`docs/` (Root):** Contains the *global* project documentation.
* **`trainer/docs/` (This):** Contains *local* documentation only relevant to `trainer/` developers.
* **`spec.md` files:** Define the *requirements* (the "what").
* **`trainer/docs/` files:** Explain the *implementation* (the "how" and "why").

## File Layout