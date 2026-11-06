# codetwin

> Zero-config, language-agnostic CLI that turns any git repository into high-quality visual
> documentation — useful from first `git clone` to ongoing refactoring.

> **Status**: the first working slice inventories Rust workspace packages and declared local
> dependencies, then generates a Markdown overview with linked crate pages. Deeper source
> parsing and the legacy language drivers remain scaffolded.

---

## Install

For the current development version, use the checkout commands below. Registry packages may
represent older releases; the current workspace is not ready for crates.io publication because
its internal product dependencies are private and path-only. npm and PyPI wrapper generation
is configured, but a successful wrapper build does not establish release availability.

---

## Quick start

```bash
# Inspect this workspace without writing files.
cargo run -p codetwin -- inspect . --json

# Generate the overview and linked crate pages.
cargo run -p codetwin -- visualize . --output docs/code

# Analyze another checkout; relative output is rooted in its Cargo workspace.
cargo run -p codetwin -- visualize /path/to/repository --output docs/code
```

Open [the generated CodeTwin overview](docs/code/index.md) to explore this repository.
Each crate page shows its targets, manifest/source links, local dependencies, and dependents.
Isolated workspace members are included. Dependency arrows point from dependent to dependency;
dev/build dependencies, aliases, optional flags, and target conditions retain their qualifications.

`inspect` without `--json` prints a concise inventory. Cargo must be available; analysis uses
`cargo metadata --no-deps --offline --locked` without compiling the analyzed repository or fetching
dependencies. The graph includes workspace members and their declared local dependencies,
not external packages, resolved feature activation, or runtime calls. Members and target sources
must be inside the workspace root. JSON paths are relative to that root.

`visualize` defaults to `docs/code`, relative to the analyzed Cargo workspace root, and generates
`index.md` plus detail pages under `crates/`. Repeated runs refresh pages bearing CodeTwin's ownership comment.
Conflicts with user-authored files, non-directory parents, and symlink destinations are refused
before writing; unrelated files are preserved. Source links are relative to each page and URL-encoded.
`visualize --json` fails with guidance to use `inspect --json`.

See [the smoke tests and acceptance cases](docs/SMOKE_TESTS.md) for reproducible checks.

### Legacy commands

The existing command routes remain available; their source extraction is still scaffolded:

```bash
# Zero-config run — writes docs/architecture.md with the project-overview layout.
codetwin gen

# Inspect what CodeTwin detected.
codetwin list --drivers --layouts

# Re-render on every filesystem change.
codetwin gen --watch

# Dump the intermediate representation as JSON.
codetwin gen --dump-ir > codemodel.json
```

`snapshot` and `diff` remain registered compatibility routes, but capture and comparison are
not implemented. `init` currently overwrites configuration even without `--force`; `gen --save`
and `--multi-file` are incomplete. Legacy drivers emit empty models.

Global flags: `--verbose`, `--quiet`, `-C/--cwd`, `--json` where supported.
For `inspect` and `visualize`, `-C` sets the base for resolving the repository path.
Legacy commands retain their existing working-directory behavior; invoking no subcommand still runs `gen`.

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

This configuration applies to the legacy commands. `inspect` and `visualize` use their
repository and output arguments.

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
cargo xtask check       # formatting, compilation, and Clippy
cargo xtask test        # check, then workspace tests
cargo xtask build       # test, then release build
cargo run -p codetwin -- list
```

`build` includes `test`, and `test` includes `check`; choose the command needed for the task.
CI runs `cargo xtask ci`, which includes tests and workspace documentation. These commands use
installed tools without installing or upgrading them.

Product crates live under `crates/`; private repository automation lives under `tools/xtask`.
Use `cargo xtask --help` and `cargo xtask scaffold --help` to inspect available commands. Add
crates only when a roadmap-backed responsibility needs a separate interface; the existing
empty crates remain transitional until the system design is settled.

```bash
cargo xtask scaffold crate <semantic-name> --dry-run
cargo xtask scaffold ci --dry-run
cargo xtask doctor
```

Scaffolds refuse to overwrite conflicting user-owned files. Optional tool installation is
explicit: `cargo xtask tools sync <group>` installs the latest stable tools under `.xtask/tools`
and updates the shared stable Rust toolchain. CodeTwin's declared MSRV remains 1.93.

See [CONTRIBUTING.md](CONTRIBUTING.md) for the development workflow and
[`crates/codetwin-legacy/tests/README.md`](crates/codetwin-legacy/tests/README.md) for the testing
cheatsheet. The template snapshot and migration decisions are recorded in
[`docs/TEMPLATE_MIGRATION.md`](docs/TEMPLATE_MIGRATION.md).

### Release pipeline

`cargo release` → `git-cliff` → `cargo-dist`, with PyPI and npm wrapper workflows publishing
after GitHub Releases. This is out of roadmap scope.

---

## Repository

- GitHub: <https://github.com/carlosferreyra/codetwin>
- Roadmap: [`ROADMAP.md`](ROADMAP.md)
- Consolidation: [`docs/CONSOLIDATION.md`](docs/CONSOLIDATION.md)
- Changelog: [`CHANGELOG.md`](CHANGELOG.md)

## License

MIT
