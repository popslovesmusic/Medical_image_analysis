# Module: trainer/config/
# Spec Version: 1.0
# Purpose

This module defines the configuration schema for the Trainer Subsystem. It ensures that all parameters (engine, model, dataset, optimizer) are loaded, validated, and hashed in a deterministic way.

This provides the immutable, auditable foundation required for reproducible training runs.

## Scope

* Defines the structure of all `.toml` configuration files.
* Provides utilities to load, parse, and validate these files.
* Implements a deterministic hashing function to create a `config_hash` for the `meta/session/ContextFrame`.

## Core Data Structures

```rust
// Placeholder struct for the main engine configuration
pub struct EngineConfig {
    pub base_seed: u64,
    pub batch_size: usize,
    pub epochs: u32,
    pub learning_rate: f32,
}

// Placeholder struct for model-specific parameters
pub struct ModelConfig {
    pub model_type: String, // e.g., "DeterministicCNN"
    pub layers: u32,
    pub activation: String, // e.g., "ReLU"
}

// Placeholder for the combined, validated configuration
pub struct TrainerConfig {
    pub engine: EngineConfig,
    pub model: ModelConfig,
    pub config_hash: String,
}