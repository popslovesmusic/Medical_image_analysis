core/tests/tensor_tests/tensor-tests-spec.md
Purpose

The Tensor Tests suite validates the foundational numerical layer of the Chromatic Core — the deterministic tensor arithmetic that underlies bridge, dream, diagnostics, and meta modules.

This suite ensures that:

All tensor operations (addition, dot product, convolution, normalization, quantization) produce bit-identical results across runs and OS environments.

The ChromaticTensor and SpectralTensor maintain energy conservation and reversibility guarantees.

Quantization and serialization yield exact round-trip consistency.

Directory Layout
core/tests/tensor_tests/
├─ tensor-tests-spec.md              ← this spec
├─ test_tensor_math.rs               ← core arithmetic and quantization
├─ test_chromatic_tensor.rs          ← RGB/HSL encoding and normalization
├─ test_spectral_tensor.rs           ← frequency-space accumulation and FFT
├─ test_tensor_roundtrip.rs          ← serialization and checksum verification
├─ test_fixed_accumulator.rs         ← fixed-point order-invariant summation
├─ test_cross_platform_hash.rs       ← platform parity hash validation
└─ fixtures/
   ├─ chromatic_reference.cten
   ├─ spectral_reference.sten
   ├─ mixed_signals.npy
   ├─ tolerances.toml
   └─ baseline_hashes.json

Core Tests
Test	Objective	Key Modules
test_tensor_math.rs	Validate element-wise ops, dot products, normalization, and scalar ops.	tensor::ops, tensor::math
test_chromatic_tensor.rs	Confirm RGB→HSL and normalization stability across wrap-around.	tensor::chromatic
test_spectral_tensor.rs	Test deterministic accumulation of Gaussian kernels and spectral FFT.	tensor::spectral, bridge
test_tensor_roundtrip.rs	Ensure serialization→deserialization preserves bit identity.	tensor::io
test_fixed_accumulator.rs	Validate stable, order-independent summation of floating-point terms.	tensor::quantized, diagnostics
test_cross_platform_hash.rs	Check hash consistency between Windows and Linux builds.	tensor::hash, meta::chronicle
Deterministic Constraints
Rule	Description
Quantized Reduction	All multi-element sums quantized to fixed-point before final accumulation (Q24.8 format default).
Canonical Ordering	Tensor indices traversed in row-major order — parallel execution prohibited during validation tests.
Hash Consistency	Each tensor produces CRC64 hash logged in baseline_hashes.json for OS parity checks.
Exact Serialization	Binary .cten and .sten files must reproduce byte-identical re-loads.
Tolerance Enforcement	All Δ checks defined in fixtures/tolerances.toml (typically 1e-7).
Example: Fixed Accumulator Stability
#[test]
fn test_fixed_accumulator_stability() {
    use chromatic_core::tensor::quantized::FixedAccumulator;

    let values = vec![0.1_f32, 0.2, 0.3, 0.4];
    let acc1 = FixedAccumulator::sum(&values);
    let acc2 = FixedAccumulator::sum(&values.iter().rev().cloned().collect::<Vec<_>>());

    assert_eq!(acc1.to_bits(), acc2.to_bits(), "Order-dependent accumulation detected");
}

Example: Tensor Serialization Round-Trip
#[test]
fn test_tensor_roundtrip() {
    use chromatic_core::tensor::{ChromaticTensor, SpectralTensor};

    let original = ChromaticTensor::from_rgb(&[0.2, 0.5, 0.8]);
    original.save("fixtures/tmp.cten").unwrap();
    let reloaded = ChromaticTensor::load("fixtures/tmp.cten").unwrap();

    assert_eq!(original.hash(), reloaded.hash(), "Tensor serialization not reversible");
}

Integration Rules
Component	Usage
tensor	Core validation target
bridge	Provides frequency↔hue test dependencies
diagnostics	Supplies comparison utilities and Δ calculators
meta	Chronicles tensor versioning and hash history
Output and Logging

All tests write summaries to:

experiments/results/tensor_validation_<timestamp>.json
experiments/results/tensor_energy_balance.md


Logs include:

Energy balance (ΔE)

Hash equivalence results

Quantization step deltas

System info (endianness, float mode, compiler version)

Baseline Data (fixtures)
File	Description
chromatic_reference.cten	Known-good color tensor (used in bridge tests).
spectral_reference.sten	Corresponding spectral tensor.
mixed_signals.npy	Cross-domain synthetic test data.
tolerances.toml	Numeric tolerance map per metric.
baseline_hashes.json	Expected hash fingerprints for tensor files.
Pass Criteria
Metric	Threshold
Mean ΔE across tensors	≤ 1e-3
Accumulator Δ across orders	0 (bitwise identical)
Serialization Δhash	0
FFT spectral energy drift	≤ 0.01 dB
Cross-platform hash difference	0
Status
Field	Value
Spec Version	1.0
Phase Alignment	Phases 6A–7A foundation
Determinism Level	Full (bit-exact reproducibility)
Readiness	✅ Ready for implementation
Next Tests	bridge_tests/ (already linked) and dream_tests/ (up next)
