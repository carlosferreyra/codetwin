# codetwin

> Zero-config, language-agnostic CLI that turns any git repository into high-quality visual
> documentation — useful from first `git clone` to ongoing refactoring.

> **Status**: v2 architecture scaffold. Phase 1 of [`ROADMAP.md`](ROADMAP.md) is underway; the
> binary builds and runs end-to-end but drivers currently produce empty `CodeModel`s. See the
> `TODO(Phase N.x)` markers in the source for concrete work items.

---

## Install

```bash
cargo install codetwin         # crates.io
uv tool install codetwin       # PyPI wrapper (native binary under the hood)
npm install -g codetwin        # npm wrapper
```

All three package managers install the same native binary. The npm/PyPI wrappers bootstrap the
binary on first run via the `cargo-dist` installer.

---

## Quick start

```bash
# Zero-config run — writes docs/architecture.md with the project-overview layout.
codetwin gen

# Inspect what CodeTwin detected.
codetwin list --drivers --layouts

# Re-render on every filesystem change.
codetwin gen --watch

# Dump the intermediate representation as JSON.
codetwin gen --dump-ir > codemodel.json

# Capture / diff architectural snapshots between commits.
codetwin snapshot --ref HEAD~5
codetwin diff HEAD~5 HEAD
```

Global flags work on every subcommand: `--verbose`, `--quiet`, `-C/--cwd`, `--json`.

---

## Layouts

| Name                 | Audience                              | Status       |
| -------------------- | ------------------------------------- | ------------ |
| `project-overview`   | Developer who just cloned the repo    | Scaffolded   |
| `architecture-map`   | Architect reviewing the system        | Scaffolded   |
| `c4`                 | C4-model consumers                    | Phase 6.a    |
| `metrics`            | Coupling / circular-dep reporting     | Phase 6.b    |

`codetwin list --layouts` prints the live registry.

---

## Configuration (`codetwin.toml`)

Everything is optional. See the checked-in [`codetwin.toml`](codetwin.toml) for an annotated
starting point.

```toml
source_dirs      = ["src"]
output_file      = "docs/architecture.md"
layout           = "project-overview"
format           = "markdown"              # "html" is reserved for Phase 7
exclude_patterns = ["**/target/**", "**/node_modules/**"]

# Override auto-detected drivers:
# drivers = ["rust", "python"]

[[layers]]
name     = "CLI"
patterns = ["src/cli/**"]
```

---

## Development

Requires Rust 1.93+ (edition 2024).

```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test --all
cargo run -- list        # exercise the CLI
```

See [`tests/README.md`](tests/README.md) for the testing cheatsheet (TDD + non-TDD workflows).

### Release pipeline

`cargo release` → `git-cliff` → `cargo-dist`, with PyPI and npm wrapper workflows publishing
after GitHub Releases. This is out of roadmap scope.

---

## Repository

- GitHub: <https://github.com/carlosferreyra/codetwin>
- Roadmap: [`ROADMAP.md`](ROADMAP.md)
- Changelog: [`CHANGELOG.md`](CHANGELOG.md)

## License

MIT
