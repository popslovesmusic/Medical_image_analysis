Specification: Validation & Evaluation Subsystem

Module Path: cognitive-research-hub/trainer/src/validator/
Parent Spec: trainer/src/spec.md
Related Specs:

trainer/src/training/spec.md — Provides trained model checkpoints

trainer/src/reports/spec.md — Receives structured summaries for export

core/diagnostics/spec.md — Supplies performance and error telemetry

core/tensor/spec.md — Supplies tensor normalization utilities

experiments/configs/spec.md — Provides validation configurations and datasets

I. Mission

The Validator Subsystem performs deterministic, reproducible evaluation of trained models.
Its primary role is to ensure that every trained model exhibits verifiable predictive stability and that metrics can be exactly recomputed from logs, datasets, and checkpoints.

This subsystem guarantees that evaluation results are not statistical artifacts, but consistent products of model–data interactions under fixed conditions.

II. Core Responsibilities
Function	Description	Determinism Level
Evaluation	Compute validation metrics across deterministic dataset partitions	Bitwise
Reproducibility	Re-run tests on previous checkpoints with identical outcomes	Bitwise
Metrics Logging	Record metrics for Reports and Chronicle integration	FP32 stable
Baseline Comparison	Compare new results against stored reference baselines	Bitwise
Bias Detection	Detect drift or bias in output distributions	Deterministic linear analysis
III. Directory Layout
validator/
├─ spec.md
├─ evaluator.rs          # Core evaluation routines (forward inference + scoring)
├─ metrics.rs            # Accuracy, precision, recall, F1, ROC, etc.
├─ confusion.rs          # Confusion matrix generation and normalization
├─ bias_analysis.rs      # Drift and bias detection across datasets
├─ baseline.rs           # Reference metric storage and comparison logic
├─ loader.rs             # Deterministic data loading for validation sets
├─ visualizer.rs         # Optional metrics plotting
└─ tests/
    ├─ validator_roundtrip.rs
    ├─ baseline_replay.rs
    ├─ metrics_consistency.rs

IV. Validation Workflow
(checkpoint) 
    │
    ▼
[evaluator.rs] 
    │
    ▼
(metrics.rs) 
    │
    ├─→ [confusion.rs]
    │
    ├─→ [bias_analysis.rs]
    │
    ▼
(baseline.rs)
    │
    ├─ compare() → pass/fail
    └─ update()  → new baseline (if approved)


The validator acts as both auditor and verifier, ensuring every inference cycle is comparable to historical benchmarks and traceable to its originating training session.

V. Evaluation Metrics

The subsystem provides a fixed, non-extensible metric set for deterministic reproducibility.
All metrics are computed using FP32 precision and stored with six-decimal truncation.

Metric	Formula	Application
Accuracy	$(TP + TN) / (TP + TN + FP + FN)$	General performance
Precision	$TP / (TP + FP)$	Medical risk thresholding
Recall (Sensitivity)	$TP / (TP + FN)$	Missed detection rate
Specificity	$TN / (TN + FP)$	False-positive control
F1 Score	$2·(Precision·Recall)/(Precision + Recall)$	Balanced metric
AUC (ROC)	Deterministic trapezoidal integration	Discrimination capacity
Dice Coefficient	$2	A∩B
Chromatic Coherence	Mean hue continuity across image slices	Visual phase validation
VI. Deterministic Mechanisms
Component	Mechanism	Guarantee
Dataset Split	Pre-seeded stratified sampler	Reuse identical partitions
Metric Accumulation	Kahan-sum accumulation	Drift-free FP32 aggregation
RNG Handling	Fixed seed + local offset per fold	Identical inference noise
Confusion Matrix	Deterministic bin assignment	No rounding ambiguity
Bias Analysis	Linear trend fit with fixed-order regression	Reproducible bias score
Baseline Comparison	Frozen JSON schema	Bitwise equality verification
VII. Bias and Drift Analysis

The bias_analysis.rs module evaluates statistical consistency across validation runs:

Test	Description	Result
Prediction Drift Test	Compares mean predicted probabilities vs baseline	Δ ≤ 0.002
Hue Coherence Test	Measures chromatic uniformity for imaging tasks	≥ 0.98
Confusion Stability Test	Verifies confusion matrix structure stability	≤ 2% change per class
Output Distribution Shift	KL-divergence between output histograms	≤ 1e-4

Failures trigger a Diagnostics Agent flag and write structured logs into:
/core/diagnostics/metrics/validation_bias.json

VIII. Integration Points
Source	Role
trainer/src/training	Supplies trained model + checkpoint
trainer/src/tokenizer	Provides deterministic input encodings
core/diagnostics	Receives bias and drift telemetry
trainer/src/reports	Generates validation summaries
experiments/results	Archives metrics and confusion plots
IX. Baseline Comparison & Reproducibility

Every validation run compares its results against a previously stored baseline manifest:

{
  "baseline_run": "20251025T224512Z",
  "accuracy": 0.912347,
  "f1": 0.903155,
  "auc": 0.963902
}


Re-running the same checkpoint with identical config must reproduce metrics bit-for-bit identical to within machine rounding.
If a new run improves performance while satisfying all deterministic constraints, the baseline may be promoted after manual audit.

X. Testing Protocols
Test	Condition	Expected Result
Roundtrip Validation	Evaluate same checkpoint twice	Identical metrics JSON
Cross-Platform Test	Linux vs Windows	Bitwise identical hashes
Baseline Replay Test	Compare new vs old	Pass if within Δ ≤ 1e-6
Drift Consistency Test	Drift regression	Deterministic slope
Confusion Matrix Test	Matrix normalization	Deterministic cell counts
XI. Output Files
File	Description
validation_metrics.json	Primary metrics file
confusion_matrix.json	Raw normalized confusion matrix
bias_report.json	Drift and bias summary
validation_summary.md	Human-readable report
baseline_manifest.json	Reference baseline for reproducibility
validator_log.csv	Audit trail (hash, seed, runtime, version)
XII. Compliance Summary
Field	Specification
Spec Version	1.0
Precision Mode	FP32
Determinism Level	Bitwise Equivalent
Validation Baseline Policy	Immutable JSON reference
Audit Agent	Validation Integrity Agent
Bias Threshold	Δ ≤ 1e-3 allowed
Chronicle Link	Phase 6E integration
Status	✅ Verified Validator Spec Complete