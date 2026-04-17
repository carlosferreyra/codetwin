# CodeTwin Roadmap v2

> **Vision**: A zero-config, language-agnostic CLI that turns any git repository into high-quality
> visual documentation — useful from first `git clone` to ongoing refactoring.

---

## Architecture Principles

- **Zero-config by default**: `codetwin gen` works out of the box; `codetwin.toml` exists for power
  users
- **Driver auto-detection**: detect project languages via manifest files (`Cargo.toml`,
  `pyproject.toml`, `package.json`, `go.mod`, etc.)
- **DRY**: prefer existing crates over custom implementations; mix when needed
- **Future-proof plugin seams**: layouts and drivers are traits behind a registry — adding one never
  touches another
- **Mermaid-first for MVP**: inline Mermaid in Markdown; other renderers (PlantUML, D2, HTML) are
  post-MVP extension points
- **Dogfooding as CI gate**: codetwin generates its own docs; quality is validated manually,
  generation is automated

---

## Terminology

| Roadmap term               | Scrum equivalent          |
| -------------------------- | ------------------------- |
| **Phase** (integer)        | Epic                      |
| **Step** (lowercase alpha) | Backlog item / User story |
| **Task** (hyphen)          | Task                      |

---

## Phase 1 — Core Engine Redesign

> Rebuild the internal pipeline so every future feature plugs in cleanly.

### a. Define the IR (Intermediate Representation) contract

- Design a `CodeModel` trait/struct that all drivers produce and all layouts consume
- Support: modules, symbols (functions, types, traits/interfaces, constants), imports/exports,
  visibility, doc comments
- Include dependency edges as first-class data (not derived later)
- Evaluate existing tree-sitter IR; refactor or replace to match the new contract
- Add `Serialize`/`Deserialize` so the IR can be cached to disk as JSON

### b. Driver trait and auto-detection registry

- Define a `Driver` trait: `detect(project_root) -> bool`, `parse(paths) -> Vec<CodeModel>`
- Implement a `DriverRegistry` that scans for manifest files and activates matching drivers
- Detection priority table: `Cargo.toml` -> Rust, `pyproject.toml`/`setup.py` -> Python,
  `package.json`/`tsconfig.json` -> TypeScript, `go.mod` -> Go, etc.
- Support multi-language repos (multiple drivers active simultaneously)
- Port existing Rust driver to the new trait
- Port existing Python driver to the new trait
- Add integration test: driver registry correctly detects languages in a polyglot fixture

### c. Layout trait and registry

- Define a `Layout` trait: `render(models: &[CodeModel], config: &Config) -> Vec<OutputFile>`
- `OutputFile` = `{ path, content, format }` where format is `Markdown` (MVP) or future `Html`
- Implement a `LayoutRegistry` with `get(name) -> Box<dyn Layout>`
- Ensure layouts are stateless and composable — one layout can delegate sections to another

### d. Pipeline orchestration

- Redesign the generation pipeline:
  `discover files -> select drivers -> parse in parallel (rayon) -> merge CodeModels -> select layout -> render -> write`
- Each stage is a standalone function, testable in isolation
- Add `--json` flag that dumps the merged `CodeModel` as JSON (useful for external tooling and
  debugging)
- Preserve ephemeral flag / `--save` semantics from current CLI

### e. Configuration v2

- Keep `codetwin.toml` optional; `Config::defaults()` covers 80%+ of projects
- Schema: `source_dirs`, `output_file`, `layout`, `exclude_patterns`, `layers` (optional), `drivers`
  (optional override of auto-detection)
- `codetwin init` remains idempotent
- Add `--watch` as a global flag available on any subcommand (replaces top-level `watch` subcommand)
- Respect `.gitignore` and nested ignore files during discovery

---

## Phase 2 — MVP Layouts

> Ship two layouts that cover the two primary audiences: the developer who just cloned a repo, and
> the architect reviewing the system.

### a. Layout 1: Project Overview

- Target audience: a developer who just ran `git clone` and wants to understand the repo
- Sections: project summary, module table (module | purpose | key symbols), dependency diagram
  (Mermaid), data flow narrative (numbered steps from entrypoint to output), quick-start dev guide
- Output: single Markdown file, < 300 lines for typical projects
- Auto-detect entrypoint(s) (`main`, `lib`, `index`, `__main__`) and trace the call graph outward
- Include a "key files" section so a newcomer knows where to look first

### b. Layout 2: Architecture Map

- Target audience: architect, tech lead, or DevOps engineer reviewing system structure
- Sections: high-level system diagram (Mermaid), layer breakdown with inter-layer dependency arrows,
  module detail per layer (types, functions, visibility), coupling metrics (fan-in / fan-out per
  module), circular dependency warnings
- Support manual layer config via `codetwin.toml` `[[layers]]` with glob patterns
- Auto-detect layers from directory structure when no config provided
- Output: single Markdown file, can be longer — completeness over brevity

### c. Shared layout utilities

- Extract reusable Mermaid generation helpers (graph builder, subgraph builder, styling)
- Extract reusable Markdown section builders (tables, collapsible details, code blocks)
- These utilities become the foundation for future layouts

### d. Layout integration tests

- For each layout: generate output against the codetwin repo itself (dogfooding)
- Snapshot tests: assert output structure (section headers, Mermaid block presence, table format)
- Regression tests: assert output is deterministic (same input = same output)

---

## Phase 3 — Dogfooding & Quality Gate

> Use codetwin on itself as a CI gate and manual validation workflow.

### a. Self-documentation CI job

- Add a CI step that runs `codetwin gen --layout project-overview` and
  `codetwin gen --layout architecture-map` against the codetwin repo
- Fail CI if the command exits non-zero
- Commit generated docs to `docs/` so they are always up to date in the repo
- Compare generated output against previous commit's output; flag if diff is unexpectedly large
  (guard against regressions)

### b. Manual validation checklist

- Create a `docs/VALIDATION.md` template with a checklist for manual doc review
- Checklist items: accuracy of module descriptions, correctness of dependency arrows, readability of
  data flow narrative, Mermaid renders correctly on GitHub, no stale/missing modules
- After each release, reviewer checks the generated docs against the checklist
- Track validation history in `docs/VALIDATION_LOG.md` (date, version, pass/fail, notes)

### c. Fixture-based functional tests

- Create fixture repos: minimal Rust project, minimal Python project, polyglot (Rust + Python),
  monorepo with workspaces
- For each fixture: run `codetwin gen` with each layout, assert expected sections exist, assert
  Mermaid syntax is valid, assert no panics or errors
- These tests validate the CLI end-to-end, not just internal functions

---

## Phase 4 — Architecture Diff

> Let developers see how the architecture changed between two commits — invaluable for refactoring.

### a. Snapshot capture

- Add `codetwin snapshot` subcommand: generates and caches the `CodeModel` JSON for a given
  commit/ref
- Store snapshots in `.codetwin/snapshots/<commit-short>.json`
- `--watch` flag available: auto-snapshot on file changes

### b. Diff engine

- Add `codetwin diff <ref-a> <ref-b>` subcommand (defaults: `ref-a` = last snapshot, `ref-b` =
  working tree)
- Compute structural diff: added/removed/renamed modules, changed dependencies, changed public API
  surface
- Ignore cosmetic changes (comments, formatting, reordering)

### c. Diff output

- Markdown report with: summary of changes, before/after Mermaid diagrams side by side, list of
  added/removed/modified modules with detail
- Color-coded Mermaid nodes: green = added, red = removed, yellow = modified (using Mermaid class
  styles)
- `--json` flag for programmatic consumption

### d. Diff integration tests

- Fixture: two commits of a test repo with known structural changes
- Assert diff output captures exactly the expected additions, removals, modifications
- Assert Mermaid syntax is valid in diff output

---

## Phase 5 — Additional Drivers

> Expand language support via the pluggable driver system established in Phase 1.

### a. TypeScript driver

- Implement `Driver` trait for TypeScript using `tree-sitter-typescript`
- Extract: classes, interfaces, functions, imports/exports, type aliases, generics
- Detect via `tsconfig.json` or `package.json` with TypeScript dependency
- Integration test with a TypeScript fixture project

### b. Go driver

- Implement `Driver` trait for Go using `tree-sitter-go`
- Extract: structs, interfaces, functions, methods, imports, packages
- Detect via `go.mod`
- Integration test with a Go fixture project

### c. Driver contribution guide

- Document how to add a new language driver: trait to implement, detection hook, fixture to create,
  tests to write
- Keep the barrier to contribution low — a new driver should be ~1 file + 1 fixture

---

## Phase 6 — Advanced Layouts & Features

> Extend the layout system with richer documentation strategies.

### a. C4 Model layout

- Implement C4 levels: System Context (manual config), Container (auto-detect crates/packages),
  Component (auto-detect modules/namespaces), Code (existing symbol-level detail)
- Generate one section per C4 level in a single Markdown file
- Support C4-PlantUML syntax as opt-in if Mermaid is insufficient (post-MVP)

### b. Coupling & metrics layout

- Dedicated layout focused on code health: coupling matrix, hub modules (high fan-in/fan-out),
  circular dependency report, module size distribution
- Output as Markdown tables + Mermaid charts
- `--json` flag for CI integration (fail build if coupling exceeds threshold)

### c. Mermaid theming

- Support `theme` config key in `codetwin.toml`: `default`, `dark`, `forest`, `neutral`
- Support custom color overrides per layer or module group
- Apply theme via Mermaid `%%{init:}%%` directive

### d. Multi-file output mode

- Add `--multi-file` flag: generate one file per module/layer instead of a single file
- Generate an index file linking to all sub-files
- Useful for large projects where a single file would be unwieldy

---

## Phase 7 — HTML & Interactive Output

> Move beyond Markdown for richer experiences.

### a. HTML renderer

- Add `--format html` flag
- Render Mermaid diagrams as inline SVG (via mermaid-js or `mermaid-rs`)
- Static single-page HTML with embedded CSS — no server required

### b. Interactive features

- Clickable diagram nodes that expand module detail
- Search/filter by module, layer, or symbol name
- Hover tooltips with function signatures and doc comments
- Collapsible sections for large projects

### c. Live preview with watch

- `codetwin gen --format html --watch` opens a local HTTP server with live reload
- File changes trigger re-generation and browser refresh

---

## Phase Summary

| Phase | Focus                                                 | Audience               |
| ----- | ----------------------------------------------------- | ---------------------- |
| 1     | Core engine redesign (IR, drivers, layouts, pipeline) | Foundation             |
| 2     | MVP layouts: Project Overview + Architecture Map      | Developers, architects |
| 3     | Dogfooding CI gate + test suite                       | Quality assurance      |
| 4     | Architecture diff between commits                     | Developers refactoring |
| 5     | Additional language drivers (TS, Go, ...)             | Multi-language repos   |
| 6     | Advanced layouts, metrics, theming                    | Power users            |
| 7     | HTML & interactive output                             | Teams, presentations   |

---

## CLI Surface (Redesigned)

```
codetwin gen [OPTIONS]
    --layout <NAME>          Layout to use (project-overview, architecture-map, c4, metrics)
    --output <PATH>          Output file path
    --format <FMT>           Output format: markdown (default), html (Phase 7)
    --source <DIR>...        Override source directories
    --exclude <PATTERN>...   Additional exclude patterns
    --drivers <NAME>...      Override auto-detected drivers
    --json                   Dump CodeModel as JSON instead of rendering a layout
    --multi-file             One output file per module/layer (Phase 6)
    --watch                  Re-generate on file changes
    --save                   Persist flag values to codetwin.toml

codetwin init [OPTIONS]
    --force                  Overwrite existing codetwin.toml

codetwin snapshot [OPTIONS]
    --ref <COMMIT>           Git ref to snapshot (default: HEAD)
    --watch                  Auto-snapshot on file changes

codetwin diff [REF_A] [REF_B]
    --json                   Output diff as JSON
    --watch                  Re-diff on file changes

codetwin list
    --drivers                List detected language drivers
    --layouts                List available layouts
```

> **Entrypoint `codetwin` is immutable** — already published on crates.io, PyPI, and npm.

---

## What Stays From Current Implementation

- **Rust driver** (tree-sitter-based) — refactored to new `Driver` trait
- **Python driver** (tree-sitter-based) — refactored to new `Driver` trait
- **Core crates**: `tree-sitter`, `petgraph`, `rayon`, `serde`, `clap`, `anyhow`, `tracing`,
  `walkdir`, `glob`, `notify-debouncer-mini`
- **Config pattern**: zero-config + optional `codetwin.toml`, ephemeral flags + `--save`
- **Release pipeline**: `cargo release` + `git-cliff` + `cargo-dist` + PyPI/npm wrappers (unchanged,
  out of roadmap scope)

## What Changes

- **`watch` subcommand** becomes `--watch` flag on `gen`, `snapshot`, `diff`
- **IR contract** redesigned as `CodeModel` with dependency edges as first-class data
- **Driver system** formalized as trait + auto-detection registry
- **Layout system** formalized as trait + registry; current 3 layouts replaced by 2 purpose-built
  MVP layouts
- **`list` subcommand** extended to show detected drivers and available layouts
- **New subcommands**: `snapshot`, `diff`
