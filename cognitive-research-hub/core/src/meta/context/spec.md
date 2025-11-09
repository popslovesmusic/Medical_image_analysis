# Module: core/src/meta/context/
# Spec Version: 1.0

## Purpose

The Context module provides functions for describing the *causal* environment of a specific operation. While the `session/` module defines the global, static context (like `run_id` and `config_hash`), this module defines the *dynamic* context: *what* is running *right now*.

It allows modules like `dream/` or `trainer/` to create a `ContextFrame` that describes their current operation (e.g., "Phase: Dream, Module: generator, Action: Perturbation"). This frame is then passed to the `chronicle/` to be saved, providing a rich, auditable log of system behavior.

## Scope

* Defines helper functions to create `ContextFrame` objects.
* Provides utilities to update or derive a new `ContextFrame` from a parent (e.g., "Phase: Dream" -> "Phase: Dream, Module: generator").
* Manages the string constants for phases and module names to ensure consistency.

## Core Data Structures

This module primarily operates on the `ContextFrame` struct, which is defined in `session/context.rs`.

```rust
// Defined in session/context.rs, but used heavily by this module.
pub struct ContextFrame {
    pub run_id: String,
    pub module: String,         // The dynamic module name (e.g., "dream/generator")
    pub config_hash: String,
    pub environment: String,
}