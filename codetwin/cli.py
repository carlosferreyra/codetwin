import os
import platform
import subprocess
import sys
from pathlib import Path


def get_binary_path() -> Path:
    """Get path to compiled binary for current platform."""
    system = sys.platform
    machine = platform.machine().lower()

    if system == "darwin":
        if machine in {"arm64", "aarch64"}:
            binary = "codetwin-aarch64-darwin"
        else:
            binary = "codetwin-x86_64-darwin"
    elif system.startswith("linux"):
        if machine in {"aarch64", "arm64"}:
            binary = "codetwin-aarch64-linux-gnu"
        else:
            binary = "codetwin-x86_64-linux-gnu"
    else:
        raise RuntimeError(f"Unsupported platform: {system}")

    path = Path(__file__).parent / "_bin" / binary
    if not path.exists():
        raise RuntimeError(f"Binary not found: {path}")
    return path


def main() -> None:
    """Main entry point - delegate to Rust binary."""
    binary = get_binary_path()
    os.chmod(binary, 0o755)
    raise SystemExit(subprocess.call([str(binary)] + sys.argv[1:]))


if __name__ == "__main__":
    main()
