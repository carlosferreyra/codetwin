# Rust template migration

Refs: Phase 0.h.

CodeTwin adopts the tooling foundation from
[`carlosferreyra/rust-template` at `62488bc`](https://github.com/carlosferreyra/rust-template/tree/62488bc7d5cf0a34501282305dedb7d32068e235).
This is an in-place migration. Existing source, product crates, release integration, and the
untracked product context are preserved.

## Adopted foundation

- Private `tools/xtask`, `.cargo/config.toml`, and workspace metadata identifying `codetwin` as
  the primary crate and product prefix.
- Resolver 3, inherited package metadata, workspace lints, and a stable toolchain with Clippy
  and rustfmt. Edition 2024, version 0.2.0, and MSRV 1.93 remain unchanged.
- Template commands for checks, tests, builds, documentation, scaffolding, tool discovery,
  explicit tool sync, and release planning.
- Lean CI rendered from the bundled action registry, running on pushes and pull requests.
- Development documentation and CLI-first guidance merged into the existing agent rules.

The action registry records stable release tags resolved during migration. New automation
dependencies are `cargo_metadata` 0.23.1 and `toml_edit` 0.25.17; the existing workspace Clap
dependency is shared with xtask. These dependencies belong to development automation, not the
CodeTwin executable. Existing product dependency requirements are preserved.

## Project adaptations

- Unsafe code is forbidden rather than merely warned about, matching CodeTwin's existing rule.
- Xtask inherits MSRV 1.93 so resolver 3 applies the project baseline to automation dependencies.
- Workflow rendering returns errors rather than using `expect`, matching the existing rule
  against panicking outside tests and startup.
- A project Clippy dictionary recognizes the CodeTwin name in prose. Independent CLI switches
  keep a targeted boolean-count exemption; other newly exposed warnings are fixed mechanically.
- Existing agent instructions, crate documentation, and release files remain user-owned;
  scaffolds refuse conflicting replacements.
- Empty transitional crates remain for a separate architecture decision. Adopting tooling does
  not complete the legacy crate decomposition or implement parsing and visualization.

Generator hooks, placeholder manifests, template test scripts, and starter application code are
not copied into CodeTwin. Full CI, dependency-policy checks, audit schedules, and Dependabot
remain optional.

## Publication boundary

The existing crates.io packaging failure remains: the published binary depends on private,
path-only `codetwin-legacy`. Resolving the dependency's publication policy belongs to the
product-crate and release design, rather than this tooling migration. No release workflows,
release scripts, wrapper versions, or publication settings are changed here.

## Verification

Validate the migration with `cargo xtask ci`, the ignored generation integration test, and a
release build. Xtask integration tests exercise member-directory discovery, dry-run preservation,
idempotent scaffolds, conflict rejection without partial writes, action substitution, and the
versioned primary CLI scaffold.

Verified during migration:

- Formatting, workspace compilation, Clippy with warnings denied, tests, and documentation through
  `cargo xtask ci`. The Mermaid regression test preserves output bytes and edge styles.
- The ignored generation integration test, invoked explicitly.
- `cargo build --workspace --release --locked` and executable smoke checks.
- `cargo +1.93.0 check --workspace --all-targets --locked` with an isolated target directory.
- Actionlint validation of the generated lean CI workflow.
- The Cargo alias from a member directory, a real-repository scaffold dry run, and idempotent CI
  regeneration. Release integration files were verified byte-identical to their original state.
