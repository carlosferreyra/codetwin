"""Sync pyproject metadata and publish the package via uv.

Run via: uv run -- python scripts/publish_pypi.py
"""

from __future__ import annotations

import subprocess

from sync_pyproject import main as sync_main


def run_publish() -> int:
    result = subprocess.run(["uv", "publish"], check=False)
    return result.returncode


def main() -> int:
    sync_code = sync_main()
    if sync_code != 0:
        return sync_code

    return run_publish()


if __name__ == "__main__":
    raise SystemExit(main())
