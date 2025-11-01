core/src/diagnostics/metrics/metrics-spec.md
Module Purpose

The Metrics Engine provides the deterministic, quantitative foundations for system introspection.
It calculates deltas, coherence scores, energy balances, and continuity indexes used across the entire Chromatic Core — ensuring precision, reproducibility, and numerical traceability for every cognitive cycle.

This subsystem underpins all diagnostics logic and feeds verifiable data to the Chronicle logs and the Continuity Control layer (Phases 6B–6D).

Scope
Function Group	Responsibility
Chromatic Metrics	Evaluate color-space differences (ΔH, ΔS, ΔL, ΔE).
Spectral Metrics	Compute FFT-based energy balance, spectral centroids, and drift.
Continuity Metrics	Quantify trend smoothness and coherence over temporal sequences.
Validation Metrics	Establish deterministic tolerances and confidence thresholds for testing.
Core Data Structures
pub struct ChromaticDelta {
    pub delta_h: f32,    // Hue difference in radians
    pub delta_s: f32,    // Saturation difference [0–1]
    pub delta_l: f32,    // Luminance difference [0–1]
    pub magnitude: f32,  // Euclidean magnitude
}

pub struct SpectralStats {
    pub energy_total: f32,
    pub energy_drift: f32,
    pub centroid: f32,       // Frequency-weighted mean
    pub coherence: f32,      // Phase alignment index
}

pub struct ContinuityMetrics {
    pub slope: f32,
    pub stdev: f32,
    pub oscillation_index: f32,
    pub trend_class: i8,     // Deterministic classification tag
}


All structs implement Serialize, Deserialize, and PartialEq for audit reproducibility.

Functions and APIs
Function	Signature	Description
compute_delta_hsl()	(a: &ChromaticTensor, b: &ChromaticTensor) -> ChromaticDelta	Computes precise hue/saturation/luminance deltas using seam-safe normalization.
spectral_energy_balance()	(spectrum: &SpectralTensor) -> SpectralStats	Calculates total energy, drift from baseline, and spectral centroid.
phase_coherence_index()	(tensor: &SpectralTensor) -> f32	Computes phase stability across frames; used in dream feedback.
continuity_from_history()	(records: &[CycleRecord]) -> ContinuityMetrics	Derives slope and oscillation metrics from Chronicle history.
validate_determinism()	(metrics_a: &MetricsSnapshot, metrics_b: &MetricsSnapshot) -> bool	Confirms bit-level equivalence across runs.
Mathematical Definitions
1. Color Delta (HSL Space)
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
−
𝐻
1
)
,
cos
⁡
(
𝐻
2
−
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
Δ
𝑆
=
∣
𝑆
2
−
𝑆
1
∣
,
Δ
𝐿
=
∣
𝐿
2
−
𝐿
1
∣
ΔS=∣S
2
	​

−S
1
	​

∣,ΔL=∣L
2
	​

−L
1
	​

∣
∣
Δ
𝐸
∣
=
(
Δ
𝐻
)
2
+
(
Δ
𝑆
)
2
+
(
Δ
𝐿
)
2
∣ΔE∣=
(ΔH)
2
+(ΔS)
2
+(ΔL)
2
	​

2. Spectral Energy Drift
𝐸
drift
=
10
log
⁡
10
∑
𝑓
𝑖
2
∑
𝑓
𝑖
,
0
2
E
drift
	​

=10log
10
	​

∑f
i,0
2
	​

∑f
i
2
	​

	​


Target:
∣
𝐸
drift
∣
<
0.5
 
dB
∣E
drift
	​

∣<0.5dB

3. Continuity Metrics
slope
=
𝑑
𝐸
𝑡
𝑑
𝑡
,
oscillation
=
𝜎
(
𝐸
𝑡
)
𝐸
𝑡
ˉ
slope=
dt
dE
t
	​

	​

,oscillation=
E
t
	​

ˉ
	​

σ(E
t
	​

)
	​


Trend classes: 0 = stable, 1 = positive growth, -1 = decay, 2 = oscillatory.

Deterministic Enforcement
Concern	Solution
Floating-point summation order	Fixed buffer iteration (ascending index)
Randomization	Seeded RNG (rng.seed(42)) for reproducibility
Seam instability in hue deltas	Modular arithmetic with atan2 wrapping
Drift accumulation	Rolling normalization vs baseline spectral energy
Integration Points
Consumer	Dependency Type	Notes
diagnostics/visual	read-only	Uses metrics for plots and spiral analysis.
core/meta/chronicle	write-only	Logs metrics snapshots and deltas.
core/continuity	bidirectional	Provides slope, stdev, and oscillation feedback for trend classification.
trainer/validator	read-only	Performs energy drift and color delta tests.
Validation & Testing
Test ID	Description	Pass Condition
T-M01	HSL delta round-trip	ΔE ≤ 1 × 10⁻³
T-M02	Energy conservation under FFT
T-M03	Phase coherence repeatability	Std ≤ 1 × 10⁻⁴ across 3 runs
T-M04	Continuity classification stability	Class unchanged under ±1 % noise
T-M05	Determinism regression	Bitwise equality of serialized snapshots
File Layout
metrics/
├─ metrics-spec.md          ← this spec
├─ hsl.rs                   ← color-space metrics
├─ spectral.rs              ← spectral energy & coherence
├─ continuity.rs            ← trend/oscillation metrics
├─ tests/
│   ├─ test_hsl_metrics.rs
│   ├─ test_spectral_metrics.rs
│   ├─ test_continuity_metrics.rs
└─ lut/
    ├─ spectral_norms.json
    ├─ hue_weights.tbl

Status
Field	Value
Spec Version	1.0
Phase Alignment	6C → 7B
Determinism Level	Bit-Exact
Readiness	✅ Approved for implementation
Next Module	diagnostics/visual
