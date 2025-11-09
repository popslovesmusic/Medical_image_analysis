# Module: core/src/meta/session/
# Spec Version: 1.0

## Purpose

The Session module manages the high-level runtime context of the application. Its primary responsibility is to initialize and track the unique session identifier (Run ID), the initial RNG seed, and the configuration hash.

This ensures that every log entry and chronicle frame can be tied back to a specific, reproducible session, which is critical for deterministic audits.

## Scope

* Initializes a unique `ContextFrame` at application startup.
* Loads and hashes configuration files to create a `config_hash`.
* Establishes the root RNG seed for the entire application run.
* Provides read-only access to this `ContextFrame` for all other modules.

## Core Data Structures

This module defines and owns the canonical `ContextFrame` struct, which is referenced in `meta/spec.md`.

```rust
// Defined in meta/spec.md, but owned by this module.
pub struct ContextFrame {
    pub run_id: String,         // Unique session identifier (e.g., UUID)
    pub module: String,         // "SESSION_INIT"
    pub config_hash: String,    // SHA256 of all loaded configs
    pub environment: String,    // "Windows", "Linux", "WSL", etc.
}