# Module: core/src/
# Spec Version: 1.1 (Aligned with canonical roadmap and modules)

## Purpose

This directory defines the computational substrate of the Chromatic Core — all executable modules that implement the deterministic reasoning, transformation, and memory logic defined in the `core/` specifications.

The `src/` layer represents the engine internals: it is the code counterpart to all conceptual documents and acts as the unified API for both high-level modules (like `trainer`) and system introspection.

## Architectural Overview
`core/src/`
├─ src-spec.md                 ← this specification
├─ bridge/                     ← chromatic↔spectral mapping
├─ diagnostics/                ← evaluation, error metrics, reports
├─ dream/                      ← synthetic imagination & generative logic
├─ error.rs                    ← canonical DreamError type
├─ meta/                       ← chronicle, audit, and contextual metadata
├─ tensor/                     ← low-level deterministic math & color/spectral ops
├─ utils/                      ← saturating arithmetic & core utilities
├─ tests/                      ← integration and regression suites (external to src/)
└─ lib.rs                      ← core exports for the entire cognitive engine

## Responsibilities by Module
Module	Primary Role	Key Interfaces
`tensor/`	Foundational math: deterministic tensors, quantization, color/spectral operations.	`ChromaticTensor`, `SpectralTensor`
`bridge/`	Cross-domain transforms: RGB↔frequency, hue normalization, spectral reconstruction.	`SpectralBridge`, `HueMap`
`diagnostics/`	Metrics and testing: coherence, loss, divergence, FFT analysis.	`DiagnosticsSnapshot`, `CoherenceMetric`
`dream/`	Synthetic imagination and memory generation: DreamPool, seeding, perturbation cycles.	`DreamCycle`, `DreamPool`
`meta/`	Context management: chronicle, audit logs, and causal metadata for deterministic replay.	`Chronicle`, `ContextFrame`
`error.rs`	Canonical error handling for the crate.	`DreamError`, `CoreResult`
`utils/`	ZAG compliance utilities.	`sat_add`, `sat_sub`, `sat_mul`
`lib.rs`	Exports unified public API for the crate.	`pub use ...`

## Module Interconnection Diagram
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
                   │
    ┌──────────────┴──────────────┐
    │                             │
┌───▼───┐  ┌──────────┐  ┌────────▼──┐
│ error │  │  meta/   │  │  utils/   │
└───────┘  └──────────┘  └───────────┘
   (Used by all modules)   (Used by all modules)

## Deterministic Build Principles
Rule	Description
Fixed Dependencies	No dynamic linking; all modules use stable Rust crates pinned in `Cargo.toml`.
Seed Recording	Every random or noise process records its seed to the Chronicle.
Saturating Math	All `usize` memory/index math MUST use `utils::sat_add` etc.
Canonical Order	All loops are explicitly ordered; no parallel race paths.
Cross-Platform Parity	Builds reproducibly under Windows and Linux.

## File Roles at core/src/ Root
File	Description
`lib.rs`	Exports unified public API: `pub use` for all modules.
`error.rs`	Defines `DreamError`.
`src-spec.md`	Documentation of this architecture and coding rules.

## Integration Tests (Located in `core/tests/`)
Test	Goal	Modules Involved
`test_core_roundtrip.rs`	Verify Chromatic→Spectral→Chromatic path consistency.	`tensor`, `bridge`
`test_dream_replay.rs`	Re-run identical DreamPool sequence under recorded seeds.	`dream`, `meta`
`test_diagnostics_stability.rs`	Validate metrics are invariant to order or platform.	`diagnostics`, `tensor`
`test_core_api.rs`	Ensure crate-level re-exports work.	`lib.rs`
`test_error_handling.rs`	Ensure modules correctly propagate `DreamError`.	`error`, all modules
`test_saturating_math.rs`	Verify `utils` helpers are used and prevent overflow.	`utils`, `tensor`

## Build & Usage
```bash
# Build the chromatic core
cargo build --release -p chromatic_core

# Run all integration tests
cargo test --workspace -- --nocapture

# Generate documentation
cargo doc --no-deps --open