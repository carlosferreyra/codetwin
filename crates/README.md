# Crates

CodeTwin is organised as a Cargo workspace. Each crate has one job and a
narrow public surface. New crates are added when a real seam appears (two
adapters, not one).

## Index

| Crate                                          | Role                                                                              | Depends on                                                                |
| ---------------------------------------------- | --------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| [`codetwin`](codetwin/)                       | Published binary. `main.rs` only — parse argv, init tracing, dispatch.            | `codetwin-cli` (eventually); `codetwin-legacy` (today)                    |
| [`codetwin-cli`](codetwin-cli/)               | `clap`-derived CLI surface and subcommand dispatch.                                | `codetwin-config`, `codetwin-pipeline`                                    |
| [`codetwin-pipeline`](codetwin-pipeline/)     | Orchestration: discover → parse → merge → render → write. Owns snapshot, diff, watch. | `codetwin-ir`, `codetwin-config`, `codetwin-drivers`, `codetwin-render`   |
| [`codetwin-render`](codetwin-render/)         | Markdown / Mermaid builders and the `Layout` trait + built-in layouts.            | `codetwin-ir`                                                             |
| [`codetwin-drivers`](codetwin-drivers/)       | `Driver` trait, auto-detection registry, and per-language tree-sitter drivers.    | `codetwin-ir`                                                             |
| [`codetwin-config`](codetwin-config/)         | `codetwin.toml` schema + loader + override merging.                                | (leaf)                                                                    |
| [`codetwin-ir`](codetwin-ir/)                 | Intermediate representation: `CodeModel`, `Module`, `Symbol`, `Edge`.             | (leaf)                                                                    |
| [`codetwin-legacy`](codetwin-legacy/)         | **Transitional.** Holds the pre-workspace monolith while modules migrate out.     | (everything the monolith did)                                             |

## Dependency DAG

```
codetwin (bin)
  └── codetwin-legacy           ← today
       └── (will become)
            codetwin-cli
              └── codetwin-pipeline
                   ├── codetwin-render ──┐
                   ├── codetwin-drivers ─┼── codetwin-ir
                   └── codetwin-config   ┘
```

Edges only point downward. A leaf crate must never depend on a higher-level
one. Adding an upward edge needs an explicit ADR.

## Conventions

- **Naming**: every workspace crate is `codetwin-<role>`. The bin keeps the
  bare name `codetwin` because that is what users install.
- **Lib name**: snake_case (`codetwin_ir`, etc.) — Rust requirement.
- **Versioning**: a single workspace version pinned in the root `Cargo.toml`
  via `[workspace.package]`. Releases bump every crate together.
- **External deps**: declared once in `[workspace.dependencies]` at the root,
  inherited by members with `{ workspace = true }`. Adding a new external
  dependency is a workspace-level decision (see [AGENTS.md §7](../AGENTS.md)).
- **Internal deps**: also declared in `[workspace.dependencies]` with `path =
  "..."`. Members reference them with `{ workspace = true }`.
- **No cyclic deps**: enforced by Cargo. If you find yourself wanting one,
  you have probably split a single concept across two crates.
- **Publish flag**: only `codetwin` is published. Every other crate sets
  `publish = false` until we have a reason to expose it.

## Adding a new crate

1. Confirm the seam is real (two adapters, not one — see [AGENTS.md](../AGENTS.md)).
2. Add the crate's directory under `crates/<name>/` with `Cargo.toml` + `src/lib.rs`.
3. Register it in `[workspace.dependencies]` at the root.
4. Add a row to the table above.
5. Update the dependency DAG diagram if a new edge appears.

## Phase 0 migration status

`codetwin-legacy` is being decomposed. As modules move out:

- `ir/` → `codetwin-ir`
- `config/` → `codetwin-config`
- `drivers/` → `codetwin-drivers`
- `render/`, `layouts/` → `codetwin-render`
- `pipeline/`, `snapshot/`, `diff/`, `watch/` → `codetwin-pipeline`
- `cli/` → `codetwin-cli`
- `util/` → folded into whichever crate uses it (no cross-cutting util crate)

When `codetwin-legacy` is empty, delete it and switch the bin to depend on
`codetwin-cli`. Track in [`ROADMAP.md`](../ROADMAP.md) Phase 0.
