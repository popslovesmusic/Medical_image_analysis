//! Integration tests for diagnostics metrics and continuity validation.
//!
//! Specification references:
//! - `cognitive-research-hub/spec.md`
//! - `cognitive-research-hub/core/spec.md`
//! - `cognitive-research-hub/core/tests/spec.md`
//! - `cognitive-research-hub/core/tests/diagnostics_tests/spec.md`

use chromatic_core::diagnostics::{self, metrics};
use chromatic_core::tensor::{rgb_to_hsl, ChromaticTensor, Shape2D, SpectralTensor};

fn build_chromatic(fill: [f32; 3], delta: [f32; 3]) -> (ChromaticTensor, ChromaticTensor) {
    let shape = Shape2D::new(2, 2);
    let mut base = Vec::new();
    let mut altered = Vec::new();
    for _ in 0..shape.cell_count() {
        base.extend_from_slice(&fill);
        altered.extend_from_slice(&[
            (fill[0] + delta[0]).clamp(0.0, 1.0),
            (fill[1] + delta[1]).clamp(0.0, 1.0),
            (fill[2] + delta[2]).clamp(0.0, 1.0),
        ]);
    }
    let chrom_a = ChromaticTensor::new(shape, base, None);
    let chrom_b = ChromaticTensor::new(shape, altered, None);
    (chrom_a, chrom_b)
}

fn build_spectral() -> SpectralTensor {
    SpectralTensor::new(vec![1.0, 0.8, 0.3, 0.2], None, 20.0, 5.0, false)
}

#[test]
fn delta_hsl_matches_expected_average() {
    let (a, b) = build_chromatic([0.2, 0.3, 0.4], [0.05, -0.02, 0.03]);
    let delta = metrics::compute_delta_hsl(&a, &b);
    assert!(delta.delta_h >= 0.0 && delta.delta_s >= 0.0 && delta.delta_l >= 0.0);
    assert!(delta.magnitude > 0.0);

    // Validate one sample explicitly for deterministic expectation.
    let sample_a = rgb_to_hsl(0.2, 0.3, 0.4);
    let sample_b = rgb_to_hsl(0.25, 0.28, 0.43);
    let (dh, ds, dl) = chromatic_core::tensor::delta_hsl(sample_a, sample_b);
    let expected_mag = ((dh.abs()).powi(2) + ds.abs().powi(2) + dl.abs().powi(2)).sqrt();
    assert!((delta.magnitude - expected_mag).abs() < 1e-3);
}

#[test]
fn spectral_stats_capture_energy_and_coherence() {
    let spec = build_spectral();
    let stats = metrics::spectral_energy_balance(&spec);
    assert!((stats.energy_total - 2.3).abs() < 1e-6);
    assert!(stats.centroid > 20.0);
    assert!(stats.coherence <= 1.0 && stats.coherence >= -1.0);
}

#[test]
fn phase_coherence_is_high_for_smooth_bins() {
    let spec = build_spectral();
    let coherence = metrics::phase_coherence_index(&spec);
    assert!(coherence > 0.7);
}

#[test]
fn continuity_metrics_detect_trend_and_oscillation() {
    let history = vec![
        metrics::CycleRecord {
            cycle_id: 0,
            coherence: 0.60,
            energy_total: 1.5,
            delta_e: 0.01,
        },
        metrics::CycleRecord {
            cycle_id: 1,
            coherence: 0.62,
            energy_total: 1.4,
            delta_e: 0.02,
        },
        metrics::CycleRecord {
            cycle_id: 2,
            coherence: 0.64,
            energy_total: 1.3,
            delta_e: 0.02,
        },
        metrics::CycleRecord {
            cycle_id: 3,
            coherence: 0.67,
            energy_total: 1.2,
            delta_e: 0.03,
        },
    ];
    let metrics = metrics::continuity_from_history(&history);
    assert!(metrics.slope > 0.0);
    assert!(metrics.stdev > 0.0);
    assert!(metrics.oscillation_index <= 0.5);
    assert_eq!(metrics.trend_class, 1);
    assert!(diagnostics::validate_continuity(&history));
}

#[test]
fn continuity_validation_flags_oscillation() {
    let history = vec![
        metrics::CycleRecord {
            cycle_id: 0,
            coherence: 0.6,
            energy_total: 1.2,
            delta_e: 0.01,
        },
        metrics::CycleRecord {
            cycle_id: 1,
            coherence: 0.8,
            energy_total: 1.2,
            delta_e: 0.02,
        },
        metrics::CycleRecord {
            cycle_id: 2,
            coherence: 0.4,
            energy_total: 1.1,
            delta_e: 0.02,
        },
        metrics::CycleRecord {
            cycle_id: 3,
            coherence: 0.9,
            energy_total: 1.0,
            delta_e: 0.03,
        },
    ];
    assert!(!diagnostics::validate_continuity(&history));
}

#[test]
fn metrics_snapshot_determinism_checks_equality() {
    let continuity = metrics::ContinuityMetrics {
        slope: 0.1,
        stdev: 0.02,
        oscillation_index: 0.1,
        trend_class: 1,
    };
    let chroma = metrics::ChromaticDelta {
        delta_h: 0.01,
        delta_s: 0.02,
        delta_l: 0.01,
        magnitude: 0.03,
    };
    let spectral = metrics::SpectralStats {
        energy_total: 2.0,
        energy_drift: 0.1,
        centroid: 25.0,
        coherence: 0.9,
    };
    let snap_a = metrics::MetricsSnapshot {
        chromatic: chroma.clone(),
        spectral: spectral.clone(),
        continuity: continuity.clone(),
    };
    let snap_b = metrics::MetricsSnapshot {
        chromatic: chroma,
        spectral,
        continuity,
    };
    assert!(metrics::validate_determinism(&snap_a, &snap_b));
}
