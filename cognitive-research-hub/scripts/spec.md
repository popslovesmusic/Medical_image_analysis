cognitive-research-hub/scripts/scripts-spec.md
Purpose

This folder houses utility, orchestration, and pipeline scripts that operate outside the compiled core.
Scripts here handle automation, dataset prep, experiment runs, log conversion, and integration tasks bridging your trainer, core, and diagnostic subsystems.

They must:

Maintain reproducibility through seed and config recording.

Respect determinism (no hidden state or random defaults).

Be platform-agnostic (Windows / Linux).

Directory Layout
cognitive-research-hub/scripts/
├─ scripts-spec.md               ← this spec
├─ data_tools/
│   ├─ dataset_splitter.py       ← deterministic train/val/test partition
│   ├─ image_preprocessor.py     ← normalization, masking, augmentation
│   ├─ metadata_extractor.py     ← DICOM/PNG tag extraction
│   ├─ hash_integrity.py         ← SHA/CRC hash registry
│   └─ README.md
├─ orchestration/
│   ├─ run_experiment.py         ← launches reproducible runs (Phase 6E–8)
│   ├─ pipeline_monitor.py       ← monitors checkpoints, metrics logs
│   ├─ archive_results.py        ← moves experiment outputs → /experiments/results
│   └─ env_check.py              ← verifies toolchain, CUDA, AVX, etc.
├─ visualization/
│   ├─ tensor_visualizer.py      ← renders Chromatic/Spectral tensors
│   ├─ metric_plotter.py         ← loss & coherence curves, FFT drift
│   └─ dream_animation.py        ← replay of dream cycles (Phase 6E)
└─ utils/
    ├─ config_loader.py          ← unified parser for .toml/.json/.yaml
    ├─ seed_manager.py           ← deterministic RNG seeding
    ├─ log_formatter.py          ← converts raw logs → standardized format
    └─ time_utils.py             ← canonical timestamp utilities

Script Categories and Roles
Category	Purpose	Interfaces With
data_tools	Dataset and preprocessing utilities ensuring reproducible sample handling and hash integrity.	trainer, diagnostics
orchestration	Run coordination, checkpointing, result archival, and environment verification.	core, trainer, experiments
visualization	Diagnostic and qualitative monitoring of tensors and dream behavior.	tensor, dream, bridge
utils	Shared deterministic support utilities (logging, config, RNG).	all
Deterministic Rules

Seed Capture: Every script imports seed_manager.py and logs seed to meta/chronicle.

Hash Integrity: All generated artifacts are SHA256-logged to data/hash_registry.json.

Environment Lock: env_check.py exports compiler, Python, CUDA, AVX version snapshot.

Logging: All scripts output structured JSON logs (no plain text) for later aggregation.

Example: Deterministic Run Invocation
python orchestration/run_experiment.py \
    --config trainer/config/phase7.toml \
    --seeds scripts/utils/seeds.toml \
    --logdir experiments/results/run_2025_10_31

Script Execution Phases
Phase	Scripts Involved	Output
Phase 5–6: Dreamer + Learner testing	run_experiment.py, tensor_visualizer.py	loss/coherence CSVs
Phase 7: Bridge & Sonic Encoding	dream_animation.py, metric_plotter.py	FFT plots, sonic mappings
Phase 8: Full System Validation	archive_results.py, hash_integrity.py	validated experiment archives
Phase 9: Deployment Tooling	env_check.py, config_loader.py	environment manifests
Output Conventions

All scripts must write artifacts to:

experiments/results/<script>_<timestamp>.json


and register them in:

meta/chronicle/manifest.json

Pass/Fail Validation
Test	Criterion
Hash reproducibility	identical across two full runs
Environment parity	all checks pass; version hash stable
Dataset split determinism	identical partitions given same seed
Visualization energy drift	≤ 0.5 dB
Status
Field	Value
Spec Version	1.0
Phase Alignment	6 → 8 automation support
Determinism Level	Full (seeded)
Readiness	✅ Implementation Ready
Next Spec	trainer/config/ and trainer/docs/ folders