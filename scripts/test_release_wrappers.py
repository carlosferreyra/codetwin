"""Regression checks for wrapper generation from an inherited Cargo workspace."""

import json
import subprocess
import tempfile
import tomllib
import unittest
from pathlib import Path


class WorkspaceWrappers(unittest.TestCase):
    def test_inherited_package_metadata(self):
        scripts = Path(__file__).resolve().parent
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            package = root / "crates/codetwin"
            (package / "src").mkdir(parents=True)
            (package / "src/main.rs").write_text("fn main() {}\n")
            (root / "Cargo.toml").write_text('''
[workspace]
members = ["crates/codetwin"]
resolver = "3"
[workspace.package]
version = "1.2.3"
edition = "2024"
description = "Workspace wrapper fixture"
repository = "https://github.com/example/codetwin"
license = "MIT"
authors = ["Example <example@example.com>"]
''')
            (package / "Cargo.toml").write_text('''
[package]
name = "codetwin"
version.workspace = true
edition.workspace = true
description.workspace = true
repository.workspace = true
license.workspace = true
authors.workspace = true
''')
            (root / "README.md").write_text("Wrapper fixture\n")
            subprocess.run(["cargo", "generate-lockfile", "--offline"], cwd=root, check=True)
            for command in (["uv", "run", str(scripts / "release_pypi.py")],
                            ["bun", "run", str(scripts / "release_npm.ts")]):
                subprocess.run(command, cwd=root, check=True, capture_output=True, text=True)
            python = tomllib.loads((root / ".release/python/pyproject.toml").read_text())["project"]
            npm = json.loads((root / ".release/npm/package.json").read_text())
            for metadata in (python, npm):
                self.assertEqual(metadata["name"], "codetwin")
                self.assertEqual(metadata["version"], "1.2.3")
                self.assertEqual(metadata["description"], "Workspace wrapper fixture")
                self.assertEqual(metadata["license"], "MIT")
            self.assertEqual(python["authors"][0]["email"], "example@example.com")
            self.assertEqual(npm["repository"]["url"], "git+https://github.com/example/codetwin.git")
            self.assertTrue((root / ".release/npm/bin/codetwin.cjs").is_file())
            self.assertTrue((root / ".release/python/src/codetwin/__init__.py").is_file())


if __name__ == "__main__":
    unittest.main()
