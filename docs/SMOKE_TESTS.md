# First dogfood slice: acceptance and smoke tests

The acceptance target is a truthful Rust workspace inventory and a navigable Markdown
Visualization Set. Source modules, types, functions, runtime calls, and feature resolution
are outside this slice.

## Automated acceptance cases

Run the executable-level cases from the workspace root:

```sh
cargo test -p codetwin --test workspace
cargo test -p codetwin-ir --test ir
```

The tests construct temporary workspaces and invoke the compiled binary. Expected fixture
relationships are specified independently of the analyzer; no installed CodeTwin binary is needed.

| Use case | Expected result |
| --- | --- |
| Binary `app`, library `model`, isolated library `unused` | Three packages, exactly `app → model`; all three diagram nodes and detail pages are present |
| Workspace-inherited renamed dependency | Edge points to the actual package; the alias is retained |
| Dev/build, optional, and target-specific dependencies | Four qualified fixture declarations appear in JSON, the diagram, and details |
| No local dependencies; names `domain-model` and `domain_model` | Explicit empty JSON edge array; distinct standalone nodes and pages |
| Dependency outside the workspace member set | Excluded from the local member graph |
| Repeated generation and equivalent workspaces at different absolute paths | Byte-identical JSON and Markdown; no absolute repository paths in the model |
| Replace `app → model` with `app → unused` | Only that relationship changes; `model` has no dependents after regeneration |
| Explicit repository, `-C`, and a member path | Same workspace model; relative output resolves from the workspace root |
| Missing repository/manifest, malformed TOML, unavailable Cargo | Nonzero exit, useful stderr, no fabricated JSON or visualization output |
| `visualize --json` | Nonzero exit with guidance to use `inspect --json` |
| User-authored destination, non-directory parent, or output symlink | Refusal before any generated file is written; user files remain intact |
| Source filename containing spaces, parentheses, and `#` | JSON preserves the source path; Markdown links URL-encode delimiters |
| Relative target path escaping the workspace | Contextual error before output; target identities cannot leak outside the selected workspace |
| Deep member paths | Bounded artifact path components; overview, detail, and source links remain valid |
| Legacy help, `list --json`, `gen --dump-ir`, and default invocation with `-C` | Existing routes still work and default generation writes its legacy output |
| Stored Code Model without new workspace fields | Loads with empty package collections; extended models round-trip and merge with source facts |

The symlink test runs on Unix. Other filesystem and CLI cases run on all platforms.
Inspection and visualization leave existing lockfiles unchanged; inspection creates neither
visualization output nor a missing lockfile.

## Dogfood this checkout

```sh
cargo run -p codetwin -- inspect .
cargo run -p codetwin -- inspect . --json
cargo run -p codetwin -- visualize . --output docs/code
```

Open [the generated overview](code/index.md). Follow a package to its detail page, then follow
its manifest and target-source links. Arrows point from dependent to dependency; `xtask` remains
visible as an isolated workspace package.

Observed after implementation on 2026-10-11:

- Nine workspace packages and eleven local dependency declarations.
- Ten Markdown files: one overview and nine crate detail pages.
- All 78 generated links resolve to files.
- Package membership and all qualified local edges match a separate
  `cargo metadata --no-deps --offline --locked --format-version 1` invocation.
- Repeated JSON and generated Markdown are byte-identical, with no absolute paths embedded.
- The repository lockfile remains unchanged by inspection and visualization.

The planning baseline had ten local dependency pairs. The eleventh is the new direct
`codetwin → codetwin-ir` dependency used by workspace analysis. Counts describe this checkout;
future manifest changes should change the graph rather than require fixed-count assertions.

For a quick JSON repeatability check without placing files in the repository:

```sh
CODETWIN_SMOKE_DIR=$(mktemp -d)
cargo run -p codetwin -- inspect . --json > "$CODETWIN_SMOKE_DIR/first.json"
cargo run -p codetwin -- inspect . --json > "$CODETWIN_SMOKE_DIR/second.json"
diff "$CODETWIN_SMOKE_DIR/first.json" "$CODETWIN_SMOKE_DIR/second.json"
```

Expected: no diff. The subprocess acceptance tests additionally check Markdown bytes,
relocation, and dependency mutation.

## Final verification

```sh
cargo xtask ci
cargo test -p codetwin-legacy --test pipeline -- --ignored
git diff --check
```

These passed locally: formatting, workspace compilation, Clippy with warnings denied,
51 ordinary tests, workspace documentation, and the explicitly enabled legacy generation test.
Compilation of all workspace targets also passed on the declared Rust 1.93 minimum version.
No new product crates or release changes were needed. Generated Markdown uses an ownership
comment for refresh; unrelated files and old unlinked pages are preserved rather than cleaning
the output directory recursively.
