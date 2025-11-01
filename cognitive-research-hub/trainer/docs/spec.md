cognitive-research-hub/trainer/docs/docs-spec.md
Purpose

The trainer/docs/ directory serves as the knowledge and reporting nucleus for all training operations.
It defines the methodology, experiment manifest, and logging schema used to interpret and reproduce results across phases (5–9).

It ensures that every training run — whether it’s a Dreamer, Learner, or Chromatic Bridge model — produces scientifically traceable documentation.

Directory Layout
cognitive-research-hub/trainer/docs/
├─ docs-spec.md                    ← this spec
├─ training_methodology.md         ← canonical overview of training loop design
├─ logging_schema.md               ← defines JSON log structure + allowed fields
├─ experiment_manifest.md          ← index of all runs, configs, and checkpoints
├─ evaluation_protocols.md         ← validation metrics, tolerance bands
├─ reproducibility_guide.md        ← deterministic pipeline walkthrough
├─ results_template.md             ← boilerplate for experiment reporting
└─ references/
   ├─ paper_links.md
   ├─ dataset_sources.md
   └─ license_citations.md

1. training_methodology.md

Describes how the Trainer subsystem orchestrates learning across three layers:

Layer	Module	Description
Data	data/ configs	Deterministic dataset partition and preprocessing
Model	trainer/src/model/	MLP, transformer, or hybrid network definitions
Cycle	trainer/src/training/	Epoch scheduling, feedback loops, and evaluation

It documents:

Phase-to-phase dependencies (5 → 6 → 7)

Batch formation and normalization rules

Weight initialization and optimizer parameters

Dream Pool augmentation protocol (Phase 6E)

Sonic bridge encoder–decoder synchronization (Phase 7)

All diagrams are drawn from docs/figures/ if present.

2. logging_schema.md

Defines the canonical JSON logging structure shared across all training modules.
Ensures logs are both machine-readable and human-interpretable.

Base Log Structure
{
  "timestamp": "2025-10-31T18:22:00Z",
  "phase": 6,
  "epoch": 12,
  "batch": 42,
  "loss": 0.0341,
  "accuracy": 0.963,
  "coherence": 0.812,
  "lr": 0.0005,
  "seed": 2025,
  "config_hash": "5c9d9bf...",
  "machine": {
    "os": "Windows 11",
    "device": "CPU",
    "threads": 1
  },
  "meta": {
    "commit": "8bcd3a9",
    "run_id": "phase6e_test_001",
    "user": "researcher"
  }
}

Validation Rules
Rule	Description
All numeric fields must be typed as floats or ints	no strings
All logs use UTC ISO-8601 timestamps	enforced by time_utils.py
Field order is canonical	defined alphabetically for hashing
Logs are stored as .jsonl (one JSON object per line)
MD5 checksum of full log file is written to experiment_manifest.md
3. experiment_manifest.md

Acts as the single source of truth for all experiments.

Structure
Field	Description
run_id	unique short name (phase7a_bridge_003)
phase	training phase
config_file	path + hash
output_dir	experiment output path
metrics_file	relative path to final CSV/JSON
status	success, failed, or partial
duration	runtime in seconds
notes	short text summary

Each completed run appends one Markdown-table row:

| Run ID | Phase | Config | Status | Accuracy | Coherence | Date |
|:-------|:------|:-------|:--------|:----------|:-----------|:----|
| phase6e_abtest_001 | 6E | config/phase6.toml | ✅ success | 0.92 | 0.83 | 2025-10-31 |


Manifest integrity is verified by hash_integrity.py.

4. evaluation_protocols.md

Defines quantitative benchmarks for Learner and Dreamer models.

Standard Metrics
Metric	Description	Threshold
Accuracy	Classification correctness	≥ 0.90
Coherence	Internal consistency metric (Dream Pool)	≥ 0.80
Spectral Drift	FFT energy deviation	≤ 0.5 dB
ΔColor	Color round-trip error	≤ 1e-3
ΔLoss	Loss improvement per 10 epochs	≤ 1e-4
Evaluation Scripts

trainer/src/validator/metrics.rs

scripts/visualization/metric_plotter.py

All evaluation runs must record both mean and variance across 3 seeds.

5. reproducibility_guide.md

A compact SOP (standard operating procedure) to reproduce any published result.

Key Steps

Clone the repo and checkout the tagged commit

Run scripts/utils/env_check.py and record hash

Copy the matching config from /trainer/config/

Execute via run_experiment.py with recorded seed

Verify that:

MD5(logs) == reference in experiment_manifest.md

Metrics fall within evaluation_protocols.md bounds

Determinism Assurance

Fixed thread count (threads.max=1)

No random data augmentations beyond those defined in config

Reproducible floating-point accumulation order

6. results_template.md

Provides the boilerplate format for new experiment reports.

# Experiment Report — {{RUN_ID}}

**Phase:** {{PHASE}}
**Config:** `{{CONFIG_PATH}}`
**Date:** {{DATE}}
**Commit:** {{COMMIT_HASH}}

---

## Summary
Short description of the experiment’s purpose.

## Configuration
| Parameter | Value |
|:-----------|:------|
| Learning Rate | 0.001 |
| Epochs | 25 |
| Dataset | Phase7 Color-Sonic Dataset |

## Results
| Metric | Train | Val |
|:--------|:------|:----|
| Loss | 0.024 | 0.030 |
| Accuracy | 0.95 | 0.93 |

## Observations
- Dream Pool convergence consistent with prior run
- FFT energy drift stable (0.3 dB)
- ΔColor within 1e-3 tolerance

## Files
- `metrics.json`
- `coherence_plot.png`

7. references/

Central repository for citations, dataset sources, and licenses.

File	Purpose
paper_links.md	links to cited academic papers
dataset_sources.md	provenance of medical image datasets
license_citations.md	all dependencies and their licenses

All references are validated during doc build to ensure links are live.

Pass Criteria
Validation	Condition
Log schema valid JSONL	✅
Manifest updated after every run	✅
Metric thresholds respected	✅
Documentation generated without warnings	✅
Reproducibility verified across 3 seeds	✅
Status
Field	Value
Spec Version	1.0
Phase Alignment	5 → 9
Determinism Level	Full documentation integrity
Readiness	✅ Complete
Next Spec	trainer/src/model/ (core trainer implementation and model interfaces)
