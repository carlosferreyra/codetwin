"""Sync pyproject.toml metadata from Cargo.toml.

Keeps Python package metadata aligned with the Rust crate.
Run via: uv run -- python scripts/sync_pyproject.py
"""

from __future__ import annotations

import pathlib
import sys

import tomllib

ROOT = pathlib.Path(__file__).resolve().parents[1]
CARGO_TOML = ROOT / "Cargo.toml"
PYPROJECT_TOML = ROOT / "pyproject.toml"


def load_cargo_metadata() -> dict:
    with CARGO_TOML.open("rb") as handle:
        data = tomllib.load(handle)
    return data.get("package", {})


def update_pyproject_lines(lines: list[str], cargo: dict) -> list[str]:
    project_section = False
    urls_section = False

    name = cargo.get("name")
    version = cargo.get("version")
    description = cargo.get("description")
    authors = cargo.get("authors", [])
    repository = cargo.get("repository")
    documentation = cargo.get("documentation")

    updated = []
    for line in lines:
        stripped = line.strip()
        if stripped.startswith("[") and stripped.endswith("]"):
            project_section = stripped == "[project]"
            urls_section = stripped == "[project.urls]"

        if project_section and stripped.startswith("name =") and name:
            updated.append(f"  name = \"{name}\"\n")
            continue
        if project_section and stripped.startswith("version =") and version:
            updated.append(f"  version = \"{version}\"\n")
            continue
        if project_section and stripped.startswith("description =") and description:
            updated.append(f"  description = \"{description}\"\n")
            continue
        if project_section and stripped.startswith("authors =") and authors:
            author_name = authors[0]
            updated.append(f"  authors = [{{ name = \"{author_name}\" }}]\n")
            continue
        if urls_section and stripped.startswith("Homepage") and repository:
            updated.append(f"  Homepage      = \"{repository}\"\n")
            continue
        if urls_section and stripped.startswith("Documentation") and documentation:
            updated.append(f"  Documentation = \"{documentation}\"\n")
            continue

        updated.append(line)

    return updated


def main() -> int:
    if not CARGO_TOML.exists() or not PYPROJECT_TOML.exists():
        print("Missing Cargo.toml or pyproject.toml", file=sys.stderr)
        return 1

    cargo = load_cargo_metadata()
    lines = PYPROJECT_TOML.read_text(encoding="utf-8").splitlines(keepends=True)
    updated = update_pyproject_lines(lines, cargo)

    if updated != lines:
        PYPROJECT_TOML.write_text("".join(updated), encoding="utf-8")
        print("Updated pyproject.toml from Cargo.toml")
    else:
        print("pyproject.toml already matches Cargo.toml")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
