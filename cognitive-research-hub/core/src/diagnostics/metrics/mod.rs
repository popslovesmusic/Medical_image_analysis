//! Deterministic diagnostics metrics for chromatic and spectral tensors.
//!
//! Specification references:
//! - `cognitive-research-hub/spec.md`
//! - `cognitive-research-hub/core/spec.md`
//! - `cognitive-research-hub/core/src/diagnostics/spec.md`
//! - `cognitive-research-hub/core/src/diagnostics/metrics/spec.md`

use crate::{
    tensor::{
        delta_hsl, rgb_to_hsl, spectral_centroid, spectral_energy, ChromaticTensor, SpectralTensor,
    },
    Fx,
};

const EPSILON: Fx = 1e-6;

/// Chromatic-space delta summary aggregated across tensors.
#[derive(Clone, Debug, PartialEq)]
pub struct ChromaticDelta {
    pub delta_h: Fx,
    pub delta_s: Fx,
    pub delta_l: Fx,
    pub magnitude: Fx,
}

/// Spectral statistics derived from deterministic accumulation.
#[derive(Clone, Debug, PartialEq)]
pub struct SpectralStats {
    pub energy_total: Fx,
    pub energy_drift: Fx,
    pub centroid: Fx,
    pub coherence: Fx,
}

/// Historical continuity metrics derived from cycle records.
#[derive(Clone, Debug, PartialEq)]
pub struct ContinuityMetrics {
    pub slope: Fx,
    pub stdev: Fx,
    pub oscillation_index: Fx,
    pub trend_class: i8,
}

/// Snapshot of metrics enabling deterministic comparisons across runs.
#[derive(Clone, Debug, PartialEq)]
pub struct MetricsSnapshot {
    pub chromatic: ChromaticDelta,
    pub spectral: SpectralStats,
    pub continuity: ContinuityMetrics,
}

/// Chronicle record capturing per-cycle diagnostics information.
#[derive(Clone, Debug, PartialEq)]
pub struct CycleRecord {
    pub cycle_id: u64,
    pub coherence: Fx,
    pub energy_total: Fx,
    pub delta_e: Fx,
}

/// Marker trait capturing deterministic serialization expectations without external dependencies.
pub trait Serialize {}

/// Marker trait capturing deterministic deserialization expectations without external dependencies.
pub trait Deserialize {}

impl Serialize for ChromaticDelta {}
impl Deserialize for ChromaticDelta {}
impl Serialize for SpectralStats {}
impl Deserialize for SpectralStats {}
impl Serialize for ContinuityMetrics {}
impl Deserialize for ContinuityMetrics {}
impl Serialize for MetricsSnapshot {}
impl Deserialize for MetricsSnapshot {}
impl Serialize for CycleRecord {}
impl Deserialize for CycleRecord {}

/// Computes seam-safe ΔHSL statistics aggregated over two chromatic tensors.
pub fn compute_delta_hsl(a: &ChromaticTensor, b: &ChromaticTensor) -> ChromaticDelta {
    assert_eq!(a.shape, b.shape, "tensor shapes must match");
    let cells = a.shape.cell_count().max(1) as Fx;
    let mut sum_dh = 0.0;
    let mut sum_ds = 0.0;
    let mut sum_dl = 0.0;

    for row in 0..a.shape.h {
        for col in 0..a.shape.w {
            let rgb_a = a.rgb_at(row, col);
            let rgb_b = b.rgb_at(row, col);
            let hsl_a = rgb_to_hsl(rgb_a[0], rgb_a[1], rgb_a[2]);
            let hsl_b = rgb_to_hsl(rgb_b[0], rgb_b[1], rgb_b[2]);
            let (dh, ds, dl) = delta_hsl(hsl_a, hsl_b);
            sum_dh += dh.abs();
            sum_ds += ds.abs();
            sum_dl += dl.abs();
        }
    }

    let delta_h = sum_dh / cells;
    let delta_s = sum_ds / cells;
    let delta_l = sum_dl / cells;
    let magnitude = (delta_h * delta_h + delta_s * delta_s + delta_l * delta_l).sqrt();

    ChromaticDelta {
        delta_h,
        delta_s,
        delta_l,
        magnitude,
    }
}

/// Computes spectral statistics including total energy, drift, centroid, and coherence.
pub fn spectral_energy_balance(spec: &SpectralTensor) -> SpectralStats {
    let total = spectral_energy(spec);
    let centroid = spectral_centroid(spec);
    let coherence = phase_coherence_index(spec);

    let half = spec.bins.len() / 2;
    let (low_energy, high_energy) = if half == 0 {
        (total, total)
    } else {
        let mut low = 0.0;
        let mut high = 0.0;
        for (idx, &amp) in spec.bins.iter().enumerate() {
            if idx < half {
                low += amp.abs();
            } else {
                high += amp.abs();
            }
        }
        (low.max(EPSILON), high.max(EPSILON))
    };
    let ratio = (high_energy / low_energy).max(EPSILON);
    let energy_drift = 10.0 * ratio.log10();

    SpectralStats {
        energy_total: total,
        energy_drift,
        centroid,
        coherence,
    }
}

/// Computes a normalized phase coherence index based on adjacent bin similarity.
pub fn phase_coherence_index(spec: &SpectralTensor) -> Fx {
    if spec.bins.len() < 2 {
        return 1.0;
    }
    let mut numerator = 0.0;
    let mut denominator = 0.0;
    for window in spec.bins.windows(2) {
        let a = window[0];
        let b = window[1];
        numerator += a * b;
        denominator += 0.5 * (a * a + b * b);
    }
    if denominator.abs() <= EPSILON {
        return 1.0;
    }
    (numerator / denominator).clamp(-1.0, 1.0)
}

/// Computes continuity metrics from a history of cycle records.
pub fn continuity_from_history(history: &[CycleRecord]) -> ContinuityMetrics {
    if history.is_empty() {
        return ContinuityMetrics {
            slope: 0.0,
            stdev: 0.0,
            oscillation_index: 0.0,
            trend_class: 0,
        };
    }

    let len = history.len() as Fx;
    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    for rec in history {
        sum_x += rec.cycle_id as Fx;
        sum_y += rec.coherence;
    }
    let mean_x = sum_x / len;
    let mean_y = sum_y / len;

    let mut num = 0.0;
    let mut den = 0.0;
    let mut variance = 0.0;
    let mut flips: Fx = 0.0;
    let mut prev_sign: Fx = 0.0;

    for window in history.windows(2) {
        let current = window[1].coherence;
        let previous = window[0].coherence;
        let diff = current - previous;
        let sign: Fx = if diff.abs() <= EPSILON {
            0.0
        } else if diff > 0.0 {
            1.0
        } else {
            -1.0
        };
        if prev_sign.abs() > EPSILON && sign.abs() > EPSILON && (sign + prev_sign).abs() < EPSILON {
            flips += 1.0;
        }
        if sign.abs() > EPSILON {
            prev_sign = sign;
        }
    }

    for rec in history {
        let dx = rec.cycle_id as Fx - mean_x;
        let dy = rec.coherence - mean_y;
        num += dx * dy;
        den += dx * dx;
        variance += dy * dy;
    }

    let slope = if den.abs() <= EPSILON { 0.0 } else { num / den };
    let stdev = (variance / len.max(1.0)).sqrt();
    let oscillation_index = if history.len() < 2 {
        0.0
    } else {
        flips / (history.len().saturating_sub(1) as Fx)
    };

    let trend_class = if slope > 0.01 {
        1
    } else if slope < -0.01 {
        -1
    } else {
        0
    };

    ContinuityMetrics {
        slope,
        stdev,
        oscillation_index,
        trend_class,
    }
}

/// Validates two metric snapshots are identical for deterministic verification.
pub fn validate_determinism(a: &MetricsSnapshot, b: &MetricsSnapshot) -> bool {
    a == b
}
