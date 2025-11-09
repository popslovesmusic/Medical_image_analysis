# Module: experiments/archive/
# Spec Version: 1.0

## Purpose

This directory stores compressed archives of completed, significant experimental results from the `experiments/results/` directory.

This is for long-term storage of important baseline or "golden" runs.

## Scope

* Contains compressed files (e.g., `.tar.gz`, `.zip`) of output directories from `experiments/results/`.
* May also contain a `README.md` summarizing the archived experiments.
* This directory *may* be tracked by Git (or Git LFS) if the archives are small enough and critical for project history.

## Sub-directory: `restoration/`

* As specified in `IMPLEMENTATION_CHECKLIST.md` (Phase 9), this directory (or a peer) will contain the verification scripts for the determinism audit.
* `archive/restoration/spec.md` will define the audit script that un-archives a run, re-runs it, and compares the results.

## Status

* **Spec Version:** 1.0
* **Phase Alignment:** **Phase 8 & 9** (per `IMPLEMENTATION_CHECKLIST.md`)
* **Readiness:** ✅ Approved