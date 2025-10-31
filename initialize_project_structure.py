"""
initialize_project_structure.py
--------------------------------
Creates the complete Medical_image_analysis project directory tree
and adds placeholder spec.md files where appropriate.

Safe to rerun — existing files are preserved.
"""

import os
from pathlib import Path

# --- Root path ---
ROOT = Path("Medical_image_analysis")

# --- Folder map (core project skeleton) ---
FOLDERS = [
    # Root
    "",
    "cognitive-research-hub",
    "cognitive-research-hub/core",
    "cognitive-research-hub/core/src",
    "cognitive-research-hub/core/src/bridge",
    "cognitive-research-hub/core/src/diagnostics",
    "cognitive-research-hub/core/src/diagnostics/visual",
    "cognitive-research-hub/core/src/diagnostics/metrics",
    "cognitive-research-hub/core/src/dream",
    "cognitive-research-hub/core/src/meta",
    "cognitive-research-hub/core/src/tensor",
    "cognitive-research-hub/core/tests/bridge_tests",
    "cognitive-research-hub/core/tests/tensor_tests",
    "cognitive-research-hub/core/tests/diagnostics_tests",
    "cognitive-research-hub/scripts",
    "cognitive-research-hub/trainer",
    "cognitive-research-hub/trainer/config",
    "cognitive-research-hub/trainer/docs",
    "cognitive-research-hub/trainer/src/model",
    "cognitive-research-hub/trainer/src/tokenizer",
    "cognitive-research-hub/trainer/src/training",
    "cognitive-research-hub/trainer/src/validator",
    "cognitive-research-hub/trainer/src/reports",
    "data/raw",
    "data/processed",
    "data/samples",
    "docs/diagrams",
    "experiments/archive",
    "experiments/results",
    "experiments/configs",
    "notebooks",
    "tests/fixtures",
    "tests/logs",
]

# --- Placeholder Markdown ---
SPEC_CONTENT = """# Specification: {name}

**Module Path:** `{path}`

This placeholder spec.md file is automatically generated.
Replace this with the full specification for this module.

---
✅ Created by initialize_project_structure.py
"""

def create_structure():
    print(f"\n📁 Initializing project structure at: {ROOT.resolve()}\n")

    for folder in FOLDERS:
        full_path = ROOT / folder
        full_path.mkdir(parents=True, exist_ok=True)
        print(f"✅ Created folder: {full_path}")

        # Determine if a spec.md should be created
        spec_file = full_path / "spec.md"
        if not spec_file.exists() and folder:
            # Only place specs in functional directories, not data/docs/fixtures
            if any(x in folder for x in ["core", "trainer", "diagnostics", "bridge", "tensor", "dream", "meta", "src"]):
                with open(spec_file, "w", encoding="utf-8") as f:
                    f.write(SPEC_CONTENT.format(name=full_path.name, path=folder))
                print(f"   └─ Added placeholder spec.md to {full_path.name}/")

    print("\n✅ Directory initialization complete.\n")

if __name__ == "__main__":
    create_structure()
