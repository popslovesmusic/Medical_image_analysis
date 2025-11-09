//! Integration tests for the deterministic dream module.
//!
//! Specification references:
//! - `cognitive-research-hub/spec.md`
//! - `cognitive-research-hub/core/spec.md`
//! - `cognitive-research-hub/core/src/dream/spec.md`

use chromatic_core::{
    dream::{
        add_dream_to_pool, dream_cycle, evaluate_dream, generate_dream, purge_stale_entries,
        retrieve_similar, DreamEntry, SimpleDreamPool,
    },
    tensor::{map_rgb_inplace, ChromaticTensor, Shape2D},
    Fx,
};

fn uniform_tensor(value: Fx, shape: Shape2D) -> ChromaticTensor {
    let mut rgb = Vec::with_capacity(shape.rgb_len());
    for _ in 0..shape.cell_count() {
        rgb.extend_from_slice(&[value, value, value]);
    }
    ChromaticTensor::new(shape, rgb, None)
}

#[test]
fn dream_cycle_populates_high_coherence_entries() {
    let shape = Shape2D::new(3, 3);
    let target = uniform_tensor(0.45, shape);
    let mut pool = SimpleDreamPool::new(6, 0.4);
    dream_cycle(&target, &mut pool, 8);
    assert!(pool.len() > 0, "expected entries in pool");
    for entry in pool.entries() {
        assert!(
            entry.coherence >= pool.coherence_threshold(),
            "entry coherence below threshold"
        );
    }
}

#[test]
fn retrieve_similar_respects_limit_and_ordering() {
    let shape = Shape2D::new(2, 3);
    let target = uniform_tensor(0.35, shape);
    let mut pool = SimpleDreamPool::new(8, 0.2);
    dream_cycle(&target, &mut pool, 6);

    let mut extra = generate_dream(&target, 0.3);
    map_rgb_inplace(&mut extra, |v| (v + 0.1).min(1.0));
    let epoch = pool.next_epoch();
    let score = evaluate_dream(&extra, &target);
    let entry = DreamEntry::new(extra, epoch, score);
    add_dream_to_pool(&mut pool, entry);

    let retrieved = retrieve_similar(&pool, &target, 3);
    assert!(retrieved.len() <= 3);
    if retrieved.len() >= 2 {
        let first = evaluate_dream(retrieved[0], &target);
        let second = evaluate_dream(retrieved[1], &target);
        assert!(first >= second - 1e-6);
    }
}

#[test]
fn purge_stale_entries_keeps_recent_epochs() {
    let shape = Shape2D::new(2, 2);
    let target = uniform_tensor(0.5, shape);
    let mut pool = SimpleDreamPool::new(5, 0.0);
    dream_cycle(&target, &mut pool, 4);

    let latest = pool.latest_epoch().unwrap_or(0);
    purge_stale_entries(&mut pool, 0);
    for entry in pool.entries() {
        assert_eq!(entry.epoch, latest);
    }
}
