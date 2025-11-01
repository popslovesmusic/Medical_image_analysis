core/src/src-spec.md
Purpose

This directory defines the computational substrate of the Chromatic Core — all executable modules that implement the deterministic reasoning, transformation, and memory logic defined in the core/ specifications.

The src/ layer represents the engine internals: it is the code counterpart to all conceptual documents (core-spec.md, meta-spec.md, etc.) and acts as the unified API for both high-level modules (trainer, bridge) and system introspection (diagnostics, dream).

Architectural Overview
core/src/
├─ src-spec.md                 ← this specification
├─ bridge/                     ← chromatic↔spectral mapping
├─ diagnostics/                ← evaluation, error metrics, reports
├─ dream/                      ← synthetic imagination & generative logic
├─ meta/                       ← chronicle, audit, and contextual metadata
├─ tensor/                     ← low-level deterministic math & color/spectral ops
├─ tests/                      ← integration and regression suites
└─ lib.rs                      ← core exports for the entire cognitive engine

Responsibilities by Module
Module	Primary Role	Key Interfaces
tensor/	Foundational math: deterministic tensors, quantization, color/spectral operations.	ChromaticTensor, SpectralTensor, FixedAccumulator
bridge/	Cross-domain transforms: RGB↔frequency, chromatic↔sonic, hue normalization, spectral reconstruction.	SpectralBridge, HueMap, EnergyCoherence
diagnostics/	Metrics and testing: coherence, loss, divergence, FFT analysis, and visualizations.	DiagnosticsSnapshot, CoherenceMetric, TrendAnalyzer
dream/	Synthetic imagination and memory generation: DreamPool, seeding, perturbation cycles.	DreamCycle, DreamPool, SolverResult
meta/	Context management: chronicle, audit logs, and causal metadata for deterministic replay.	Chronicle, ContextFrame, AuditReport
tests/	Unit and integration validation for reproducibility and determinism.	Rust test harness; cargo test integration
Module Interconnection Diagram
           ┌──────────────────┐
           │   Trainer (top)  │
           └──────┬───────────┘
                  │
        ┌─────────┼──────────┐
        │          │          │
   ┌────▼────┐ ┌───▼─────┐ ┌──▼───────┐
   │ bridge  │ │ dream   │ │ diagnostics│
   └────┬────┘ └────┬────┘ └────┬──────┘
        │           │           │
        ▼           ▼           ▼
      ┌─────────────────────────────┐
      │         tensor/             │
      └────────────┬────────────────┘
                   ▼
              ┌──────────┐
              │  meta/   │  ← chronicle + audit persistence
              └──────────┘

Deterministic Build Principles
Rule	Description
Fixed Dependencies	No dynamic linking; all modules use stable Rust crates pinned in Cargo.toml.
Seed Recording	Every random or noise process records its seed to the Chronicle.
Quantized Math	Tensor ops and spectral accumulation use fixed-point reductions.
Canonical Order	All loops are explicitly ordered by row/column/bin; no parallel race paths.
Cross-Platform Parity	Builds reproducibly under Windows and Linux (endianness-neutral I/O).
File Roles at core/src/ Root
File	Description
lib.rs	Exports unified public API: pub use for bridge, tensor, dream, meta, diagnostics.
src-spec.md	Documentation of this architecture and coding rules.
tests/	Cross-module tests for reproducibility, serialization, and full dream→learn→chronicle loop.
Integration Tests (core/src/tests/)
Test	Goal	Modules Involved
test_core_roundtrip.rs	Verify Chromatic→Spectral→Chromatic path consistency.	tensor, bridge
test_dream_replay.rs	Re-run identical DreamPool sequence under recorded seeds.	dream, meta
test_diagnostics_stability.rs	Validate metrics are invariant to order or platform.	diagnostics, tensor
test_core_api.rs	Ensure crate-level re-exports work.	lib.rs
Build & Usage
# Build the chromatic core
cargo build --release -p chromatic_core

# Run all integration tests
cargo test --workspace -- --nocapture

# Generate documentation
cargo doc --no-deps --open

Determinism Policy

All computations within core/src must satisfy the HQAPR standard
(High-Quality Audit and Predictable Reproduction).

Category	Requirement
Floating-Point Operations	Must be order-stable and quantized when summing more than 8 terms.
Randomness	All RNGs seeded and logged per cycle.
File I/O	Uses CRC64 for every .cten, .sten, .cmeta file.
Cross-System Parity	Identical hashes between Windows and Linux builds verified weekly.
Status
Field	Value
Spec Version	1.0
Alignment	Foundation for Phases 5–7
Dependencies	serde, ndarray, crc64, approx, rayon (optional, gated)
Determinism Level	Full deterministic replay
Implementation Readiness	✅ Stable for code generation
