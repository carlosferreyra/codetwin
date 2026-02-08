"""Verify Cargo.toml and pyproject.toml versions match."""

from __future__ import annotations

from pathlib import Path

import tomllib


def load_toml(path: Path) -> dict:
    return tomllib.loads(path.read_text(encoding="utf-8"))


def main() -> int:
    cargo = load_toml(Path("Cargo.toml"))
    pyproject = load_toml(Path("pyproject.toml"))

    cargo_version = cargo.get("package", {}).get("version")
    pyproject_version = pyproject.get("project", {}).get("version")

    print(f"Cargo version: {cargo_version}")
    print(f"pyproject version: {pyproject_version}")

    if not cargo_version or not pyproject_version:
        raise SystemExit("Missing version in Cargo.toml or pyproject.toml")
    if cargo_version != pyproject_version:
        raise SystemExit("Version mismatch between Cargo.toml and pyproject.toml")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
