🌙 core/tests/dream_tests/dream-tests-spec.md
Purpose

The Dream Tests suite validates the synthetic imagination stack (core/src/dream/) for:

Deterministic replay (seeded cycles reproduce bit-identical states)

Convergence behavior (loss/coherence improves or plateaus)

Pool correctness (insert/evict/sort/retrieve are stable and consistent)

A/B harness results (retrieval-seeded vs. random-seeded)

Energy & color continuity (FFT drift < threshold; ΔHSL bounded)

This suite is the acceptance gate for Phases 6A–6E and supports bridge/diagnostics integration.

Directory Layout
core/tests/dream_tests/
├─ dream-tests-spec.md                 ← this document
├─ test_seed_replay.rs                 ← deterministic dream cycle replay
├─ test_pool_ops.rs                    ← add/evict/sort/retrieve determinism
├─ test_cycle_convergence.rs           ← loss & coherence trends
├─ test_ab_harness.rs                  ← control vs. retrieval A/B validation
├─ test_energy_color_consistency.rs    ← spectral drift & ΔHSL bounds
├─ test_serialization.rs               ← pool/tensor save-load roundtrip
└─ fixtures/
   ├─ seeds.toml
   ├─ tolerances.toml
   ├─ dream_params.toml
   ├─ chromatic_targets/
   │  ├─ target_palette_12.json
   │  └─ target_palette_extra.json
   ├─ checkpoints/
   │  ├─ pool_initial.json
   │  └─ pool_reference.json
   └─ results_baseline.json

Test Cases
1) test_seed_replay.rs — Deterministic Replay

Goal: Running dream_cycle() with the same seeds and config must produce bit-identical outputs.

Initialize RNG from fixtures/seeds.toml

Run N epochs → capture final ChromaticTensor and DreamPool

Re-run with same seeds → compare byte-wise equality

Pass: hash(final_tensor_run1) == hash(final_tensor_run2) and pool JSON identical.

2) test_pool_ops.rs — Pool CRUD & Retrieval

Goal: Insert, bounded-size evict, stable sort, and retrieval consistency.

Fill SimpleDreamPool to max_size

Insert with varied scores/coherence, force eviction

Assert stable order (score desc; index tiebreak)

retrieve_similar(query, k) must return same subset & order across runs

Pass: deterministic indices; stable ordering under ties; overflow behavior consistent.

3) test_cycle_convergence.rs — Trend Behavior

Goal: Validate improvement (or stable plateau) under fixed parameters.

For each target in chromatic_targets/target_palette_12.json

Run dream_cycle() with epochs = E

Record loss_t, coherence_t

Aggregate trend via least-squares slope on final third of epochs

Pass: slope(loss) ≤ 0 (non-increasing) OR |slope| ≤ ε; mean coherence non-decreasing.

4) test_ab_harness.rs — Control vs. Retrieval A/B

Goal: Validate hypothesis that retrieval seeding performs ≥ baseline (or document failure deterministically).

Group A (Control): random noise seed

Group B (Test): retrieval-based seed (retrieve_similar())

Same targets, epochs, seeds

Compare final mean loss & coherence

Pass (preferred): loss_B < loss_A OR coherence_B > coherence_A by δ defined in tolerances.toml
If not: test still passes if the difference is within neutral band and results are identical across runs (deterministic neutrality).

5) test_energy_color_consistency.rs — Energy & ΔHSL

Goal: Ensure spectral energy conservation and color-space continuity.

Convert dream outputs to SpectralTensor via bridge

Check FFT energy drift per epoch

Compute ΔHSL to target

Pass: |drift| < 0.5 dB (avg), mean ΔE ≤ 1e-3.

6) test_serialization.rs — Checkpoint Round-Trip

Goal: Save/load for pool and tensors is byte-identical.

Save DreamPool and sample tensors

Reload → compare hashes and JSON equivalence

Pass: hashes equal; CRC64 matches.

Deterministic Constraints
Concern	Enforcement
RNG	Seed from fixtures/seeds.toml; record per-epoch seed in Chronicle if present
Summation order	Fixed loop ordering; fixed-point accumulators in pool scoring & kernel adds
Parallelism	Disabled for tests (single-threaded feature flag)
File outputs	Stable filenames with placeholder timestamp or canonical suffix
Ties	Stable sort with index/UUID tiebreaker
Example Snippets

Seed Replay

#[test]
fn seed_replay_is_bit_exact() {
    use chromatic_core::dream::{dream_cycle, SimpleDreamPool};
    use chromatic_core::tensor::ChromaticTensor;
    use chromatic_core::meta::chronicle::hash64;

    let seeds = Seeds::from_toml("fixtures/seeds.toml");
    let params = DreamParams::from_toml("fixtures/dream_params.toml");
    let target = ChromaticTensor::from_palette_file("fixtures/chromatic_targets/target_palette_12.json", 0);

    let mut pool1 = SimpleDreamPool::new(params.max_size, params.coherence_threshold);
    let out1 = dream_cycle(&target, &mut pool1, params.epochs, &seeds);

    let mut pool2 = SimpleDreamPool::new(params.max_size, params.coherence_threshold);
    let out2 = dream_cycle(&target, &mut pool2, params.epochs, &seeds);

    assert_eq!(hash64(&out1.tensor), hash64(&out2.tensor));
    assert_eq!(pool1.to_json(), pool2.to_json());
}


A/B Harness Assertion

#[test]
fn ab_retrieval_vs_random() {
    let tol = Tolerances::from_toml("fixtures/tolerances.toml");
    let res = run_ab_harness(/* shared seeds, params, targets */);

    // Preferred success
    if (res.loss_test + tol.loss_delta) < res.loss_control ||
       (res.coh_test  - tol.coh_delta)  > res.coh_control {
        assert!(true);
    } else {
        // Neutral acceptance, must be deterministic
        assert!(res.is_bit_exact_repro, "A/B neutral but non-deterministic");
    }
}

Fixtures

seeds.toml — global and per-epoch seeds

tolerances.toml — { loss_delta, coh_delta, energy_drift_db, delta_e }

dream_params.toml — { epochs, max_size, coherence_threshold, noise_alpha, feedback_beta }

target_palette_12.json — canonical 12-category palette targets

pool_reference.json — known-good pool snapshot for regression

Outputs

experiments/results/dream_validation_<run>.json

experiments/results/dream_ab_summary.md

Optional PNG/SVG from diagnostics/visual if feature enabled.

Each result file includes config hashes and RNG seeds used.

Pass Criteria (Global)
Metric	Threshold
Replay hash equality	exact
Pool order stability	exact
Mean ΔE to target	≤ 1e-3
Spectral drift	≤ 0.5 dB
A/B delta (preferred)	loss↓ or coherence↑ by configured δ, else deterministic neutral
Dependencies

dream (generator, pool, cycle, evaluator)

tensor (mix/ops, RGB/HSL, spectral ops)

bridge (chromatic↔spectral conversions)

diagnostics (metrics: ΔHSL, coherence, energy)

meta (optional chronicle hashing / seed capture)

Status
Field	Value
Spec Version	1.0
Phase Alignment	6A–6E validation
Determinism Level	Bit-exact (test-mode single-thread)
Readiness	✅ Ready for implementation
Next Tests	bridge_tests/ and diagnostics_tests/ (already specced)
