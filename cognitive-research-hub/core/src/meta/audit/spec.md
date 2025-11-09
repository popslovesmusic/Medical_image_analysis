# Module: core/src/meta/audit/
# Spec Version: 1.0

## Purpose

The Audit module provides the tools to verify the "Zero Ambiguity Guarantee" of determinism. It compares `chronicle` logs, `MetricsSnapshot` data, and tensor checksums from different runs to provide a definitive pass/fail on reproducibility.

This module is the ultimate arbiter of the system's compliance with its core determinism mandate.

## Scope

* **Hash Verification:** Provides utilities to compute and compare SHA-256 (or similar) hashes of `chronicle` files and serialized tensors.
* **Log Comparison:** Implements functions to perform a "diff" on two `Chronicle` logs, highlighting any discrepancies in metrics, seeds, or actions.
* **Replay Auditing:** Orchestrates a full replay audit by loading a `chronicle` log and comparing its *original* metrics against a *newly computed* run using the same seeds.

## Core Data Structures

```rust
// A report summarizing the result of a deterministic audit.
pub struct AuditReport {
    pub run_a_hash: String,
    pub run_b_hash: String,
    pub is_identical: bool,
    pub first_mismatch_cycle: Option<u64>,
    pub mismatch_details: Option<String>,
}