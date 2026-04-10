# /// script
# requires-python = ">=3.12"
# dependencies = []
# ///

import sys
import textwrap
from dataclasses import dataclass, field
from pathlib import Path
from typing import Self

try:
    import tomllib
except ImportError:
    import tomli as tomllib


@dataclass(frozen=True)
class Author:
    name: str
    email: str | None = None

    def to_pep621(self) -> str:
        email_part = f', email = "{self.email}"' if self.email else ""
        return f'{{ name = "{self.name}"{email_part} }}'


@dataclass(frozen=True)
class PackageMetadata:
    name: str
    version: str
    description: str
    repository: str
    license_id: str
    authors: list[Author] = field(default_factory=list)

    @property
    def module_name(self) -> str:
        return self.name.lower().replace("-", "_")

    @property
    def classifiers(self) -> list[str]:
        return [
            "Programming Language :: Python :: 3",
            "Programming Language :: Python :: 3.12",
            "Programming Language :: Python :: 3 :: Only",
            "Environment :: Console",
            "Intended Audience :: Developers",
            "Topic :: Software Development :: Documentation",
            "Topic :: Software Development :: Libraries",
        ]

    @classmethod
    def from_cargo(cls, root: Path) -> Self:
        cargo_path = root / "Cargo.toml"
        if not cargo_path.exists():
            raise FileNotFoundError(f"Missing Cargo.toml at {root}")

        data = tomllib.loads(cargo_path.read_text())["package"]

        parsed_authors = []
        for raw_author in data.get("authors", []):
            match raw_author.split("<"):
                case [name, email_raw]:
                    parsed_authors.append(
                        Author(name.strip(), email_raw.removesuffix(">").strip())
                    )
                case [name]:
                    parsed_authors.append(Author(name.strip()))

        return cls(
            name=data["name"],
            version=data["version"],
            description=data.get("description", "Rust CLI wrapper"),
            repository=data.get("repository", ""),
            license_id=data.get("license", "MIT"),
            authors=parsed_authors,
        )


class Templates:
    PYPROJECT = textwrap.dedent("""
        [project]
        name = "{meta.name}"
        version = "{meta.version}"
        description = "{meta.description}"
        readme = "README.md"
        requires-python = ">=3.12"
        license = "{meta.license_id}"
        authors = [
            {authors}
        ]
        dependencies = []
        classifiers = [
            {classifiers}
        ]

        [project.scripts]
        {meta.name} = "{meta.module_name}:main"

        [project.urls]
        Repository = "{meta.repository}"
        Homepage = "{meta.repository}#readme"

        [build-system]
        requires = ["uv_build>=0.11.2,<0.12.0"]
        build-backend = "uv_build"
    """).strip()

    CLI_WRAPPER = textwrap.dedent("""
        import platform
        import subprocess
        import sys
        from pathlib import Path


        def _bootstrap_binary() -> None:
            tag = "v{meta.version}"
            base = "{meta.repository}".rstrip("/")

            match platform.system().lower():
                case "windows":
                    url = f"{{base}}/releases/download/{{tag}}/{meta.name}-installer.ps1"
                    cmd = ["powershell", "-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", f"iwr -useb '{{url}}' | iex"]
                case _:
                    url = f"{{base}}/releases/download/{{tag}}/{meta.name}-installer.sh"
                    cmd = ["sh", "-c", f"curl -LsSf '{{url}}' | sh"]

            subprocess.run(cmd, check=False)


        def main() -> int:
            bin_name = "{meta.name}"
            if platform.system().lower() == "windows":
                bin_name += ".exe"

            # Use absolute path to avoid recursion with the Python shim in PATH.
            exe = Path.home() / ".cargo" / "bin" / bin_name

            if not exe.exists():
                print(f"Binary not found at {{exe}}. Attempting to install...", file=sys.stderr)
                _bootstrap_binary()

            if exe.exists():
                return subprocess.run([str(exe), *sys.argv[1:]]).returncode

            print(f"Failed to find or install {{bin_name}} at {{exe}}", file=sys.stderr)
            return 1


        if __name__ == "__main__":
            sys.exit(main())
    """).strip()


def main() -> None:
    try:
        meta = PackageMetadata.from_cargo(Path.cwd())
    except Exception as e:
        print(f"FATAL: {e}", file=sys.stderr)
        sys.exit(1)

    out_dir = Path(".release/python")
    pkg_dir = out_dir / "src" / meta.module_name
    pkg_dir.mkdir(parents=True, exist_ok=True)

    authors_toml = ",\n    ".join(a.to_pep621() for a in meta.authors)
    classifiers_toml = ",\n    ".join(f'"{c}"' for c in meta.classifiers)

    (out_dir / "pyproject.toml").write_text(
        Templates.PYPROJECT.format(
            meta=meta, authors=authors_toml, classifiers=classifiers_toml
        )
    )

    (pkg_dir / "__init__.py").write_text(Templates.CLI_WRAPPER.format(meta=meta))

    if (readme := Path("README.md")).exists():
        (out_dir / "README.md").write_text(readme.read_text())

    print(f"Generated {meta.name} v{meta.version} Python wrapper in .release/python/")


if __name__ == "__main__":
    main()
