//! Dream module implementing deterministic synthetic tensor generation.
//!
//! Specification references:
//! - `cognitive-research-hub/spec.md`
//! - `cognitive-research-hub/core/spec.md`
//! - `cognitive-research-hub/core/src/dream/spec.md`

use std::cmp::Ordering;

use crate::{tensor::*, Fx};

/// Helper clamp that ensures the value resides within the unit interval.
fn clamp_unit(x: Fx) -> Fx {
    x.max(0.0).min(1.0)
}

fn coherence_map_from_rgb(shape: Shape2D, rgb: &[Fx]) -> Vec<Fx> {
    let cells = shape.cell_count();
    if cells == 0 {
        return Vec::new();
    }
    let mut sums = [0.0; 3];
    for chunk in rgb.chunks(3) {
        sums[0] += chunk[0];
        sums[1] += chunk[1];
        sums[2] += chunk[2];
    }
    let denom = cells as Fx;
    let mean = [sums[0] / denom, sums[1] / denom, sums[2] / denom];
    let mut map = Vec::with_capacity(cells);
    for chunk in rgb.chunks(3) {
        let delta =
            (chunk[0] - mean[0]).abs() + (chunk[1] - mean[1]).abs() + (chunk[2] - mean[2]).abs();
        let score = clamp_unit(1.0 - delta / 3.0);
        map.push(score);
    }
    map
}

fn coherence_metric_internal(shape: Shape2D, rgb: &[Fx], coh: Option<&[Fx]>) -> Fx {
    if let Some(map) = coh {
        if map.is_empty() {
            return 0.0;
        }
        return map.iter().fold(0.0, |acc, &v| acc + v) / map.len() as Fx;
    }
    let map = coherence_map_from_rgb(shape, rgb);
    if map.is_empty() {
        0.0
    } else {
        map.iter().fold(0.0, |acc, &v| acc + v) / map.len() as Fx
    }
}

fn cell_index(shape: Shape2D, row: usize, col: usize) -> usize {
    let row_offset = row.saturating_mul(shape.w);
    row_offset.saturating_add(col)
}

fn base_offset(shape: Shape2D, row: usize, col: usize) -> usize {
    cell_index(shape, row, col).saturating_mul(3)
}

fn synthesize_rgb(seed: &ChromaticTensor, noise: Fx) -> (Vec<Fx>, Vec<Fx>) {
    let shape = seed.shape;
    let noise_amp = clamp_unit(noise);
    let mean = mean_rgb(seed);
    let mut rgb = Vec::with_capacity(shape.rgb_len());
    for row in 0..shape.h {
        for col in 0..shape.w {
            let offset = base_offset(shape, row, col);
            let base_idx = offset;
            for channel in 0..3 {
                let idx = base_idx + channel;
                let source = seed.rgb[idx];
                let modulation =
                    ((row as Fx + 1.0) * (col as Fx + 1.0) * (channel as Fx + 1.0) * 0.173_205)
                        .sin();
                let blended = source * (1.0 - noise_amp)
                    + (mean[channel] + modulation * 0.1).clamp(0.0, 1.0) * noise_amp;
                rgb.push(clamp_unit(blended));
            }
        }
    }
    let coherence_map = coherence_map_from_rgb(shape, &rgb);
    (rgb, coherence_map)
}

fn hsl_distance_field(a: &ChromaticTensor, b: &ChromaticTensor) -> Fx {
    assert_eq!(a.shape, b.shape);
    let shape = a.shape;
    let mut accum = 0.0;
    let mut count = 0.0;
    for row in 0..shape.h {
        for col in 0..shape.w {
            let offset = base_offset(shape, row, col);
            let rgb_a = [a.rgb[offset], a.rgb[offset + 1], a.rgb[offset + 2]];
            let rgb_b = [b.rgb[offset], b.rgb[offset + 1], b.rgb[offset + 2]];
            let a_hsl = rgb_to_hsl(rgb_a[0], rgb_a[1], rgb_a[2]);
            let b_hsl = rgb_to_hsl(rgb_b[0], rgb_b[1], rgb_b[2]);
            let (dh, ds, dl) = delta_hsl(a_hsl, b_hsl);
            let delta = (dh * dh + ds * ds + dl * dl).sqrt();
            accum += delta;
            count += 1.0;
        }
    }
    if count <= f32::EPSILON {
        0.0
    } else {
        accum / count
    }
}

fn frequency_profile(t: &ChromaticTensor) -> Vec<Fx> {
    let mut profile = vec![0.0; t.shape.w.max(1)];
    for row in 0..t.shape.h {
        for col in 0..t.shape.w {
            let offset = base_offset(t.shape, row, col);
            let rgb = [t.rgb[offset], t.rgb[offset + 1], t.rgb[offset + 2]];
            let energy = (rgb[0] + rgb[1] + rgb[2]) / 3.0;
            profile[col] += energy;
        }
    }
    let norm = profile.iter().fold(0.0, |acc, &v| acc + v).max(1e-6);
    for value in &mut profile {
        *value /= norm;
    }
    profile
}

fn spectral_similarity(a: &[Fx], b: &[Fx]) -> Fx {
    let mut dot = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;
    for (x, y) in a.iter().zip(b.iter()) {
        dot += *x * *y;
        norm_a += x * x;
        norm_b += y * y;
    }
    if norm_a <= 1e-6 || norm_b <= 1e-6 {
        return 0.0;
    }
    let denom = norm_a.sqrt() * norm_b.sqrt();
    clamp_unit(dot / denom)
}

/// Returns the coherence metric for the supplied chromatic tensor.
pub fn coherence_metric(tensor: &ChromaticTensor) -> Fx {
    coherence_metric_internal(
        tensor.shape,
        &tensor.rgb,
        tensor.coh.as_ref().map(|v| v.as_slice()),
    )
}

/// Dream entry containing a generated tensor and associated metadata.
#[derive(Clone, Debug)]
pub struct DreamEntry {
    pub tensor: ChromaticTensor,
    pub epoch: u32,
    pub score: Fx,
    pub coherence: Fx,
}

impl DreamEntry {
    /// Constructs a new dream entry computing its coherence metric.
    pub fn new(tensor: ChromaticTensor, epoch: u32, score: Fx) -> Self {
        let coherence = coherence_metric(&tensor);
        Self {
            tensor,
            epoch,
            score,
            coherence,
        }
    }
}

/// Simple deterministic dream pool storing the top-scoring entries.
#[derive(Debug)]
pub struct SimpleDreamPool {
    entries: Vec<DreamEntry>,
    max_size: usize,
    coherence_threshold: Fx,
    epoch_cursor: u32,
}

impl SimpleDreamPool {
    /// Creates a new dream pool with capacity and coherence threshold.
    pub fn new(max_size: usize, coherence_threshold: Fx) -> Self {
        let capacity = max_size.max(1);
        Self {
            entries: Vec::with_capacity(capacity),
            max_size: capacity,
            coherence_threshold: clamp_unit(coherence_threshold),
            epoch_cursor: 0,
        }
    }

    /// Returns the number of entries stored.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true when the pool does not contain any entries.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns the coherence threshold enforced by the pool.
    pub fn coherence_threshold(&self) -> Fx {
        self.coherence_threshold
    }

    /// Returns the next epoch identifier and advances the counter.
    pub fn next_epoch(&mut self) -> u32 {
        let epoch = self.epoch_cursor;
        self.epoch_cursor = self.epoch_cursor.saturating_add(1);
        epoch
    }

    /// Provides immutable access to the stored entries.
    pub fn entries(&self) -> &[DreamEntry] {
        &self.entries
    }

    /// Returns the highest epoch recorded in the pool, if any.
    pub fn latest_epoch(&self) -> Option<u32> {
        self.entries.iter().map(|entry| entry.epoch).max()
    }

    fn push_or_replace(&mut self, entry: DreamEntry) {
        if self.entries.len() < self.max_size {
            self.entries.push(entry);
            return;
        }
        if let Some((idx, _)) = self
            .entries
            .iter()
            .enumerate()
            .min_by(|a, b| a.1.score.partial_cmp(&b.1.score).unwrap_or(Ordering::Equal))
        {
            if self.entries[idx].score < entry.score {
                self.entries[idx] = entry;
            }
        }
    }

    fn best_entry(&self) -> Option<&DreamEntry> {
        self.entries
            .iter()
            .max_by(|a, b| a.score.partial_cmp(&b.score).unwrap_or(Ordering::Equal))
    }
}

/// Generates a deterministic dream tensor from the seed using noise strength.
pub fn generate_dream(seed: &ChromaticTensor, noise: Fx) -> ChromaticTensor {
    let shape = seed.shape;
    let (rgb, coherence_map) = synthesize_rgb(seed, noise);
    ChromaticTensor::new(shape, rgb, Some(coherence_map))
}

/// Evaluates the dream tensor against the target returning a scalar score.
pub fn evaluate_dream(dream: &ChromaticTensor, target: &ChromaticTensor) -> Fx {
    let hsl_delta = hsl_distance_field(dream, target);
    let hsl_score = clamp_unit(1.0 - hsl_delta);
    let profile_dream = frequency_profile(dream);
    let profile_target = frequency_profile(target);
    let spectral_score = spectral_similarity(&profile_dream, &profile_target);
    0.6 * hsl_score + 0.4 * spectral_score
}

/// Inserts a dream entry into the pool if it satisfies the coherence threshold.
pub fn add_dream_to_pool(pool: &mut SimpleDreamPool, dream: DreamEntry) {
    if dream.coherence < pool.coherence_threshold {
        return;
    }
    pool.push_or_replace(dream);
}

/// Retrieves the most similar dream tensors to the query according to ΔHSL.
pub fn retrieve_similar<'a>(
    pool: &'a SimpleDreamPool,
    query: &ChromaticTensor,
    limit: usize,
) -> Vec<&'a ChromaticTensor> {
    if pool.is_empty() {
        return Vec::new();
    }
    let mut scored: Vec<(usize, Fx)> = pool
        .entries
        .iter()
        .enumerate()
        .map(|(idx, entry)| {
            let delta = hsl_distance_field(&entry.tensor, query);
            (idx, delta)
        })
        .collect();
    scored.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));
    let take = limit.min(scored.len());
    scored
        .into_iter()
        .take(take)
        .map(|(idx, _)| &pool.entries[idx].tensor)
        .collect()
}

/// Executes a deterministic dream cycle updating the pool with generated entries.
pub fn dream_cycle(target: &ChromaticTensor, pool: &mut SimpleDreamPool, epochs: u32) {
    if epochs == 0 {
        return;
    }
    let mut seed = target.clone();
    for step in 0..epochs {
        let noise = 0.05 + 0.02 * ((step % 7) as Fx);
        let dream = generate_dream(&seed, noise);
        let score = evaluate_dream(&dream, target);
        let epoch = pool.next_epoch();
        let entry = DreamEntry::new(dream, epoch, score);
        add_dream_to_pool(pool, entry);
        if let Some(best) = pool.best_entry() {
            seed = best.tensor.clone();
        }
    }
}

/// Purges entries whose age exceeds the provided threshold.
pub fn purge_stale_entries(pool: &mut SimpleDreamPool, max_age: u32) {
    if pool.is_empty() {
        return;
    }
    let latest_epoch = pool
        .entries
        .iter()
        .fold(0, |acc, entry| acc.max(entry.epoch));
    pool.entries.retain(|entry| {
        let age = latest_epoch.saturating_sub(entry.epoch);
        age <= max_age
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn uniform_tensor(value: Fx, shape: Shape2D) -> ChromaticTensor {
        let mut rgb = Vec::with_capacity(shape.rgb_len());
        for _ in 0..shape.cell_count() {
            rgb.extend_from_slice(&[value, value, value]);
        }
        ChromaticTensor::new(shape, rgb, None)
    }

    #[test]
    fn coherence_metric_uses_map_when_present() {
        let shape = Shape2D::new(1, 1);
        let tensor = ChromaticTensor::new(shape, vec![0.5, 0.5, 0.5], Some(vec![0.8]));
        assert!((coherence_metric(&tensor) - 0.8).abs() <= 1e-6);
    }

    #[test]
    fn generate_assigns_coherence_map() {
        let shape = Shape2D::new(2, 2);
        let seed = uniform_tensor(0.3, shape);
        let dream = generate_dream(&seed, 0.2);
        assert_eq!(dream.shape, seed.shape);
        assert!(dream.rgb.iter().all(|&v| (0.0..=1.0).contains(&v)));
        assert_eq!(dream.coh.as_ref().unwrap().len(), shape.cell_count());
    }

    #[test]
    fn evaluate_prefers_target_alignment() {
        let shape = Shape2D::new(2, 2);
        let target = uniform_tensor(0.4, shape);
        let mut variant = target.clone();
        crate::tensor::map_rgb_inplace(&mut variant, |v| clamp_unit(v + 0.2));
        let perfect = evaluate_dream(&target, &target);
        let perturbed = evaluate_dream(&variant, &target);
        assert!(perfect >= perturbed);
    }

    #[test]
    fn pool_enforces_coherence_threshold() {
        let shape = Shape2D::new(1, 1);
        let tensor_high = ChromaticTensor::new(shape, vec![0.4, 0.4, 0.4], Some(vec![0.9]));
        let tensor_low = ChromaticTensor::new(shape, vec![0.6, 0.6, 0.6], Some(vec![0.2]));
        let mut pool = SimpleDreamPool::new(2, 0.5);
        let high = DreamEntry::new(tensor_high, pool.next_epoch(), 0.8);
        let low = DreamEntry::new(tensor_low, pool.next_epoch(), 0.9);
        add_dream_to_pool(&mut pool, high);
        add_dream_to_pool(&mut pool, low);
        assert_eq!(pool.len(), 1);
    }

    #[test]
    fn retrieve_orders_by_similarity() {
        let shape = Shape2D::new(1, 2);
        let target = uniform_tensor(0.5, shape);
        let mut pool = SimpleDreamPool::new(3, 0.0);
        for value in [0.2f32, 0.4f32, 0.8f32] {
            let tensor = uniform_tensor(value, shape);
            let epoch = pool.next_epoch();
            let score = evaluate_dream(&tensor, &target);
            let entry = DreamEntry::new(tensor, epoch as u32, score);
            add_dream_to_pool(&mut pool, entry);
        }
        let retrieved = retrieve_similar(&pool, &target, 2);
        assert_eq!(retrieved.len(), 2);
        let first_score = evaluate_dream(retrieved[0], &target);
        let second_score = evaluate_dream(retrieved[1], &target);
        assert!(first_score >= second_score);
    }

    #[test]
    fn dream_cycle_populates_pool() {
        let shape = Shape2D::new(2, 2);
        let target = uniform_tensor(0.5, shape);
        let mut pool = SimpleDreamPool::new(5, 0.3);
        dream_cycle(&target, &mut pool, 5);
        assert!(pool.len() > 0);
    }

    #[test]
    fn purge_removes_stale_entries() {
        let shape = Shape2D::new(1, 1);
        let mut pool = SimpleDreamPool::new(3, 0.0);
        for epoch in [0u32, 1, 5] {
            let tensor = uniform_tensor(0.1 * (epoch as Fx + 1.0), shape);
            let score = evaluate_dream(&tensor, &tensor);
            let entry = DreamEntry::new(tensor, epoch, score);
            add_dream_to_pool(&mut pool, entry);
            pool.next_epoch();
        }
        purge_stale_entries(&mut pool, 1);
        let latest = pool.latest_epoch().unwrap_or(0);
        assert!(pool
            .entries()
            .iter()
            .all(|e| latest.saturating_sub(e.epoch) <= 1));
    }
}
