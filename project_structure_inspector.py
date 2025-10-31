"""
project_structure_inspector.py
--------------------------------
Enhanced directory discovery and reporting tool for the
Medical_image_analysis / Cognitive Research Hub project.

Features:
- Recursively maps the full directory tree.
- Classifies directories into Core, Trainer, Docs, Data, Experiments, and Tests.
- Excludes caches, virtual environments, build artifacts.
- Provides optional JSON export for automated report generation.
"""

import os
import json
from datetime import datetime

EXCLUDE_DIRS = {
    '.venv', 'venv', 'env', '__pycache__',
    'node_modules', '.git', '.pytest_cache', '.mypy_cache',
    'build', 'dist', '.tox', '.coverage', 'htmlcov',
    '.idea', '.vscode', 'site-packages'
}

EXCLUDE_EXTS = {'.pyc', '.pyo', '.pyd', '.so', '.egg-info'}
EXCLUDE_FILES = {'.DS_Store', 'Thumbs.db', '.gitkeep'}


def classify_directory(dirname):
    """Return a semantic tag for known top-level folders."""
    dirname_lower = dirname.lower()
    if 'core' in dirname_lower:
        return 'CORE_MODULE'
    if 'trainer' in dirname_lower:
        return 'TRAINER_MODULE'
    if 'docs' in dirname_lower:
        return 'DOCUMENTATION'
    if 'data' in dirname_lower:
        return 'DATASETS'
    if 'experiments' in dirname_lower:
        return 'EXPERIMENTS'
    if 'tests' in dirname_lower:
        return 'TESTS'
    if 'scripts' in dirname_lower:
        return 'SCRIPTS'
    if 'notebooks' in dirname_lower:
        return 'NOTEBOOKS'
    return 'MISC'


def list_project_contents(path, export_json=False, output_file="project_structure.json"):
    """
    Recursively lists the file and directory structure of a given path.
    If export_json=True, also writes structured metadata to a JSON file.

    Args:
        path (str): Root directory.
        export_json (bool): Whether to write to JSON.
        output_file (str): JSON export filename.
    """
    structure = {
        "project": os.path.basename(path),
        "generated_at": datetime.utcnow().isoformat(),
        "structure": []
    }

    for root, dirs, files in os.walk(path):
        dirs[:] = [d for d in dirs if d not in EXCLUDE_DIRS]
        level = root.replace(path, '').count(os.sep)
        indent = ' ' * 4 * level

        if level == 0:
            print(f"\nProject structure for: {os.path.basename(path)}/\n")
        else:
            print(f"{indent}├─ {os.path.basename(root)}/")

        subindent = ' ' * 4 * (level + 1)
        dir_entry = {
            "path": root,
            "type": classify_directory(os.path.basename(root)),
            "files": []
        }

        for f in files:
            ext = os.path.splitext(f)[1].lower()
            if ext not in EXCLUDE_EXTS and f not in EXCLUDE_FILES:
                print(f"{subindent}├─ {f}")
                dir_entry["files"].append(f)

        structure["structure"].append(dir_entry)

    if export_json:
        with open(output_file, "w", encoding="utf-8") as out:
            json.dump(structure, out, indent=4)
        print(f"\n✅ JSON export written to {output_file}")

    return structure


if __name__ == "__main__":
    current_dir = os.getcwd()
    list_project_contents(current_dir, export_json=True)
