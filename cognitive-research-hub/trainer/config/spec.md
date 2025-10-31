cognitive-research-hub/trainer/config/config-spec.md
Purpose

The config/ directory centralizes experiment, model, and environment configuration for the training subsystem.
It defines all parameter sets used by the trainer, ensuring that each experiment is reproducible, version-controlled, and self-documenting.

All .toml, .json, or .yaml configuration files here are immutable during execution — any runtime changes must be logged as deltas into the meta/chronicle.

Directory Layout
cognitive-research-hub/trainer/config/
├─ config-spec.md                     ← this spec
├─ base/
│   ├─ default.toml                   ← canonical global defaults
│   ├─ environment.toml               ← paths, hardware, thread settings
│   ├─ seeds.toml                     ← RNG seeds (trainer + dreamer)
│   └─ logging.toml                   ← log format, verbosity, color maps
├─ model/
│   ├─ transformer.toml               ← encoder/decoder and attention params
│   ├─ learner.toml                   ← MLP classifier, optimizer params
│   └─ dreamer.toml                   ← dream cycle configuration
├─ data/
│   ├─ dataset_paths.toml             ← local and remote data references
│   ├─ augmentation.toml              ← deterministic augmentation config
│   └─ split.toml                     ← train/val/test proportions + seeds
├─ experiments/
│   ├─ phase5.toml                    ← learner validation config
│   ├─ phase6.toml                    ← dream pool and feedback loop config
│   ├─ phase7.toml                    ← chromatic↔sonic bridge training config
│   ├─ phase8.toml                    ← full-system integration tests
│   └─ phase9.toml                    ← deployment and monitoring settings
└─ registry.json                      ← index of configs + checksum manifest

Configuration Schema Overview

Each .toml or .json adheres to a unified schema to ensure that all modules can be validated and parsed through scripts/utils/config_loader.py.

Global Keys
Key	Type	Description
meta.name	string	Human-readable name of configuration
meta.version	string	Semantic version number (e.g., 1.0.0)
meta.phase	int	Phase association (5–9)
paths.output_dir	string	Output directory for experiment results
seeds.global	int	Global RNG seed
device	string	Target device (cpu, cuda, tpu)
threads.max	int	Thread cap for deterministic reproducibility
logging.level	string	One of DEBUG, INFO, WARN, ERROR
Example: base/default.toml
[meta]
name = "default-base"
version = "1.0.0"
phase = 0

[paths]
output_dir = "experiments/results/"
data_dir = "data/processed/"

[seeds]
global = 42
numpy = 1234
torch = 4321

[device]
backend = "cuda"
precision = "float32"

[threads]
max = 1

[logging]
level = "INFO"
format = "structured-json"
color = true

Example: model/learner.toml
[model]
type = "MLP"
input_dim = 3072
hidden_dim = 256
output_dim = 10
activation = "ReLU"
init = "xavier"
dropout = 0.05

[optimizer]
type = "SGD"
learning_rate = 0.01
decay = 0.95
momentum = 0.9

[training]
epochs = 25
batch_size = 16
convergence_threshold = 0.95

Example: data/split.toml
[data]
train_ratio = 0.8
val_ratio = 0.1
test_ratio = 0.1
shuffle = true
seed = 2025

Example: experiments/phase6.toml
[experiment]
phase = 6
description = "Dream Pool feedback refinement test"

[dream_pool]
max_size_gb = 10.0
coherence_threshold = 0.75
retrieval_limit = 5
retrieval_mode = "cosine"

[feedback]
track_utility = true
delta_metric = "Δaccuracy"
bias_profile_output = "experiments/results/phase6_bias_profile.json"

Validation Rules

All configs are validated at runtime by config_loader.py.
The loader enforces:

Rule	Enforcement
Schema consistency	Required keys must exist
Determinism	Seeds and thread caps are mandatory
Cross-phase linkage	Each phase config references its predecessor via meta.previous
Integrity	Registry checksum verified via hash_integrity.py
Version control	All configs must have a semantic version field
Registry System

registry.json keeps SHA256 checksums and last modified timestamps for every config file.

Used by scripts/orchestration to ensure the configuration state matches the commit hash of the codebase.

Supports rollback through registry.json.bak.

Output & Logging

When a training or validation run starts:

A copy of the used config is stored in:

experiments/archive/configs/<timestamp>_<phase>.toml


The run’s meta log includes config hash:

{
  "phase": 6,
  "config_hash": "af3d12b9...",
  "timestamp": "2025-10-31T17:42:00Z"
}

Pass Criteria
Validation Target	Requirement
Config schema check	✅ passes
Registry checksum	✅ verified
Determinism (seed/thread)	✅ enforced
Cross-phase link integrity	✅ valid
Loader performance	≤ 100 ms parse time
Status
Field	Value
Spec Version	1.0
Phase Alignment	Supports 5–9
Readiness	✅ Complete
Next Spec	trainer/docs/ — trainer-level documentation, including experiment protocols and results indexing