# Module: core/src/dream/
# Spec Version: 1.1 (Aligned with canonical roadmap and determinism mandate)

## Purpose

The Dream Module implements the imaginative inference layer of the Chromatic Core — a self-contained synthetic environment for generating hypothetical states, testing internal models, and providing rich data augmentation for the learner and diagnostics systems.

It functions as a closed-loop, **fully deterministic** simulator that models “creative reasoning through perturbation,” producing synthetic `ChromaticTensor` and `SpectralTensor` instances from baseline reality data.

## Scope
Subsystem	Responsibility
Dream Generator	Produces new `ChromaticTensor` instances through **controlled, deterministic** perturbations.
Dream Pool	Manages the storage, retrieval, and mutation of synthetic dreams (vector archives).
Dream Cycle Controller	Executes the iterative “dream–evaluate–refine” loop.
Retrieval Interface	Supplies high-coherence synthetic tensors to the **Trainer** or Planner.
Validation Layer	Measures coherence, diversity, and energy balance between dreams and ground truth.

## Core Data Structures

This module operates on the canonical tensor definitions provided by the Tensor module. All functions in this module MUST use the types defined in **`core/src/tensor/spec.md`**.

* **`tensor::ChromaticTensor`**: The canonical 2D array representation of chromatic data (RGB + Coherence).

* **`pub struct DreamEntry`**
    * `pub tensor: tensor::ChromaticTensor`
    * `pub epoch: u32`
    * `pub score: f32` (Fitness or relevance score)

* **`pub struct SimpleDreamPool`**
    * `pub entries: Vec<DreamEntry>`
    * `pub max_size: usize`
    * `pub coherence_threshold: f32`

## Functional Overview
Function	Signature	Description
`generate_dream()`	`(seed: &tensor::ChromaticTensor, noise_factor: f32, rng: &mut impl RngCore) -> tensor::ChromaticTensor`	Produces a perturbed version of the seed using **deterministic blending** and **pseudo-random harmonic noise** sampled from a LUT.
`evaluate_dream()`	`(dream: &tensor::ChromaticTensor, target: &tensor::ChromaticTensor) -> f32`	Computes a scalar fitness score based on ΔHSL distance and spectral coherence.
`add_dream_to_pool()`	`(pool: &mut SimpleDreamPool, dream: DreamEntry)`	Inserts or replaces pool entries based on coherence and diversity.
`retrieve_similar()`	`(query: &tensor::ChromaticTensor, limit: usize) -> Vec<&tensor::ChromaticTensor>`	Retrieves most similar tensors by color and coherence distance.
`dream_cycle()`	`(target: &tensor::ChromaticTensor, pool: &mut SimpleDreamPool, epochs: u32)`	Runs iterative synthesis, evaluation, and selection cycles.
`purge_stale_entries()`	`(pool: &mut SimpleDreamPool, max_age: u32)`	Removes low-fitness or outdated dream entries to maintain memory freshness.

## Algorithmic Summary

### 1. Deterministic Dream Generation
$D' = \alpha D + (1 - \alpha)N + \beta \cdot F(t)$

Where:
* $D$: current dream tensor
* $N$: **Noise tensor, deterministically sampled from a precomputed LUT** (e.g., `gaussian_noise.tbl`) using the seeded RNG.
* $F(t)$: spectral feedback modulation
* $\alpha, \beta$: deterministic blend coefficients set via config

### 2. Dream Evaluation
$\text{score}(D') = w_1 \cdot (1 - \Delta E_{HSL}) + w_2 \cdot C_{spec}$

Where:
* $\Delta E_{HSL}$: perceptual color delta from target
* $C_{spec}$: spectral coherence from FFT phase similarity

### 3. Dream Pool Management
Entries are kept sorted by score. New dreams replace the lowest-score entries when `max_size` is reached.

## Deterministic Guarantees

This module's output is fully deterministic. All sources of potential variation are strictly constrained.

* **RNG:** All pseudo-random operations MUST use an RNG (e.g., `rand_chacha::ChaCha8Rng`) that is explicitly **seeded at the start of each dream cycle** (e.g., `Rng::seed_from_u64(epoch_id)`). This seeded RNG is the *only* source for selecting noise parameters.
* **Noise Injection:** All "noise" (Gaussian, Perlin, etc.) MUST be sourced from **precomputed Look-Up Tables (LUTs)** stored in the `lut/` directory. The seeded RNG is used to select *indices* or *offsets* into these tables, not to generate new random values.
* **Floating Point:** All floating-point summations MUST use fixed-order accumulation (e.g., Neumaier summation) to prevent reordering errors.
* **Parallelism:** All parallel retrievals (e.g., with Rayon) MUST use a stable sort (`sort_by`) followed by a deterministic selection (`take`) to ensure identical results.
* **Storage:** The `SimpleDreamPool` MUST use a deterministic eviction policy (e.g., replace lowest `score`, then oldest `epoch`).

## Integration Points
| Module | Direction | Function |
| :--- | :--- | :--- |
| `tensor` | Input/Output | Provides base tensor structure and FFT operations |
| `metrics` | Input | Supplies coherence, delta, and drift metrics |
| **`trainer`** | Output | Supplies augmented data for **trainer** learning |
| `continuity` | Feedback | Feeds stability trend data for synthetic perturbation scheduling |
| `chronicle` | Log | Records each dream cycle’s metrics for replay |

## Validation Tests
Test	Description	Expected Result
`test_dream_generation_determinism`	Regenerate same dream under fixed seed	Bitwise identical tensors
`test_dream_pool_insertion`	Pool maintains sorted score order	Entries sorted descending
`test_retrieval_consistency`	Similar queries return same tensor subset	Order preserved
`test_dream_cycle_convergence`	Loss decreases over epochs	Monotonic or plateau trend
`test_energy_balance`	FFT energy balance stable	Drift < 0.5 dB

## File Layout
`dream/`
├─ `spec.md`                  ← this specification (was `dream-spec.md`)
├─ `generator.rs`                   ← synthetic perturbation logic
├─ `pool.rs`                        ← dream storage and retrieval
├─ `evaluator.rs`                   ← fitness scoring and coherence metrics
├─ `cycle.rs`                       ← orchestrates dream iteration loop
├─ `mod.rs`                         ← (Missing from original spec, needed for module)
├─ `config/`
│   ├─ `dream_params.toml`
│   └─ `noise_profiles.json`
├─ `tests/`
│   ├─ `test_dream_generation.rs`
│   ├─ `test_dream_pool.rs`
│   ├─ `test_retrieval.rs`
│   ├─ `test_cycle.rs`
└─ `lut/`
    ├─ `gaussian_noise.tbl`
    └─ `perlin_noise.tbl`

## Status
Field	Value
Spec Version	1.1
**Phase Alignment**	**Phase 4**
Dependencies	`tensor`, `metrics`, `chronicle`, `continuity`
Determinism Level	Full replay guaranteed
Readiness	✅ Approved for full implementation
Next Module	`core/src/meta/chronicle/`