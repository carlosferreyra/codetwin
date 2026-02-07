"""Generate RELEASES.md entries from git history.

Run via: python3 scripts/generate_releases.py
"""

from __future__ import annotations

import datetime
import pathlib
import subprocess
import sys

import tomllib

ROOT = pathlib.Path(__file__).resolve().parents[1]
CARGO_TOML = ROOT / "Cargo.toml"
RELEASES_MD = ROOT / "RELEASES.md"


def run(cmd: list[str]) -> str:
    result = subprocess.run(cmd, check=False, capture_output=True, text=True)
    if result.returncode != 0:
        stderr = result.stderr.strip()
        raise RuntimeError(stderr or "Command failed")
    return result.stdout.strip()


def load_version() -> str:
    with CARGO_TOML.open("rb") as handle:
        data = tomllib.load(handle)
    version = data.get("package", {}).get("version")
    if not version:
        raise RuntimeError("Missing package.version in Cargo.toml")
    return version


def list_tags() -> list[str]:
    output = run(["git", "tag", "--list", "v*", "--sort=-v:refname"])
    return [line for line in output.splitlines() if line.strip()]


def last_tag_excluding(version: str) -> str | None:
    current = f"v{version}"
    for tag in list_tags():
        if tag != current:
            return tag
    return None


def git_commits(since_tag: str | None) -> list[str]:
    cmd = ["git", "log", "--pretty=format:%s", "--no-merges"]
    if since_tag:
        cmd.insert(2, f"{since_tag}..HEAD")
    output = run(cmd)
    return [line for line in output.splitlines() if line.strip()]


def build_section(version: str, commits: list[str]) -> list[str]:
    date = datetime.date.today().isoformat()
    lines = [f"## v{version} - {date}", ""]
    if commits:
        lines.extend([f"- {message}" for message in commits])
    else:
        lines.append("- No changes recorded.")
    lines.append("")
    return lines


def load_existing() -> list[str]:
    if not RELEASES_MD.exists():
        return ["# Releases", "", ""]
    return RELEASES_MD.read_text(encoding="utf-8").splitlines()


def write_release(version: str, commits: list[str]) -> None:
    existing = load_existing()
    heading = f"## v{version} -"
    if any(line.startswith(heading) for line in existing):
        return

    section = build_section(version, commits)
    if existing and existing[0].startswith("# Releases"):
        new_lines = existing[:2] + section + existing[2:]
    else:
        new_lines = ["# Releases", "", *section, *existing]

    RELEASES_MD.write_text("\n".join(new_lines).rstrip() + "\n", encoding="utf-8")


def main() -> int:
    try:
        version = load_version()
        since_tag = last_tag_excluding(version)
        commits = git_commits(since_tag)
        write_release(version, commits)
    except Exception as exc:  # noqa: BLE001 - CLI tool
        print(f"Release generation failed: {exc}", file=sys.stderr)
        return 1

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
