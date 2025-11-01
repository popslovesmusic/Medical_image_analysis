core/src/meta/meta-spec.md
Purpose

The Meta Layer defines the self-referential control surface of the Chromatic Core.
It coordinates logging, chronicle recording, semantic indexing, and context management, ensuring every computational event can be traced, reproduced, and understood in its causal context.

This layer’s purpose is to provide time-aligned, information-complete introspection — transforming raw operations (dream, learn, analyze) into a coherent historical record.

Subsystems Overview
Submodule	Function	Primary Output
chronicle/	Deterministic time-series recorder	.cmeta files and replay streams
logger/	Real-time message streaming and event broadcast	Console/UI diagnostics
session/	Runtime session context tracking	Run UUIDs, seed states, and paths
context/	Environmental description and causal metadata	ContextFrame objects
audit/	Integrity verification and state comparison	Hash and replay audits
Core Data Structures
pub struct MetaFrame {
    pub cycle_id: u64,
    pub timestamp: u64,
    pub context: ContextFrame,
    pub diagnostics: DiagnosticsSnapshot,
    pub rng_seed: u64,
    pub phase: String,
}

pub struct ContextFrame {
    pub run_id: String,         // Unique session identifier
    pub module: String,         // Current executing subsystem
    pub config_hash: String,    // SHA256 of loaded config
    pub environment: String,    // "Windows", "Linux", "WSL", etc.
}

Functional Overview
Function	Signature	Description
init_session()	() -> ContextFrame	Initializes a deterministic runtime context and assigns UUID.
log_event()	(msg: &str, level: LogLevel)	Appends formatted event messages with timestamps to the meta log.
record_frame()	(frame: &MetaFrame)	Stores the frame in both in-memory log and chronicle/.
export_metadata()	(chronicle: &Chronicle, format: ExportFormat)	Outputs summary CSV/JSON for external analysis.
verify_integrity()	(path: &Path) -> AuditReport	Performs hash verification of recorded run.
Deterministic Rules
Rule	Description
Fixed Timestamp Source	All modules use the same monotonic tick counter, not system clock.
Ordered Logging	Log messages are buffered and sorted by tick before flush.
Static Context Hash	Config + environment hash included in every MetaFrame.
Replay Consistency	Replaying a Chronicle regenerates identical ContextFrames.
File Layout
meta/
├─ meta-spec.md                ← this specification
├─ chronicle/                  ← deterministic time-series recorder
│   ├─ chronicle-spec.md
│   ├─ recorder.rs
│   ├─ serializer.rs
│   ├─ reader.rs
│   └─ audit.rs
├─ logger/                     ← real-time log streamer and message bus
│   ├─ logger-spec.md
│   ├─ console.rs
│   ├─ file.rs
│   └─ tests/
├─ session/                    ← session context manager
│   ├─ session-spec.md
│   ├─ context.rs
│   └─ tests/
├─ audit/                      ← replay verification utilities
│   ├─ audit-spec.md
│   ├─ hashcheck.rs
│   ├─ comparer.rs
│   └─ tests/
└─ tests/
    ├─ test_meta.rs
    ├─ test_logger.rs
    ├─ test_audit.rs

Validation Tests
Test	Description	Pass Criterion
test_context_consistency	Ensure identical context hashes across modules	Equal hash values
test_deterministic_logging	Same events under same seed produce identical log files	Bitwise equality
test_replay_audit	Replay a Chronicle and compare diagnostics snapshot	≤1e-6 drift
test_integrity_hash	Verify stored hash matches recomputed hash	✅ Match
test_environment_detection	Correctly identifies platform without breaking determinism	Stable enum output
Status
Field	Value
Spec Version	1.0
Phase Alignment	Global (meta across all phases)
Dependencies	diagnostics, continuity, dream, trainer
Determinism Level	Complete — bit-accurate replay across OS boundaries
Readiness	✅ Ready for implementation
Next Module	core/src/tensor/ (ChromaticTensor foundational specification)
