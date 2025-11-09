# Module: core/src/utils/
# Spec Version: 1.0

## Purpose

This module provides canonical, crate-wide helper functions and utilities, primarily to satisfy the ZAG (Zero Ambiguity Guarantee) constraints defined in `AGENTS.md`.

Its most critical responsibility is to provide the **standard implementation for saturating arithmetic**.

## Scope

* **Saturating Arithmetic:** Defines standard helper functions for memory-safe `usize` calculations.
* **Type Aliases:** May host other common type aliases if they are used across multiple sub-modules (e.g., `tensor`, `bridge`).

## 1. ZAG-B.1: Saturating Arithmetic

Per `AGENTS.md`, all `usize` calculations related to memory, array sizing, or indexing MUST use saturating arithmetic to prevent overflow-related panics or undefined behavior.

This module MUST provide (or re-export from `std::num::Saturating`) the following functions. All `core` crate code MUST use these helpers for memory-critical math instead of standard `+` or `-` operators.

### Required Functions

| Function | Signature | Description | ZAG Rationale |
| --- | --- | --- | --- |
| **`sat_add`** | `(usize, usize) -> usize` | Adds two `usize` values, saturating at `usize::MAX`. | Prevents overflow on index or size addition. |
| **`sat_sub`** | `(usize, usize) -> usize` | Subtracts two `usize` values, saturating at `0`. | Prevents underflow on index or size subtraction. |
| **`sat_mul`** | `(usize, usize) -> usize` | Multiplies two `usize` values, saturating at `usize::MAX`. | Prevents overflow when calculating total buffer sizes. |

### Implementation Note

The implementation should be a simple wrapper around the standard library's `saturating_add`, `saturating_sub`, and `saturating_mul` methods to ensure consistent, auditable use.

```rust
// Placeholder implementation snippet for clarity
// File: cognitive-research-hub/core/src/utils/mod.rs

/// Saturating addition for memory-safe usize math.
#[inline]
pub fn sat_add(a: usize, b: usize) -> usize {
    a.saturating_add(b)
}

/// Saturating subtraction for memory-safe usize math.
#[inline]
pub fn sat_sub(a: usize, b: usize) -> usize {
    a.saturating_sub(b)
}

/// Saturating multiplication for memory-safe usize math.
#[inline]
pub fn sat_mul(a: usize, b: usize) -> usize {
    a.saturating_mul(b)
}