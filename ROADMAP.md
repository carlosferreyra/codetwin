# CodeTwin Roadmap

> **Vision**: A zero-config, language-agnostic CLI that turns any git repository into high-quality
> visual documentation — useful from first `git clone` to ongoing refactoring.

This roadmap supersedes the previous incremental plan. The old plan grew organically and accreted
layouts/concepts that never paid their keep; we're starting from a clean architectural contract
instead.

> **History**: The first attempt at consolidation
> ([`feat/phase1-scaffold`](https://github.com/carlosferreyra/codetwin/tree/feat/phase1-scaffold))
> tried to bolt a `CodeModel` onto the existing `Blueprint` IR. It was dropped because the two
> contracts disagree on what "a module" is and the glue layer was accumulating workarounds faster
> than features. This v2 roadmap rebuilds the core from scratch so later phases plug in cleanly.

---

## Architecture Principles

- **Zero-config by default**: `codetwin gen` works out of the box; `codetwin.toml` is an optional
  power-user knob.
- **Driver auto-detection**: languages are detected via manifest files (`Cargo.toml`,
  `pyproject.toml`, `package.json`, `go.mod`, ...).
- **DRY**: prefer existing crates (`tree-sitter`, `petgraph`, `ignore`, ...) over bespoke
  implementations.
- **Plugin seams**: layouts and drivers are traits behind a registry — adding one never touches the
  other.
- **Mermaid-first for MVP**: inline Mermaid in Markdown; HTML/PlantUML/D2 are post-MVP extension
  points.
- **Dogfooding as a CI gate**: CodeTwin generates its own docs on every push; the binary is its
  first user.
- **Deterministic output**: the same `(CodeModel, Config)` must render byte-identical output. This
  unlocks snapshot tests and diffs.

---

## Terminology

| Roadmap term               | Scrum equivalent          |
| -------------------------- | ------------------------- |
| **Phase** (integer)        | Epic                      |
| **Step** (lowercase alpha) | Backlog item / User story |
| **Task** (hyphen)          | Task                      |

Every `TODO(Phase N.x)` comment in the codebase points to a step in this document.

---

## Phase 0 — Workspace migration

> Decompose the monolith into a uv-style Cargo workspace before Phase 1 work resumes. Pre-alpha —
> destructive moves are allowed.

The target shape lives under [`crates/`](crates/) (see [`crates/README.md`](crates/README.md) for
the full index and dependency DAG):

| Crate               | Owns                                                             |
| ------------------- | ---------------------------------------------------------------- |
| `codetwin`          | Bin only (`main.rs`)                                             |
| `codetwin-cli`      | `clap` surface + dispatch                                        |
| `codetwin-pipeline` | Discover → parse → merge → render → write; snapshot; diff; watch |
| `codetwin-render`   | Markdown / Mermaid + `Layout` trait + layouts                    |
| `codetwin-drivers`  | `Driver` trait + per-language tree-sitter drivers                |
| `codetwin-config`   | `codetwin.toml` schema + loader                                  |
| `codetwin-ir`       | `CodeModel`, `Module`, `Symbol`, `Edge`                          |
| `codetwin-legacy`   | Transitional monolith — emptied during Phase 0                   |

### a. Skeleton (done)

- [x] Root `Cargo.toml` is workspace-only (no `[package]`).
- [x] `[workspace.package]` and `[workspace.dependencies]` centralise version and external dep pins.
- [x] `crates/codetwin/` holds the bin (`main.rs`); depends on `codetwin-legacy` until Phase 0
      completes.
- [x] `crates/codetwin-{ir,config,drivers,render,pipeline,cli}/` exist as empty shells with intent
      docs and pre-wired internal deps.
- [x] `crates/codetwin-legacy/` holds the existing `lib.rs` and integration tests verbatim, renamed
      to `codetwin_legacy`.
- [x] `dist-workspace.toml` targets `cargo:crates/codetwin`.
- [x] `cargo build --workspace`, `cargo clippy --all-targets -- -D warnings`, `cargo test --all` all
      green.

### b. Move `ir/` → `codetwin-ir`

- [ ] Lift `codetwin-legacy::ir` into `codetwin-ir`. Update legacy re-exports so call sites keep
      compiling.
- [ ] Move `tests/ir.rs` to `crates/codetwin-ir/tests/`.

### c. Move `config/` → `codetwin-config`

- [ ] Lift `codetwin-legacy::config` into `codetwin-config`.
- [ ] Move `tests/config.rs` to `crates/codetwin-config/tests/`.

### d. Move `drivers/` → `codetwin-drivers`

- [ ] Lift `codetwin-legacy::drivers` into `codetwin-drivers`.
- [ ] Move `tests/drivers.rs` to `crates/codetwin-drivers/tests/`.

### e. Move `render/` + `layouts/` → `codetwin-render`

- [ ] Lift `codetwin-legacy::{render,layouts}` into `codetwin-render`.
- [ ] Move `tests/layouts.rs` to `crates/codetwin-render/tests/`.

### f. Move `pipeline/`, `snapshot/`, `diff/`, `watch/` → `codetwin-pipeline`

- [ ] Lift the four modules into `codetwin-pipeline`.
- [ ] Move `tests/{pipeline,snapshot,diff}.rs` to `crates/codetwin-pipeline/tests/`.
- [ ] Fold `util/` into wherever it is actually used; do not promote it to a cross-cutting crate.

### g. Move `cli/` → `codetwin-cli`; retire `codetwin-legacy`

- [ ] Lift `codetwin-legacy::cli` into `codetwin-cli`.
- [ ] Switch `crates/codetwin/Cargo.toml` to depend on `codetwin-cli` instead of `codetwin-legacy`.
      Update `main.rs` import.
- [ ] Move `tests/cli.rs` to `crates/codetwin-cli/tests/`.
- [ ] Delete `crates/codetwin-legacy/` and its workspace entry.

Each step (b–g) is a single commit. Tree must stay green at every step.

---

## Phase 1 — Core Engine Redesign

> Rebuild the internal pipeline so every future feature plugs in cleanly. Path references below are
> written against the post-Phase-0 layout. Until a module has migrated out, it still lives under
> `crates/codetwin-legacy/src/<old-path>/`.

### a. Define the IR (`CodeModel`)

- [x] Sketch `CodeModel`, `Module`, `Symbol`, `Edge`, `Visibility` enums (`codetwin-ir`).
- [ ] Extend `Symbol` with a structured `Signature` (parameters + return) once a layout needs it.
- [ ] Confirm JSON round-trips cleanly (`serde_json`) — covered by `tests/ir.rs`.
- [x] Include dependency edges (`Edge`) as first-class data (not derived).

### b. Driver trait + auto-detection registry

- [x] `Driver` trait: `detect`, `parse`, `name` (`codetwin-drivers`).
- [x] `DriverRegistry` with the four built-ins (`rust`, `python`, `typescript`, `go`).
- [ ] Port existing tree-sitter Rust extraction to produce a real `CodeModel`.
- [ ] Port existing tree-sitter Python extraction to produce a real `CodeModel`.
- [ ] Multi-language integration test (polyglot fixture).

### c. Layout trait + registry

- [x] `Layout` trait + `OutputFile` + `LayoutRegistry` (`codetwin-render`).
- [x] Register `project-overview`, `architecture-map`, `c4`, `metrics` (last two scaffolded).
- [ ] Implement layout-composition helpers so one layout can delegate sections to another.

### d. Pipeline orchestration

- [x] Discover → detect drivers → parse in parallel (rayon) → merge → render → write
      (`codetwin-pipeline`).
- [x] `--dump-ir` / `--json` dump the merged `CodeModel`.
- [ ] Replace naive merge with de-duplication by `(module_id, symbol_name)`.
- [ ] Preserve the ephemeral-flag + `--save` semantics (persist to `codetwin.toml`).

### e. Configuration v2

- [x] Optional `codetwin.toml` (`Config::load_or_default`), `deny_unknown_fields`.
- [x] Schema: `source_dirs`, `output_file`, `layout`, `format`, `exclude_patterns`, `layers`,
      `drivers`.
- [ ] `codetwin init --force`.
- [x] Global `--watch` flag on `gen`, `snapshot`, `diff`.
- [ ] Apply `exclude_patterns` (glob) on top of `.gitignore` during discovery.

---

## Phase 2 — MVP Layouts

> Two layouts that cover the two primary audiences.

### a. `project-overview`

- Target: a developer who just ran `git clone`.
- Sections: project summary, module table, dependency diagram (Mermaid), numbered data-flow
  narrative, quick-start dev guide, "key files".
- Output: single Markdown file, < 300 lines for typical projects.
- Auto-detect entrypoints (`main`, `lib`, `index`, `__main__`) and trace outward.

### b. `architecture-map`

- Target: architect / tech lead reviewing system structure.
- Sections: high-level Mermaid diagram, layer breakdown, per-layer module detail, coupling metrics
  (fan-in / fan-out), circular-dependency warnings (via `petgraph::algo::tarjan_scc`).
- Manual layer config via `[[layers]]`; auto-detect when absent.

### c. Shared render helpers

- [x] `render::markdown::MarkdownBuilder` skeleton.
- [x] `render::mermaid::graph_td` skeleton.
- [ ] Table, collapsible `<details>`, code-fence, and subgraph helpers.

### d. Integration tests

- Snapshot-style assertions for every layout against fixture repos.
- Regression tests: assert output is deterministic.

---

## Phase 3 — Dogfooding & Quality Gate

### a. Self-documentation CI job

- CI step runs both MVP layouts against the CodeTwin repo; non-zero exit fails the build.
- Generated docs are committed to `docs/` on every merge.
- Diff against the previous commit's output; flag unexpectedly large changes.

### b. Manual validation checklist

- `docs/VALIDATION.md` with a reviewer checklist.
- Track per-release validation in `docs/VALIDATION_LOG.md`.

### c. Fixture-based functional tests

- Fixtures for: minimal Rust, minimal Python, polyglot, monorepo. Assert end-to-end output per
  layout.

---

## Phase 4 — Architecture Diff

### a. Snapshot capture

- `codetwin snapshot [--ref <COMMIT>] [--watch]` writes `.codetwin/snapshots/<short>.json`.

### b. Diff engine

- `codetwin diff [REF_A] [REF_B]` (defaults: last snapshot → working tree).
- Computes added / removed / renamed modules, changed edges, changed public-API surface.
- Ignores cosmetic changes (comments, formatting, ordering).

### c. Diff output

- Markdown with summary + side-by-side before/after Mermaid + change list.
- Color-coded nodes (green / red / yellow) via Mermaid `classDef`.
- `--json` for programmatic consumption.

### d. Integration tests

- Fixture with two commits; assert the diff captures exactly the expected deltas.

---

## Phase 5 — Additional Drivers

### a. TypeScript driver

- `tree-sitter-typescript`; extract classes, interfaces, functions, exports, type aliases, generics.
- Detect via `tsconfig.json` or `package.json` with a `typescript` dependency.

### b. Go driver

- `tree-sitter-go`; extract structs, interfaces, functions, methods, packages.
- Detect via `go.mod`.

### c. Driver contribution guide

- `docs/CONTRIBUTING_DRIVERS.md` — "a new driver should be ~1 file + 1 fixture".

---

## Phase 6 — Advanced Layouts & Features

### a. C4 Model layout

- System Context (manual), Container (auto), Component (auto), Code (symbols).

### b. Coupling & metrics layout

- Coupling matrix, hub modules, circular-dep report, module size distribution.
- `--json` for CI integration (fail build if thresholds exceeded).

### c. Mermaid theming

- `theme` config key: `default`, `dark`, `forest`, `neutral`, or custom overrides.

### d. Multi-file output

- `--multi-file`: one file per module/layer, plus an index file.

---

## Phase 7 — HTML & Interactive Output

### a. HTML renderer

- `--format html`; static single-page HTML with inline Mermaid SVG.

### b. Interactive features

- Clickable nodes, search/filter, hover tooltips, collapsible sections.

### c. Live preview

- `codetwin gen --format html --watch` serves a local HTTP page with live reload.

---

## Phase Summary

| Phase | Focus                                                 | Audience               |
| ----- | ----------------------------------------------------- | ---------------------- |
| 1     | Core engine redesign (IR, drivers, layouts, pipeline) | Foundation             |
| 2     | MVP layouts: project-overview + architecture-map      | Developers, architects |
| 3     | Dogfooding CI gate + test suite                       | Quality assurance      |
| 4     | Architecture diff between commits                     | Developers refactoring |
| 5     | Additional language drivers                           | Polyglot repos         |
| 6     | Advanced layouts, metrics, theming                    | Power users            |
| 7     | HTML & interactive output                             | Teams, presentations   |

---

## CLI Surface

```text
codetwin gen [OPTIONS]
    --layout <NAME>          project-overview | architecture-map | c4 | metrics
    --output <PATH>          Output file path
    --format <FMT>           markdown (default) | html (Phase 7)
    --source <DIR>...        Override source directories
    --exclude <PATTERN>...   Additional exclude patterns
    --drivers <NAME>...      Override auto-detected drivers
    --dump-ir                Dump the merged CodeModel as JSON
    --multi-file             One output file per module/layer (Phase 6.d)
    --watch                  Re-run on filesystem changes
    --save                   Persist flag values to codetwin.toml

codetwin init [--force]

codetwin snapshot [--ref <COMMIT>] [--watch]

codetwin diff [REF_A] [REF_B] [--watch]

codetwin list [--drivers] [--layouts]
```

Global flags (on every subcommand): `--verbose`, `--quiet`, `-C/--cwd`, `--json`.

> **Entrypoint `codetwin` is immutable** — already published on crates.io, PyPI, and npm.

---

## What Stays From Current Implementation

- **Release pipeline**: `cargo release` + `git-cliff` + `cargo-dist` + PyPI/npm wrappers remain
  unchanged.
- **Config shape**: optional `codetwin.toml`, ephemeral flags + `--save`.
- **Core crates**: `tree-sitter`, `petgraph`, `rayon`, `serde`, `clap`, `anyhow`, `tracing`,
  `walkdir`, `ignore`, `glob`, `notify-debouncer-mini`.

## What Changes

- **Repo layout rebuilt** as a uv-style Cargo workspace under `crates/` (see Phase 0 and
  [`crates/README.md`](crates/README.md)).
- **IR rebuilt** as `CodeModel` with dependency edges as first-class data (no more `Blueprint`).
- **Driver system** formalised as trait + auto-detection registry.
- **Layout system** formalised as trait + registry; the three pre-pivot layouts are dropped in
  favour of two purpose-built MVP layouts.
- **`watch` subcommand** removed; `--watch` is a global flag on `gen`, `snapshot`, `diff`.
- **New subcommands**: `snapshot`, `diff`.
- **`list` subcommand** extended with `--drivers` / `--layouts`.
- **Minimum Rust version** bumped to match the `edition = "2024"` toolchain.

---

## How to Contribute

1. Pick a `TODO(Phase N.x)` comment in the source — every one corresponds to a concrete step above.
2. Add a test under `tests/` (see [`tests/README.md`](tests/README.md) for the cheatsheet).
3. Run `cargo fmt --all && cargo clippy --all-targets -- -D warnings && cargo test --all`.
4. Open a PR against `dev`; keep each PR scoped to a single step.
