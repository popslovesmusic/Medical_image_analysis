core/src/diagnostics/continuity/continuity-spec.md
Purpose

The Continuity Module provides deterministic temporal reasoning over diagnostic metrics.
It converts the raw numerical history (from metrics/) into trend models, stability classifications, and predictive actions.
This forms the analytical backbone for the Continuity Control Layer (Phases 6C → 6E), allowing the system to anticipate instability and react before failure.

Scope
Function Group	Responsibility
Trend Modeling	Extract smooth, low-noise trajectories from historical deltas.
Oscillation Detection	Identify periodic instability patterns in spectral or chromatic data.
Stability Classification	Convert trends into discrete, deterministic state categories.
Predictive Thresholding	Trigger deterministic alerts or corrective plans when thresholds are breached.
Core Structures
pub struct TrendModel {
    pub short_slope: f32,          // Instantaneous rate of change
    pub long_slope: f32,           // Smoothed rate over N cycles
    pub stdev: f32,                // Standard deviation of energy or color drift
    pub oscillation_score: f32,    // Normalized oscillation intensity
    pub class_id: i8,              // Deterministic trend classification
}

pub struct TemporalAction {
    pub action_code: u8,           // Encoded decision (0=Hold, 1=Reset, 2=Adjust, 3=Relearn)
    pub confidence: f32,           // Deterministic confidence metric [0–1]
}

Functions
Function	Signature	Description
analyze_trends()	(records: &[CycleRecord]) -> TrendModel	Performs linear and exponential regression over Chronicle data.
detect_oscillations()	(series: &[f32]) -> f32	Computes normalized FFT-based oscillation score.
classify_stability()	(trend: &TrendModel) -> i8	Deterministic rule-based classifier producing stable, decay, growth, or oscillatory tags.
plan_temporal_action()	(trend: &TrendModel) -> Option<TemporalAction>	Maps classification and slope data to deterministic corrective actions.
verify_trend_determinism()	(a: &TrendModel, b: &TrendModel) -> bool	Ensures identical classification across re-runs.
Classification Rules
Class ID	Category	Condition
0	Stable	
1	Improvement	slope > ε₁ and oscillation < ε₃
-1	Degradation	slope < −ε₁ and stdev > ε₂
2	Oscillatory	oscillation ≥ ε₃
3	Divergent	

Default tolerance set:
ε₁ = 0.01, ε₂ = 0.005, ε₃ = 0.02, ε₄ = 0.05, ε₅ = 0.03.

Deterministic Enforcement
Concern	Enforcement
Regression reproducibility	Fixed window size, static least-squares matrix inversion
FFT oscillation analysis	Fixed radix-2 window, ordered summation
Classification stability	Integer rounding at defined thresholds
Temporal alignment	Chronological sort enforced before analysis
Integration Points
Module	Role	Purpose
metrics	Input	Provides slope, stdev, and oscillation primitives
chronicle	Input	Supplies cycle history
planner (6E)	Output	Uses TemporalAction to adjust learning parameters
visual	Output	For rendering stability timelines and trend plots
Validation Tests
Test	Description	Pass Criterion
test_trend_consistency	Identical trend output for repeated runs	Bitwise equal struct
test_oscillation_detection	Detect known synthetic oscillations	Score ≥ 0.95 on known sinusoid
test_classification_rules	Verify correct mapping for all class types	Matches rule table
test_action_mapping	Correct TemporalAction code generation	ActionCode stable across runs
test_threshold_determinism	Regression under slight noise	ClassID unchanged
File Layout
continuity/
├─ continuity-spec.md        ← this document
├─ trend.rs                  ← regression + slope models
├─ oscillation.rs            ← FFT oscillation analysis
├─ classifier.rs             ← rule-based trend classifier
├─ planner.rs                ← TemporalAction generation
├─ tests/
│   ├─ test_trend.rs
│   ├─ test_oscillation.rs
│   ├─ test_classification.rs
│   ├─ test_planner.rs
└─ lut/
    ├─ thresholds.tbl
    └─ smoothing_window.tbl

Status
Field	Value
Spec Version	1.0
Phase Alignment	6D → 7C
Dependencies	metrics, chronicle
Readiness	✅ Ready for integration
Next Module	core/src/meta/chronicle