Specification: Archive Restoration & Replay Subsystem

Module Path: experiments/archive/restoration/
Parent Specs:

experiments/archive/spec.md (archive creation & integrity)

experiments/configs/spec.md (validated experiment definitions)

core/meta/chronicle/spec.md (system-wide provenance ledger)

trainer/spec.md (execution environment consistency)

I. Mission

The Restoration Subsystem enables deterministic re-execution (“replay”) of any archived experiment.
Its goals are:

Bit-Exact Reconstruction — Recreate the environment, configuration, and data exactly as they were at archive time.

Integrity Verification — Confirm that restored outputs match archived results within tolerance (Δ ≤ 1e⁻⁸).

Temporal Continuity — Reinsert the restored run into the active chronicle for traceability.

This subsystem is the keystone of reproducibility across the Medical Image Analysis project.

II. Directory Layout
experiments/archive/restoration/
├─ scripts/             # Restoration automation tools
├─ env/                 # Environment specifications (conda/pip/freeze)
├─ manifests/           # Metadata for each restored experiment
├─ logs/                # Restoration and verification logs
├─ replays/             # Working directories for re-executed runs
└─ reports/             # Verification reports and deltas

III. Restoration Workflow
Step	Description	Output
1. Select Archive	Choose bundle or manifest from archive/manifests/	manifest.json
2. Verify Integrity	Compare SHA-256 hashes	verification/log.txt
3. Extract Bundle	Unpack into isolated replays/<exp_id>/	File tree
4. Reconstruct Environment	Restore exact Python + system dependencies	env/<exp_id>/
5. Replay Execution	Rerun training/inference with frozen config	results/
6. Validate Outputs	Compare to archived metrics and plots	reports/verification.md
7. Chronicle Update	Append restoration event	core/meta/chronicle.log

All seven steps are automated via scripts/restore_experiment.py.

IV. Deterministic Environment Rebuild

Every archived experiment includes an environment snapshot file, one of:

File	Description
environment.yml	Conda environment definition
requirements.txt	Pip package list
system_manifest.json	OS, CUDA, compiler, and hardware fingerprint
Example:
name: exp_2025_11_01_restore
dependencies:
  - python=3.11.5
  - pytorch=2.3.0
  - torchvision=0.18.0
  - numpy=1.26.4
  - pandas=2.2.2
  - cudatoolkit=12.1
  - sha256: b8d091...


Reconstruction process:

Resolve version pins exactly (no wildcard >= or <).

Use platform-specific installers (Windows/Linux parity checked).

Confirm environment hash via:

python -m pip freeze | sha256sum > env_hash.txt


Match against recorded value in system_manifest.json.

V. Replay Execution Rules
Constraint	Enforcement
Global Seed	Must match archived seed (default 42)
File Paths	Relative, normalized to /
I/O Order	Alphabetical enumeration
Threading	Single-threaded mode for determinism
Floating Precision	Float32 unless otherwise specified
GPU Determinism	torch.use_deterministic_algorithms(True) enforced
Logging	Timestamped to UTC but excluded from hashes

During replay, the script executes:

python train.py --config configs/validated/<id>.yaml --replay-mode


which ensures that only deterministic code paths are used.

VI. Output Verification

After replay, the subsystem runs a deterministic diff procedure:

1. Metric Verification

For each metric 
𝑀
M:

∣
Δ
𝑀
∣
=
∣
𝑀
restored
−
𝑀
archived
∣
<
10
−
8
∣ΔM∣=∣M
restored
	​

−M
archived
	​

∣<10
−8
2. File Hash Verification

Every file in results/ is re-hashed and compared with the archived manifest:

sha256sum results/* > hash_restore.txt
diff hash_restore.txt manifests/hash_archive.txt

3. FFT Energy Consistency

For models producing spectral data, ensure:

Δ
𝐸
FFT
<
0.5
 
dB
ΔE
FFT
	​

<0.5dB

validated through core/diagnostics/energy_check.py.

4. Report Generation

Example summary (reports/verification.md):

# Restoration Verification Report: exp_2025_11_01
✅ Hash Verification: Passed
✅ Metric Drift: < 1e-8
✅ FFT Energy Balance: Stable
✅ Environment Checksum: Match
Overall Result: ✅ FULL DETERMINISTIC RESTORATION

VII. Manifest Schema

Every restoration creates its own manifest entry (appended to chronicle).

{
  "experiment_id": "exp_2025_11_01_phase7b_bridge",
  "restoration_id": "restore_2025_11_02_01",
  "source_archive": "exp_2025_11_01_phase7b.zip",
  "restored_at": "2025-11-02T14:00:00Z",
  "restorer": "codex",
  "environment_hash": "3a9f7e...",
  "metric_drift": "4.2e-09",
  "verified": true
}

VIII. Automation Scripts
Script	Purpose
restore_experiment.py	Main restoration entrypoint
verify_restoration_integrity.py	Performs hash & metric checks
generate_restoration_report.py	Produces Markdown + JSON outputs
compare_environment_hashes.py	Confirms identical dependency sets
register_restoration_event.py	Appends verified restoration to chronicle
IX. Error Handling & Recovery
Error	Response
Hash mismatch	Abort restoration; mark manifest as invalid
Missing dependency	Retry with fallback mirror (logged deterministically)
Metric drift > threshold	Run double replay check; if consistent, flag "drift-verified"
Corrupt bundle	Move to /archive/quarantine/ and issue SHA report

All anomalies are logged and linked to the global chronicle.

X. Integration & Provenance Flow
Source	Destination	Description
archive/bundles/	restoration/replays/	Extract bundle
archive/manifests/	restoration/manifests/	Validate metadata
core/meta/chronicle/	restoration/logs/	Record replay event
core/diagnostics/	restoration/reports/	Cross-validate outputs
XI. Deterministic Guarantees
Property	Enforcement Mechanism
Environment Identity	SHA-256 hash of dependency tree
Config Integrity	Manifest hash cross-check
Random State	Global seed fixed at 42
GPU Algorithm Control	Deterministic flags enforced
Parallelism	Disabled or single-threaded
Numeric Stability	Float32 + reproducible math kernels
File Order	Sorted read/write operations
XII. Long-Term Validation

A nightly CI task (verify_all_restorations.py) automatically replays one archived experiment per batch to confirm ongoing system reproducibility under current OS and driver versions.

XIII. Compliance Rules
Rule	Guarantee
SHA integrity required	No replay without verified archive
Drift threshold	≤ 1e⁻⁸ numeric delta
GPU flag	Deterministic enforced
OS independence	Cross-platform validation mandatory
UTF-8 logs	Required for all reports
ISO timestamps	UTC only
XIV. Compliance Summary
Field	Specification
Spec Version	1.0
Determinism Level	Bit-Exact
Hash Algorithm	SHA-256
Tolerance Threshold	1e⁻⁸
Global Seed	42
Audit Authority	Codex Restoration Agent
Revision	{{auto-date}}
Status	✅ Verified Replay Integrity