core/tests/bridge_tests/bridge-tests-spec.md
Purpose

The bridge_tests/ suite validates bidirectional integrity of the Chromatic–Spectral Bridge module (core/src/bridge/).
Its goal is to ensure all transformations:

Are numerically deterministic (bit-identical across runs).

Preserve round-trip accuracy (ΔColor ≤ 1e-3).

Maintain continuity across hue and spectral seams (no jumps at 0 ↔ 2π or fₘᵢₙ ↔ fₘₐₓ).

Respect the system’s HQAPR determinism policy.

Test Suite Layout
core/tests/bridge_tests/
├─ bridge-tests-spec.md          ← this spec
├─ test_hue_frequency.rs         ← hue↔frequency determinism
├─ test_roundtrip.rs             ← chromatic↔spectral↔chromatic accuracy
├─ test_amplitude_pan.rs         ← saturation→amplitude and luminance→pan mapping
├─ test_kernel_accumulation.rs   ← Gaussian accumulation stability
├─ test_inverse_decoding.rs      ← harmonic template decoding determinism
├─ test_appendixA_examples.rs    ← validation of Appendix A reference data
└─ fixtures/
   ├─ sample_colors.json         ← canonical test hues (12-category palette)
   ├─ spectral_reference.json    ← expected spectral outputs
   ├─ bridge_roundtrip.csv       ← logged ΔColor metrics
   └─ tolerances.toml            ← numeric tolerances for assertions

Core Tests
Test	Goal	Pass Criterion
test_hue_frequency.rs	Verify the hue→frequency mapping
𝑓
=
𝑓
𝑚
𝑖
𝑛
 
2
𝐻
/
(
2
𝜋
)
⋅
𝑜
𝑐
𝑡
𝑎
𝑣
𝑒
𝑠
f=f
min
	​

2
H/(2π)⋅octaves
 is continuous and reversible across 0 ↔ 2π.	Δf/f ≤ 1e-8; no discontinuity at wrap.
test_roundtrip.rs	Confirm full ChromaticTensor → SpectralTensor → ChromaticTensor round-trip accuracy.	Mean ΔRGB ≤ 1e-3 per channel.
test_amplitude_pan.rs	Check deterministic mapping
𝐴
=
𝑆
𝛾
,
  
𝑃
=
2
𝐿
−
1
A=S
γ
,P=2L−1 for monotonicity and continuity.	RMS ΔA ≤ 1e-6 across S sweep; P linearity error < 1e-8.
test_kernel_accumulation.rs	Ensure Gaussian kernel accumulation order independence.	Bitwise-identical spectra regardless of pixel order.
test_inverse_decoding.rs	Validate harmonic template matching recovers correct timbre index.	Correct index in ≥ 99.99 % cases; deterministic argmax output.
test_appendixA_examples.rs	Compare bridge output to Appendix A published constants.	All reference pairs within 0.1 % of spec values.
Deterministic Validation Rules
Rule	Enforcement
Fixed RNG Seed	Each test initializes RNG with recorded seed in fixtures/tolerances.toml.
Sorted Iteration	All arrays sorted before accumulation to remove nondeterministic order effects.
Quantized Assertion	Floating-point deltas quantized to fixed 1 LSB = 2⁻²⁰ for equality checks.
Cross-Platform	Tests run under Windows + Linux CI; hashes of serialized tensors compared.
Seam Visualization	Optional debug mode plots spiral trajectory for manual verification.
Example Test Snippet
#[test]
fn test_hue_frequency_determinism() {
    use chromatic_core::bridge::hue_to_frequency;
    use std::f32::consts::PI;

    let f_min = 110.0;
    let octaves = 7.0;

    for step in 0..=360 {
        let h = (step as f32).to_radians();
        let f1 = hue_to_frequency(h, f_min, octaves);
        let f2 = hue_to_frequency((h + 2.0 * PI) % (2.0 * PI), f_min, octaves);
        assert!((f1 - f2).abs() < 1e-8, "Discontinuity at hue seam");
    }
}

Output & Logging

Each test writes structured JSON to
experiments/results/bridge_validation_<timestamp>.json
containing metrics: ΔColor, ΔEnergy, ΔPhase, pass/fail status.

Aggregate reports automatically feed into the diagnostics metrics dashboard.

Integration Policy
Interface	Dependency
bridge module	Required (chromatic_core::bridge::*)
tensor module	For tensor creation & normalization
diagnostics	For coherence and energy metrics
meta/chronicle	For deterministic seed & context tracking
Status
Field	Value
Spec Version	1.0
Phase Alignment	Phase 7A – Appendix A validation
Determinism Level	Bit-exact (cross-platform)
Readiness	✅ Ready for implementation
Next Tests	diagnostics_tests/ (Phase 6 + 7 cross-checks)
