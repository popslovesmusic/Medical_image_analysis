# Specification: tensor

**Module Path:** `cognitive-research-hub/core/src/tensor`

This placeholder spec.md file is automatically generated.
Replace this with the full specification for this module.

---
✅ Created by initialize_project_structure.py
core/src/tensor/tensor-spec.md
Purpose

The Tensor module provides the canonical, deterministic data structures and math primitives for the Chromatic Core:

ChromaticTensor — color-space representation (HSL/RGB views + per-cell metadata).

SpectralTensor — frequency-space representation (discrete bins + kernel widths).

Fixed-order, fixed-point accumulators and SIMD-safe operations for bit-stable results.

Color–space transforms, FFT hooks, gradients, and serialization with replay integrity.

It underpins bridge, diagnostics, dream, continuity, and chronicle.

Scope
Area	Responsibilities
Data Layouts	Row-major buffers with explicit strides; CPU cache-friendly; SIMD-addressable.
Color Math	RGB↔HSL transforms, seam-safe hue normalization, ΔHSL metrics.
Spectral Math	Discrete spectrum buffers, Gaussian kernel ops, energy/centroid/coherence.
Ops & Gradients	Mix/add/filter/mask, pointwise maps, reductions, deterministic gradients.
Quantization	Fixed-point accumulators for reproducible sums across threads/OS.
Serialization	Compact, endian-stable binary with checksums; JSON for debug.
Core Types
/// Scalar & index types
pub type Fx = f32;            // arithmetic scalar (compute)
pub type Qx = i32;            // fixed-point accumulator (storage/reduce)
pub type Ix = usize;          // indices

/// 2D tensor shape/stride
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Shape2D { pub h: Ix, pub w: Ix }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Stride2D { pub row: Ix, pub col: Ix }

/// ChromaticTensor: RGB base with HSL view; optional per-cell coherence
#[derive(Clone, Debug)]
pub struct ChromaticTensor {
    pub shape: Shape2D,
    pub stride: Stride2D,
    pub rgb: Vec<Fx>,           // len = h * w * 3, channel order = R,G,B
    pub coh: Option<Vec<Fx>>,   // optional coherence per cell [0..1]
}

/// SpectralTensor: discrete spectrum with optional bandwidth per bin
#[derive(Clone, Debug)]
pub struct SpectralTensor {
    pub bins: Vec<Fx>,          // amplitude/energy per bin
    pub sigma: Option<Vec<Fx>>, // kernel width per bin (if used)
    pub f_min: Fx,              // base frequency (Hz)
    pub f_res: Fx,              // Hz per bin (log or linear)
    pub log_scale: bool,        // true for log-frequency layout
}

Memory & Determinism

Row-major layout; RGB interleaved per pixel: (r,g,b) triplets.

Fixed-order loops: outer = rows, inner = cols, innermost = channels/bins.

Fixed-point reductions: convert f32 → Qx via shared scale S=2^20 (configurable), sum in i64, convert back at end. Prevents summation order drift.

SIMD-safe: operations use contiguous chunks; if SIMD enabled, still adhere to identical reduction order via block-local accumulators merged deterministically.

Color Utilities
pub fn rgb_to_hsl(r: Fx, g: Fx, b: Fx) -> (Fx, Fx, Fx); // H in [0,2π), S,L in [0,1]
pub fn hsl_to_rgb(h: Fx, s: Fx, l: Fx) -> (Fx, Fx, Fx);
pub fn normalize_hue(h: Fx) -> Fx;                      // wrap to [0,2π)
pub fn delta_hsl(a: (Fx,Fx,Fx), b: (Fx,Fx,Fx)) -> (Fx,Fx,Fx); // seam-safe ΔH, ΔS, ΔL


ΔH computed via:

Δ
𝐻
=
a
t
a
n
2
(
sin
⁡
(
𝐻
2
 ⁣
 ⁣
−
 ⁣
𝐻
1
)
,
cos
⁡
(
𝐻
2
 ⁣
 ⁣
−
 ⁣
𝐻
1
)
)
ΔH=atan2(sin(H
2
	​

−H
1
	​

),cos(H
2
	​

−H
1
	​

))
Spectral Utilities
/// Frequency at bin k (linear or log)
pub fn bin_freq(k: Ix, f_min: Fx, f_res: Fx, log_scale: bool) -> Fx;

/// Deterministic Gaussian add: A * exp(-(f - f0)^2 / (2σ^2)) into bins
pub fn add_gaussian_kernel(
    spec: &mut SpectralTensor, f0: Fx, sigma: Fx, amp: Fx
);

/// Energy & centroid
pub fn spectral_energy(spec: &SpectralTensor) -> Fx;
pub fn spectral_centroid(spec: &SpectralTensor) -> Fx; // Hz, weighted by bin freq


Determinism: kernel addition iterates bins in ascending order; amplitude accumulation uses fixed-point sum.

Tensor Operations
/// Pointwise linear blend: out = α * a + (1-α) * b
pub fn mix_rgb(out: &mut ChromaticTensor, a: &ChromaticTensor, b: &ChromaticTensor, alpha: Fx);

/// Pointwise add with clamp to [0,1]
pub fn add_rgb(out: &mut ChromaticTensor, a: &ChromaticTensor, b: &ChromaticTensor);

/// Masked inject: out = base; out[idx] = mix(base, inj, m[idx])
pub fn mask_inject(out: &mut ChromaticTensor, base: &ChromaticTensor, inj: &ChromaticTensor, mask: &[Fx]);

/// Map f over channels (e.g., gamma)
pub fn map_rgb_inplace(t: &mut ChromaticTensor, f: impl Fn(Fx)->Fx);

/// Reduce: mean RGB triplet
pub fn mean_rgb(t: &ChromaticTensor) -> [Fx;3];

/// Fixed-point sum over tensor (deterministic)
pub fn sum_fixed_rgb(t: &ChromaticTensor, scale: i32) -> [Qx;3];

Gradients (Deterministic)
pub struct GradRGB { pub dr: Fx, pub dg: Fx, pub db: Fx }

/// d( mix(a,b,α) ) / d inputs
pub fn grad_mix(a: (Fx,Fx,Fx), b: (Fx,Fx,Fx), alpha: Fx, d_out: (Fx,Fx,Fx))
  -> (GradRGB, GradRGB, Fx); // grads wrt a, b, α

/// d( HSL loss ) / d RGB via chained derivatives
pub fn grad_hsl_loss(a_rgb: (Fx,Fx,Fx), b_hsl: (Fx,Fx,Fx)) -> GradRGB;


All gradient reductions use fixed-point accumulation when batched.

FFT Hooks (Optional Feature fft)

The module does not implement an FFT itself; it defines stable adapters.

Any enabled backend must:

Use power-of-two window sizes with fixed window function (e.g., Hann LUT).

Export bit-stable magnitudes for the same inputs and window alignment.

#[cfg(feature="fft")]
pub trait FftBackend {
    fn forward(&self, samples: &[Fx]) -> Vec<(Fx, Fx)>; // (re, im)
}

#[cfg(feature="fft")]
pub fn magnitude_spectrum<B: FftBackend>(b: &B, x: &[Fx]) -> Vec<Fx>;

Serialization

Binary format .cten (Chromatic) and .sten (Spectral):

header: 4 bytes magic, 2 bytes version
meta:   shape/stride or freq params
data:   little-endian f32 arrays (rgb/coh or bins/sigma)
crc64:  payload checksum


APIs:

pub fn save_chromatic(t: &ChromaticTensor, path: &std::path::Path) -> std::io::Result<()>;
pub fn load_chromatic(path: &std::path::Path) -> std::io::Result<ChromaticTensor>;

pub fn save_spectral(s: &SpectralTensor, path: &std::path::Path) -> std::io::Result<()>;
pub fn load_spectral(path: &std::path::Path) -> std::io::Result<SpectralTensor>;

Integration Contracts
Module	Uses
bridge	normalize_hue, color transforms, spectral kernel ops
diagnostics/metrics	mean_rgb, ΔHSL, energy/centroid
diagnostics/visual	raw buffers for spirals/heatmaps
dream	mix/add/mask; fixed-point sums for scoring
continuity	tensor signatures [mean_rgb] for trend inputs
chronicle	(de)serialization with checksums
Deterministic Guarantees

Single source of truth for loop orders, strides, and accumulation.

Fixed-point reductions for all multi-term sums (configurable scale).

Stable seam handling for hue (wrap to [0, 2π)).

Optional SIMD never changes reduction order or final sums.

File Layout
tensor/
├─ tensor-spec.md            ← this spec
├─ layout.rs                 ← Shape/Stride, index helpers
├─ chromatic.rs              ← ChromaticTensor, RGB/HSL ops
├─ spectral.rs               ← SpectralTensor, kernels, energy
├─ ops.rs                    ← mix/add/mask/map/reduce
├─ grad.rs                   ← deterministic gradients
├─ quant.rs                  ← fixed-point accumulators
├─ io.rs                     ← serialization (.cten/.sten)
├─ fft.rs                    ← feature-gated FFT adapters (trait)
├─ tests/
│  ├─ test_layout.rs
│  ├─ test_color_ops.rs
│  ├─ test_spectral_ops.rs
│  ├─ test_quant.rs
│  ├─ test_io.rs
│  └─ test_determinism.rs
└─ lut/
   ├─ hann_window.tbl
   └─ quant_scale.tbl

Validation Tests
Test	Goal	Pass Criteria
test_color_roundtrip	RGB↔HSL↔RGB	max
test_hue_seam	seam continuity	no jump at 0↔2π (ΔH ≤ 1e-6)
test_kernel_add_determinism	spectral kernel addition	bit-identical bins across runs
test_quant_reduce	fixed-point sums	equals high-precision reference within 1 LSB
test_fft_adapter	magnitude spectrum stability	identical output given same backend
test_io_crc	serialization integrity	CRC64 matches; roundtrip exact
Status
Field	Value
Spec Version	1.0
Phase Alignment	6A–7C foundation
Determinism Level	Bit-exact (with fixed-point reductions)
Readiness	✅ Implementation Ready
Next	core/src/bridge (already specced) & diagnostics consumers