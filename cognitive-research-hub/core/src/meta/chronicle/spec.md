core/src/meta/chronicle/chronicle-spec.md
Purpose

The Chronicle Module is the system’s temporal ledger — a deterministic event recorder that tracks every measurable transformation within the Chromatic Core.
It provides the data substrate for diagnostics, learning analysis, and continuity planning by maintaining a coherent sequence of CycleRecords.

Its design ensures bit-exact replay of experiments and supports the validation of long-term temporal coherence in both dream and learner subsystems.

Scope
Subsystem	Function
Cycle Recorder	Captures state snapshots at the end of each iteration (Dream, Train, or Validation Cycle).
Chronicle Writer	Serializes and stores cycle data to persistent log files or databases.
Chronicle Reader	Provides deterministic replay and selective query access.
Compression Layer	Applies reversible delta compression to minimize storage without losing precision.
Audit Interface	Enables external or internal modules to verify experiment reproducibility.
Core Data Structures
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CycleRecord {
    pub timestamp: u64,                  // Monotonic cycle index or epoch time
    pub phase: String,                   // Phase label: "dream", "train", "validate"
    pub metrics: DiagnosticsSnapshot,    // Aggregated diagnostic metrics
    pub tensor_signature: [f32; 3],      // Mean chromatic vector
    pub loss_value: f32,                 // Learner or evaluator loss
    pub coherence: f32,                  // Dream pool or tensor coherence
    pub action_code: Option<u8>,         // If a TemporalAction was triggered
    pub seed_state: u64,                 // RNG seed for deterministic replay
}

pub struct Chronicle {
    pub records: Vec<CycleRecord>,
    pub max_records: usize,
    pub file_path: PathBuf,
}

Functional Overview
Function	Signature	Description
record_cycle()	(rec: CycleRecord, chronicle: &mut Chronicle)	Appends a new deterministic record to memory and persistent log.
save_chronicle()	(chronicle: &Chronicle) -> Result<(), IOError>	Serializes the entire record chain to disk (.cmeta file).
load_chronicle()	(path: &Path) -> Result<Chronicle, IOError>	Loads a chronicle and reconstructs full sequence deterministically.
query_range()	(start: u64, end: u64) -> Vec<CycleRecord>	Returns all records within the given time window.
compress_records()	(chronicle: &mut Chronicle)	Performs reversible delta compression on numeric fields.
replay_cycle()	(record: &CycleRecord) -> ReplayState	Reconstructs system state from saved seed, ensuring perfect replay.
Compression and Determinism

To achieve exact replay while saving storage:

Strategy	Description
Delta Compression	Store only the difference from the previous record: Δloss, Δcoherence, etc.
Fixed-point Quantization	Round floating-point values to deterministic 1e-6 granularity before compression.
Static RNG Restoration	Each record logs the RNG seed; replay sets this seed before regeneration.
Ordered Write Buffering	Chronological index enforced before disk serialization.

All compression and decompression routines are invertible and validated against checksum hashes per record batch.

Chronicle File Format (.cmeta)
Field	Type	Description
header.signature	u32	Magic bytes 0xC0DA55E
header.version	u16	Spec version for backward compatibility
records_count	u32	Number of stored cycles
payload	Binary	Serialized compressed record stream
checksum	u64	CRC64 hash of full payload for integrity check
Deterministic Guarantees
Concern	Mechanism
Record ordering	Monotonic timestamp enforced via phase index
Floating-point drift	Fixed-point quantization pre-serialization
Reproducibility	RNG seed + serialized configuration stored per cycle
Multi-thread race safety	Mutex-guarded append queue, deterministic flush interval
Replay verification	Hash-matching between original and reconstructed metrics
Integration Points
Module	Direction	Role
dream	Input	Logs every synthetic generation cycle
learner	Input	Records training/validation metrics
diagnostics	Input	Inserts derived metrics snapshots
continuity	Input/Output	Reads for trend analysis; writes TemporalAction triggers
scripts	Output	Used for generating plots, summaries, and audit reports
Validation Tests
Test	Description	Expected Result
test_chronicle_record_append	Verify records append in chronological order	Monotonic timestamps
test_chronicle_compression_roundtrip	Compress → Decompress → Compare	Bitwise identical
test_deterministic_replay	Replay identical sequence using saved seeds	Reconstructed tensors match originals
test_file_integrity	CRC64 validation	Pass checksum verification
test_query_range	Query returns correct subset	Record count and order match expectation
File Layout
chronicle/
├─ chronicle-spec.md           ← this specification
├─ recorder.rs                 ← cycle recording logic
├─ serializer.rs               ← serialization and compression
├─ reader.rs                   ← deterministic replay interface
├─ audit.rs                    ← reproducibility checks and hash validation
├─ tests/
│   ├─ test_record.rs
│   ├─ test_compression.rs
│   ├─ test_replay.rs
│   ├─ test_query.rs
└─ formats/
    ├─ cmeta_format.rs
    └─ cmeta_schema.toml

Status
Field	Value
Spec Version	1.0
Phase Alignment	6B → 7C → 8A
Dependencies	dream, metrics, continuity, diagnostics
Determinism Level	Full Replay & Bit-Exact Compression
Readiness	✅ Implementation Ready
Next Module	core/src/meta/logger/ (lightweight streaming interface for external monitoring)
