Specification: Training Engine

Module Path: cognitive-research-hub/trainer/src/training/
Parent Spec: trainer/src/spec.md
Related Specs:

trainer/src/model/spec.md — Model definition and forward/backward functions

trainer/src/tokenizer/spec.md — Input tokenization and dataset preparation

trainer/src/validator/spec.md — Validation and performance scoring

trainer/src/reports/spec.md — Logging and export of training metrics

core/diagnostics/spec.md — Error detection and continuity tracking

I. Mission

The Training Engine executes the full learning process — from data ingestion and gradient computation to weight updates, checkpointing, and chronicle recording — under strict determinism guarantees.

Its design ensures that any training run can be exactly replayed from a given configuration hash, dataset state, and seed, producing bit-identical results across systems and sessions.

II. Core Responsibilities
Function	Description	Determinism Level
Dataset Management	Load and shuffle batches via seed-controlled index streams	Bitwise
Forward Pass Execution	Compute model outputs from tokenized inputs	Bitwise
Loss Computation	Compute scalar objectives deterministically	FP32
Backward Pass	Calculate exact gradients with no floating rounding drift	FP32
Optimization Step	Apply updates via fixed-order operations	Bitwise
Checkpointing	Save model + optimizer state + RNG seed	Bitwise
Chronicle Update	Log per-step metrics for continuity	FP32 stable
III. Directory Layout
training/
├─ spec.md
├─ engine.rs                 # Core training loop, epoch orchestration
├─ optimizer.rs              # Deterministic optimizer implementations (SGD, Adam)
├─ scheduler.rs              # Learning-rate control (cosine, step, linear)
├─ hooks.rs                  # Pre-/post-step diagnostic callbacks
├─ checkpoint.rs             # Save/load model + optimizer + RNG state
├─ chronicle_writer.rs       # Log structured training continuity records
├─ loss_functions.rs         # MSE, CrossEntropy, StructuralLoss, etc.
├─ metrics.rs                # Running averages, EMA, gradient norms
└─ tests/
    ├─ deterministic_training.rs
    ├─ checkpoint_roundtrip.rs

IV. Training Workflow
config → dataset → tokenizer → model → loss → optimizer → validator → reports
                    │
                    └──→ chronicle (phase 6 integration)


The engine executes the canonical Cycle Record pipeline:

Batch Load: Samples fetched deterministically via seeded shuffle index.

Forward Pass: Compute outputs through the model.

Loss Calculation: Deterministic aggregation of scalar loss.

Backward Pass: Gradient accumulation with strict FP32 ordering.

Optimizer Step: Update weights in fixed order per layer.

Metrics Logging: Write to Chronicle and local reports.

Checkpoint Save: Serialize full system state every N epochs.

V. Deterministic Mechanisms
Component	Mechanism	Guarantee
RNG	Fixed global seed, propagated across dataloader, augmentations, model init	Identical sequences
GPU Kernels	Deterministic ops only (disable atomic-add variance)	Stable accumulation
Optimizer Order	Layer-update order fixed by model key sort	Bitwise repeatability
Loss Aggregation	Accumulate via Kahan-sum compensated FP32 addition	No precision drift
Checkpoint Hashing	SHA256(model + optimizer + RNG + config)	Replay validation
Chronicle Logging	Step-indexed, fixed-field JSON lines	Temporal continuity trace
VI. Loss Functions

The module exposes a small set of deterministic, medically relevant loss functions:

Name	Formula	Application
MSELoss	$(y - \hat{y})^2$	Regression, reconstruction
CrossEntropyLoss	$-\sum y_i \log(\hat{y_i})$	Classification
StructuralLoss	Weighted edge-preserving loss for medical images	MRI, CT scans
ChromaticCoherenceLoss	Penalizes discontinuity in chromatic embeddings	DreamPool coherence learning
TemporalSmoothnessLoss	Enforces stable learning curves	Recurrent modules

All losses are implemented in deterministic FP32 arithmetic and batch-reduced using fixed-axis summation.

VII. Optimizers
Optimizer	Description	Determinism Feature
SGD	Momentum + dampening	Fixed update order
Adam (Deterministic)	AdamW variant with frozen epsilon ordering	Bitwise stable
RMSProp	For medical imaging, where gradient magnitudes vary widely	FP32 sum normalization
ChromaticSGD	Custom variant aligned with ChromaticTensor scaling	Phase 7 integration-ready
VIII. Scheduler

Implements reproducible learning-rate adjustment policies:

LinearDecay – simple linear drop over epochs

CosineAnnealing – deterministic trigonometric curve

StepDecay – explicit epoch milestones

Each scheduler includes a serialize() method producing a config hash that verifies deterministic state alignment.

IX. Checkpointing

All checkpoints include:

model_state

optimizer_state

rng_state

epoch_index

config_hash

chronicle_offset

Saved as deterministic Rust binary blobs (.chk) with SHA256 integrity headers.
The checkpoint_roundtrip.rs test ensures byte-for-byte reproducibility after load.

X. Integration with Chronicle

The training engine writes structured temporal records for each cycle into the Phase 6 Chronicle subsystem:

{
  "epoch": 42,
  "batch": 120,
  "loss": 0.01452,
  "coherence": 0.9831,
  "lr": 0.0003,
  "grad_norm": 1.21,
  "timestamp": 5234.37
}


Chronicle guarantees global synchronization between model performance trends and diagnostics telemetry.

XI. Testing Protocols
Test	Purpose	Expected Result
Replay Test	Run same config twice	Identical output hash
Checkpoint Roundtrip	Save → Load → Resume	No drift
Loss Consistency	CPU vs GPU	≤ 1e-8 diff
Optimizer Reproducibility	Layer-order check	Same update sequence
Chronicle Sync Test	Step-by-step continuity log	No gaps
XII. Compliance Summary
Field	Specification
Spec Version	1.0
Determinism Level	Full Bitwise
Precision Mode	FP32 default
Audit Agent	Continuity Control Agent
Chronicle Integration	Phase 6C–6E synchronized
Hash Integrity	SHA256 per checkpoint
Status	✅ Verified Training Engine Spec Complete