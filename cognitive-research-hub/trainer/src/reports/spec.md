# Module: trainer/src/reports/
# Spec Version: 1.0
# Purpose

This module is responsible for generating the final, human-readable and machine-readable reports from a training or validation run.

It collects all metrics, configuration details, and validation results and serializes them into deterministic, standardized formats (e.g., JSON, Markdown) with SHA-256 signatures for integrity.

## Scope

* **JSON Reporting:** Serializes `ValidationReport` and `TrainingMetrics` into a `metrics.json` file.
* **Markdown Reporting:** Generates a human-readable `run_summary.md` file, including key metrics and configuration details.
* **Signature Generation:** Computes a SHA-256 hash of the generated reports to ensure they have not been tampered with.

## Core Data Structures

```rust
// Uses `ValidationReport` from `validator/mod.rs`
// Uses `TrainerConfig` from `config/schema.rs`

// Placeholder for the final combined report
pub struct FinalReport {
    pub config: TrainerConfig,
    pub validation: ValidationReport,
    pub report_hash: String,
}