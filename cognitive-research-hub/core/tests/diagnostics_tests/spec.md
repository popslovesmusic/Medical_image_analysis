core/tests/diagnostics_tests/diagnostics-tests-spec.md
Purpose

The Diagnostics Tests suite verifies that all metrics, coherence analyses, and spectral evaluations from
core/src/diagnostics/ are numerically deterministic, order-invariant, and consistent with analytical reference data.

It also validates integration between diagnostics, tensor, bridge, and dream modules.

Directory Layout
core/tests/diagnostics_tests/
├─ diagnostics-tests-spec.md          ← this document
├─ test_coherence.rs                  ← coherence metric stability
├─ test_energy_consistency.rs         ← spectral energy and centroid precision
├─ test_delta_color.rs                ← ΔHSL and ΔRGB correctness
├─ test_loss.rs                       ← loss surface determinism
├─ test_report_generation.rs          ← audit & report output validation
├─ test_trend_analysis.rs             ← continuity planner trend regression
└─ fixtures/
   ├─ tensors/
   │  ├─ chromatic_reference.cten
   │  ├─ spectral_reference.sten
   │  └─ degraded_examples/
   ├─ metrics_baseline.json
   ├─ tolerances.toml
   └─ seeds.toml

Test Categories
Test	Objective	Key Modules
test_coherence.rs	Verify that computed coherence scores are order- and platform-invariant.	diagnostics::metrics, tensor::spectral
test_energy_consistency.rs	Confirm total spectral energy equals chromatic intensity within tolerance.	tensor, bridge
test_delta_color.rs	Validate ΔHSL seam safety and perceptual ΔE accuracy.	diagnostics::metrics, tensor::color
test_loss.rs	Ensure deterministic loss curves under repeated evaluation (DreamPool checkpoints).	dream, diagnostics::loss
test_report_generation.rs	Compare generated .md and .json diagnostic reports with known templates.	diagnostics::report, meta::chronicle
test_trend_analysis.rs	Validate continuity planner’s slope and regression outputs for time series.	diagnostics::trend, meta::chronicle
Deterministic Validation Rules
Rule	Enforcement
Fixed Seeds	All random or sampling routines initialized with fixtures/seeds.toml.
Canonical Order	Metrics iterate over tensor elements in deterministic row-major order.
Tolerance Quantization	Δ ≤ 1e-6 for floats, enforced via integer quantization of Δ×2²⁰.
Platform Parity	CI compares metric hash outputs across Windows and Linux.
Log Reproducibility	Markdown reports include fixed timestamp stubs (TEMPLATE_TIME placeholder) for diff testing.
Example: Coherence Metric Determinism
#[test]
fn test_coherence_determinism() {
    use chromatic_core::diagnostics::metrics::coherence_score;
    use chromatic_core::tensor::{SpectralTensor, ChromaticTensor};

    let spec = SpectralTensor::load("fixtures/tensors/spectral_reference.sten").unwrap();
    let chrom = ChromaticTensor::load("fixtures/tensors/chromatic_reference.cten").unwrap();

    let c1 = coherence_score(&chrom, &spec);
    let c2 = coherence_score(&chrom, &spec);
    assert!((c1 - c2).abs() < 1e-9, "Non-deterministic coherence score!");
}

Example: Energy Consistency Check
#[test]
fn test_energy_consistency() {
    use chromatic_core::tensor::{spectral_energy, mean_rgb};

    let spec = SpectralTensor::load("fixtures/tensors/spectral_reference.sten").unwrap();
    let chrom = ChromaticTensor::load("fixtures/tensors/chromatic_reference.cten").unwrap();

    let e_spec = spectral_energy(&spec);
    let e_rgb = mean_rgb(&chrom).iter().sum::<f32>() / 3.0;

    assert!((e_spec - e_rgb).abs() < 1e-3, "Energy mismatch exceeds tolerance");
}

Integration with Diagnostics Reports

The suite verifies that diagnostic reports are:

Bitwise-stable across multiple invocations.

Consistent with the baseline metrics in metrics_baseline.json.

Self-validating, i.e., the report’s embedded hash matches the computed metric CRC.

Output goes to:

experiments/results/diagnostics_validation_<timestamp>.json
experiments/results/diagnostics_report_diff.md

Fixture Details
File	Description
chromatic_reference.cten	Canonical RGB/HSL tensor (ground truth).
spectral_reference.sten	Corresponding spectral tensor.
metrics_baseline.json	Reference outputs for energy, coherence, ΔE, etc.
tolerances.toml	Defines Δ thresholds for each test category.
seeds.toml	RNG seeds for test reproducibility.
Regression Targets

Mean absolute Δ for all metrics ≤ 1e-6

Report hash identical across OS (CRC64)

Trend slopes reproducible within ±0.1 %

Dependencies
Dependency	Purpose
tensor	Base math operations and serialization
bridge	Color↔frequency reference conversions
dream	Synthetic replay loss data
meta	Chronicle and deterministic seed tracking
Status
Field	Value
Spec Version	1.0
Phase Alignment	Phase 6C – 6E, feeds Phase 7A reports
Determinism Level	Bit-exact metrics
Readiness	✅ Implementation Ready
Next Tests	dream_tests/ (synthetic generation validation)
