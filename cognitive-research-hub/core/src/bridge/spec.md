# Module: core/src/bridge/
# Spec Version: 1.1 (Aligned with canonical roadmap and types)

## Purpose

The Bridge module forms the bidirectional link between chromatic color-space tensors and spectral frequency-space tensors. It ensures deterministic reversibility of mappings (ΔColor ≤ 10⁻³) across the system’s `ChromaticTensor` ↔ `SpectralTensor` conversions.

It is a **Phase 2** implementation module providing the canonical “Hue–Frequency Bridge,” designed for use in all spectral encoding, decoding, and round-trip verification stages.

## Scope
Role	Responsibility
Encoder	Converts color-space hue, saturation, and luminance (HSL) values from a `ChromaticTensor` to spectral parameters (f, σ, A) in a `SpectralTensor`.
Decoder	Performs deterministic peak identification and deconvolution of overlapping spectral kernels from a `SpectralTensor` to recover HSL values.
Normalizer	Handles seam correction (0 ↔ 2π wrapping) and floating-point normalization within deterministic thresholds.
Utilities	Provides helper functions for normalization, seam-weight logging, and LUT access.

## Core Structures

This module operates on the canonical tensor definitions provided by the Tensor module. All functions in this module MUST use the types defined in **`core/src/tensor/spec.md`**.

* **`tensor::ChromaticTensor`**: The canonical 2D array representation of chromatic data (RGB + Coherence).
* **`tensor::SpectralTensor`**: The canonical representation of data in the frequency domain.

## Bridge Functions
Function	Signature	Description
encode_to_spectral()	`(&tensor::ChromaticTensor) -> tensor::SpectralTensor`	Computes frequency from hue and maps saturation → amplitude, luminance → σ.
decode_to_chromatic()	`(&tensor::SpectralTensor) -> tensor::ChromaticTensor`	Recovers hue using log-based inverse, reconstructs saturation from energy, and computes luminance from spectral width.
normalize_hue()	`(f32) -> f32`	Applies modular normalization to maintain continuity near the hue seam.
record_seam_weights()	`(f32, f32) -> f32`	Computes and logs relative weighting across the hue seam for round-trip consistency.
validate_round_trip()	`(&tensor::ChromaticTensor) -> bool`	Performs encode/decode loop and checks ΔColor ≤ 1e-3.

## Mathematical Formulation
Hue–Frequency Mapping
$f = f_{\min} \cdot 2^{\frac{H}{2\pi} \cdot \text{octaves}}$

Inverse Recovery
$H' = \frac{2\pi \cdot \log_{2}(f / f_{\min})}{\text{octaves}}, H = (H' \bmod 2\pi + 2\pi) \bmod 2\pi$

### Deterministic Peak Recovery

To guarantee reproducible inverse mappings when multiple Gaussian kernels overlap:

1.  Compute local maxima via quadratic interpolation over each spectral bin triplet.
2.  Cross-correlate the observed spectral window with all 12 fixed timbre templates (T₀…T₁₁).
3.  Select the highest correlation match as the reconstructed frequency $f_n$.
4.  Estimate amplitude as normalized total kernel energy.
5.  Compute saturation and luminance using the amplitude-width relationships.

## Phase 2 Integration Hooks
Interface	Purpose
`bridge::encode_to_spectral()`	Used by `trainer::model` during forward projection
`bridge::decode_to_chromatic()`	Used by `validator::spectral_checker`
`bridge::validate_round_trip()`	Used in unit tests for continuity audits
`bridge::normalize_hue()`	Exported globally for other color modules

## Validation
Test	Description	Expected Result
`test_hue_frequency_roundtrip`	Encode → decode → compare	ΔH < 1e-3 radians
`test_overlap_resolution`	Multi-kernel spectral mix	Correct dominant kernel recovered
`test_seam_continuity`	Hue near 0 ↔ 2π	No wrap discontinuity
`test_energy_conservation`	Amplitude vs. spectral energy	Energy deviation < 0.1 %

## Dependencies

* `tensor/` module (for `ChromaticTensor`, `SpectralTensor`, and math)
* `diagnostics/metrics` (for validation and precision reporting)
* `meta/chronicle` (for append-only logging of round-trip deltas)

## Status
Field	Value
Spec Version	1.1
Author	System Architect
**Phase**	**2**
Implementation Readiness	✅ Complete
Integration Point	Verified
**Next Phase**	**Phase 3 – Diagnostics and Continuity**