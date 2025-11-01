Specification: Comparative Results Subsystem

Module Path: experiments/results/comparisons/
Parent Specs:

experiments/results/spec.md (result generation and storage)

experiments/configs/spec.md (parameter provenance)

core/diagnostics/spec.md (metric computation)

core/meta/chronicle/spec.md (chronological experiment tracking)

I. Mission

The Comparisons Subsystem enables structured, deterministic statistical analysis between experiments.
It exists to answer a single question:

“Did the new model, dataset, or algorithm produce a statistically significant improvement — and is that improvement reproducible?”

This layer integrates raw metric files (metrics/*.json, metrics/*.csv) from two or more experiment runs and outputs structured, human- and machine-readable deltas, complete with significance scores and metadata for publication.

II. Directory Layout
experiments/results/comparisons/
├─ reports/               # Markdown summaries of comparisons
├─ csv/                   # Raw numeric comparison tables
├─ json/                  # Machine-readable comparison data
├─ plots/                 # Visualized metric deltas
├─ logs/                  # Process logs and error diagnostics
└─ templates/             # Reusable Jupyter/Matplotlib comparison templates

III. Input Sources

Comparisons are computed using validated metrics only:

Source	Description	Validation
/experiments/results/metrics/	JSON or CSV metric outputs	Must pass manifest integrity check
/experiments/results/manifests/	Metadata linking to configs	Ensures experiment ID and SHA alignment
/experiments/configs/validated/	Configurations used in runs	Confirms comparability between experiments

Each pair (or group) of experiments to be compared is defined in a YAML comparison descriptor.

Example descriptor:

comparison_id: "cmp_2025_11_01_phase7a_vs_phase7b"
experiments:
  baseline: "exp_2025_10_29_phase7a_base"
  variant:  "exp_2025_10_31_phase7b_bridge"
metrics: ["accuracy", "f1_score", "energy_drift_db"]
tests: ["delta", "paired_t", "bootstrap"]
confidence_level: 0.95
notes: "Testing Phase 7 bridge integration improvements."

IV. Deterministic Comparison Workflow
Step	Description	Output
1. Parse Metrics	Load and align metrics from baseline & variant	raw_data.json
2. Normalize Units	Apply consistent rounding and float precision (1e⁻⁶)	Standardized arrays
3. Compute Deltas	Calculate mean difference per metric	delta_table.csv
4. Test Significance	Apply fixed-seed resampling or analytical tests	significance.json
5. Generate Report	Produce Markdown + plots for publication	reports/*.md
V. Mathematical Specification
1. Metric Delta (Δ)

For each metric 
𝑀
M:

Δ
𝑀
=
𝑀
𝑣
𝑎
𝑟
𝑖
𝑎
𝑛
𝑡
−
𝑀
𝑏
𝑎
𝑠
𝑒
𝑙
𝑖
𝑛
𝑒
ΔM=M
variant
	​

−M
baseline
	​


where 
𝑀
M is the arithmetic mean of all validation samples.

2. Paired t-Test

Assumes identical sample pairs between runs:

𝑡
=
𝑑
ˉ
𝑠
𝑑
/
𝑛
t=
s
d
	​

/
n
	​

d
ˉ
	​


where 
𝑑
ˉ
d
ˉ
 is mean difference, 
𝑠
𝑑
s
d
	​

 is standard deviation, 
𝑛
n is sample count.

3. Bootstrap Resampling (Fixed Seed)

When samples differ in order/size, use fixed-seed bootstrap:

𝐶
𝐼
95
%
=
Percentile
[
2.5
,
97.5
]
(
bootstrap
(
Δ
𝑀
,
𝑁
=
10
4
)
)
CI
95%
	​

=Percentile
[2.5,97.5]
	​

(bootstrap(ΔM,N=10
4
))

All random resampling uses seed = 42 for determinism.

VI. Output Schema
1. JSON Format (json/comparison_<id>.json)
{
  "comparison_id": "cmp_2025_11_01_phase7a_vs_phase7b",
  "baseline": "exp_2025_10_29_phase7a_base",
  "variant": "exp_2025_10_31_phase7b_bridge",
  "metrics": {
    "accuracy": { "baseline": 0.912, "variant": 0.934, "delta": 0.022, "p_value": 0.018 },
    "f1_score": { "baseline": 0.894, "variant": 0.915, "delta": 0.021, "p_value": 0.025 },
    "energy_drift_db": { "baseline": 0.58, "variant": 0.45, "delta": -0.13, "p_value": 0.041 }
  },
  "confidence_level": 0.95,
  "significant_metrics": ["accuracy", "f1_score"],
  "timestamp": "2025-11-01T22:50:00Z",
  "seed": 42
}

2. CSV Format (csv/comparison_<id>.csv)

| Metric | Baseline | Variant | Δ | p-value | Significant |
|---------|-----------|----------|----------|-------------|
| Accuracy | 0.912 | 0.934 | +0.022 | 0.018 | ✅ |
| F1 Score | 0.894 | 0.915 | +0.021 | 0.025 | ✅ |
| Energy Drift (dB) | 0.58 | 0.45 | −0.13 | 0.041 | ✅ |

VII. Visual Outputs
1. Delta Plots

Plotted using matplotlib (deterministic backend: Agg)

Consistent palette derived from the Chromatic Tensor hue map:

Positive deltas → green

Negative deltas → red

Non-significant → gray

2. Confidence Bands

All plots include 95% confidence intervals as fixed-width shaded regions, computed via pre-seeded bootstrap.

3. Export

Stored as:

plots/comparison_<id>_delta.svg

plots/comparison_<id>_confidence.svg

All plots include embedded metadata (experiment IDs, hash, timestamp).

VIII. Deterministic Rules
Rule	Enforcement
Global seed	42 for all tests and resampling
Float precision	6 decimal places
RNG backend	numpy.random.default_rng(seed)
Date/time	UTC ISO 8601 only
Metric alignment	Lexicographic sort of metric keys
File ordering	Sorted inclusion order before hashing
Line endings	LF (UTF-8)
Plot hash	SHA-256 of SVG content recorded in manifest
IX. Statistical Test Set
Test	Use Case	Deterministic Property
Delta	Simple performance gap	Algebraic
Paired t-test	Same dataset, same samples	Analytical
Bootstrap (seeded)	Different sample sets	Stochastic, fixed-seed
Effect size (Cohen’s d)	Scale of improvement	Derived algebraically
Energy coherence delta	Spectral stability measure	Derived from FFT consistency

All tests are implemented in scripts/run_comparisons.py and validated through tests/comparison_tests/.

X. Report Template

Example:
reports/exp_phase7a_vs_phase7b_summary.md

# Experiment Comparison: Phase 7A vs Phase 7B

| Metric | Baseline | Variant | Δ | Significance |
|---------|-----------|----------|---------------|
| Accuracy | 0.912 | 0.934 | +0.022 | ✅ p=0.018 |
| F1 Score | 0.894 | 0.915 | +0.021 | ✅ p=0.025 |
| Energy Drift (dB) | 0.58 | 0.45 | −0.13 | ✅ p=0.041 |

**Result:**  
✅ Statistically significant improvement in accuracy, F1, and energy coherence.  
⚙️ No regression detected.  

Generated automatically by `scripts/generate_comparison_report.py`

XI. Automation Scripts
Script	Function
generate_comparison_descriptor.py	Create new comparison YAML file
run_comparisons.py	Execute all tests deterministically
plot_comparison_results.py	Render deltas with CI
generate_comparison_report.py	Build Markdown summaries
verify_comparison_integrity.py	Cross-check hashes and inputs
XII. Integration and Data Flow
Source	Destination	Description
experiments/results/metrics/	comparisons/json/	Metric ingestion
comparisons/json/	comparisons/reports/	Report generation
comparisons/csv/	docs/reports/	Publication
core/meta/chronicle/	comparisons/logs/	Audit entries for reproducibility
XIII. Compliance Rules
Rule	Guarantee
Fixed RNG seed	Ensures identical confidence intervals
Analytical fallback	Bootstrap replaced by t-test if sample mismatch <2%
No floating RNG state	RNG reinitialized per metric
Plot determinism	Fixed-size, fixed font
Hash registry	SHA-256 stored in comparison manifest
XIV. Compliance Summary
Field	Specification
Spec Version	1.0
Determinism Level	Bit-Exact
Statistical Tests	Deterministic, seeded bootstrap
Hash Algorithm	SHA-256
Audit Authority	Codex Comparison Agent
Revision	{{auto-date}}
Status	✅ Verified Comparative Integrity