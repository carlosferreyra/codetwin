# First dogfood slice: implementation and verification

Status: implemented; acceptance tests and live dogfood verification recorded in
[SMOKE_TESTS.md](SMOKE_TESTS.md).
Goal: CodeTwin analyzes its own Rust workspace and produces a useful, accurate Markdown
overview with linked crate detail pages. Baseline is `9c41fbc`.

## Command and artifact contract

| Command | Result |
| --- | --- |
| `codetwin inspect [REPOSITORY] --json` | The extracted Code Model on stdout; diagnostics on stderr; no visualization files |
| `codetwin inspect [REPOSITORY]` | A concise human-readable workspace inventory |
| `codetwin visualize [REPOSITORY] --output <DIR>` | `index.md` and linked crate detail pages in the chosen directory |

Repository defaults to the current directory. Analyze the selected repository without changing
the process working directory. On new commands, global `-C` changes the path-resolution base;
resolve an explicit repository path against that base. Relative output directories resolve from its Cargo workspace root;
the default is `docs/code`. The JSON model stores repository-relative paths with `/`
separators, not absolute Cargo package IDs or machine-specific paths.

The existing `gen`, `init`, `list`, `snapshot`, and `diff` routes remain intact for this slice.
Their existing flags and default invocation behavior remain compatibility concerns handled by
the legacy route. New commands use explicit repository paths and the new analysis service.
On `visualize`, reject `--json` with a usage error pointing to `inspect --json`; it must not
silently bypass visualization. This keeps the first command contract small.

The first detail page shows package metadata, binary/library targets, manifest and target-source
links, local dependencies and dependents. The deeper module/type/function view in
[the dogfood target](DOGFOOD_TARGET.md) is a later slice, not an acceptance requirement here.

Expected artifact layout:

```text
docs/code/
  index.md
  crates/
    <stable-artifact-id>.md
```

## Implementation sequence

### 1. Establish fixtures and command tests

- Add subprocess integration tests under `crates/codetwin/tests/`; execute the compiled
  binary rather than changing the test process's working directory.
- Build a small temporary Cargo workspace containing a binary package `app`, a library
  package `model`, and an isolated library `unused`. Declare `app → model`.
- Specify exact expected package identities and relationships independently of analyzer output.
- Add failing tests for the new commands, valid JSON output, output files and invalid inputs.
- Preserve the existing suite as the compatibility baseline.

Expected test result: new tests fail because the commands do not exist; existing tests pass.
The fixture's expected graph is three packages and one edge, including the isolated package.

### 2. Extend the shared Code Model with workspace facts

- Keep `codetwin-ir` as the existing shared model boundary. Add explicit package records and
  package-dependency records rather than pretending a Cargo package is a source module.
- A package record contains a stable repository-relative manifest identity, package name,
  manifest path, and targets with their kind and source path.
- A dependency record connects package identities and preserves dependency kind, alias,
  optional status and target condition when present. Edges point from dependent to dependency.
- Add the new collections to `CodeModel`; default missing collections during deserialization
  so existing stored models remain readable. Preserve existing module and symbol fields.
- Establish a canonical order for records and dependency qualifications.

Expected test result: old JSON still loads; the extended model round-trips; packages with no
edges remain represented; equivalent inputs serialize in the same order. Existing IR and
legacy consumers continue to compile and pass their tests.

### 3. Implement Rust workspace inventory

- Add an analysis module in the primary `codetwin` crate, exposed through `src/lib.rs` for
  meaningful library tests. Reuse the root's existing `cargo_metadata` dependency declaration.
- Obtain metadata with `--no-deps --offline --locked`, using an explicit manifest path.
  This inventories members without compiling the analyzed repository or fetching dependencies.
  Rust/Cargo must be installed for this first analyzer.
- Include all workspace members, not only packages reachable from one binary. Map path
  dependencies to workspace package identities; preserve their declared qualifications.
- Exclude dependencies outside the workspace from the local graph. Keep package target kinds
  and manifest paths as facts; do not infer implementation quality from an empty graph or TODO.
- Normalize paths and ordering before returning the Code Model. Provide contextual failures
  for invalid repositories or unavailable Cargo rather than returning an empty success model.

Expected test result: the fixture produces the exact three-package graph; renamed and inherited
workspace dependencies resolve to their actual packages; dev/build/conditional relationships
retain their qualifications. Malformed manifests fail with a nonzero exit status and an error
on stderr. Inspection leaves repository inputs, including the lockfile, unchanged.

### 4. Wire `inspect` through a shared analysis service

- Add the two new command argument models and dispatch in the primary crate's CLI module.
  Reuse existing legacy argument types and dispatch for compatibility routes.
- `inspect --json` serializes the canonical model; ordinary `inspect` prints the inventory.
- Both commands will call the same analysis service, so visualization does not reimplement
  discovery or manifest interpretation. Keep CLI flags out of the model library.

Expected test result: JSON parses without log prefixes; running from another directory with
an explicit repository path gives the same model. Inspection creates no output directory.
Legacy command parsing and help still work.

### 5. Render and write the Visualization Set

- Add a visualization module in `codetwin`; consume the model without reparsing manifests.
- Produce an overview table, Mermaid package dependency graph, and links to crate details.
  Include standalone nodes for isolated crates and distinguish dependency qualifications.
- Generate one detail page per package with targets, dependencies, dependents and source links.
- Derive safe, collision-free diagram and artifact identifiers from stable package identities;
  do not rely on replacing punctuation in package names.
- Separate rendering from writing. Compute the artifact set and validate all destinations first.
  Refuse conflicts with user-owned files. Mark generated Markdown with a fixed ownership
  comment so repeat runs can recognize and refresh it. Do not recursively clean the output
  directory or remove unrelated files.
- Existing generated destinations may be refreshed. Determinism is checked at the same relative
  output location; source links are relative to each artifact's location.

Expected test result: exact fixture nodes and edges appear; every overview/detail/source link
resolves; all packages have detail pages; an isolated package remains visible. Two runs produce
byte-identical artifact contents. Different names that sanitize similarly do not collide.

### 6. Exercise the real dogfood loop and finish validation

- Run the new commands against CodeTwin and inspect the overview and detail pages manually.
- Compare membership and local relationships with Cargo metadata as an independent check of
  our mapping. Fixture assertions remain independent of metadata-derived expected output.
- Mutate a fixture dependency, regenerate, and verify that the model and diagram change.
- Copy the unchanged fixture to a different absolute location; require identical model bytes
  and artifact bytes at the same repository-relative output path.
- Update usage documentation with working examples and the actual support boundary.
- Reassess module interfaces before proposing domain-crate extraction.

Historical planning expectation at `9c41fbc` before implementation: nine workspace packages, ten distinct local
dependency pairs, and nine crate detail pages. The graph contains
`codetwin → codetwin-legacy → codetwin-ir` and the dependencies of the five successor crates.
`xtask` is included as a separate workspace package with no local dependencies. The implemented slice adds the direct `codetwin → codetwin-ir` edge, giving eleven
local dependency declarations; see [SMOKE_TESTS.md](SMOKE_TESTS.md). These baseline counts
describe the planning input; tests must follow manifest changes rather than freeze this architecture.

## Verification matrix

| Scenario | Expected outcome |
| --- | --- |
| Three-package fixture | All three packages, exactly `app → model`, no invented edges |
| Renamed/inherited local dependency | Points to the real package while preserving alias metadata |
| Dev/build/optional/conditional declarations | Visible qualifications; graph is declared dependencies, not a feature-resolved build or runtime trace |
| No local dependencies | Valid inventory and standalone diagram nodes |
| Invalid path or malformed manifest | Nonzero exit, useful stderr, no generated artifacts |
| Missing Cargo executable | Contextual error; no fallback to a fabricated empty model |
| `inspect --json` | Valid JSON only on stdout; no filesystem output or input changes |
| Explicit repository from another directory | Same model and correctly resolved repository-relative output |
| Repeated generation | Same paths and bytes; existing generated pages refresh |
| Copied repository | No absolute paths leak into stable model or presentation |
| Dependency changed in fixture | The precise changed relationship appears in JSON and Markdown |
| Artifact and diagram identifiers | Distinct packages cannot overwrite pages or merge diagram nodes |
| Unrelated file in output directory | File remains untouched |
| User-owned file at a planned destination | Refuse replacement before writing any artifacts |
| Legacy command surface | Existing tests and help checks remain green |

Use focused tests during iteration, then `cargo xtask ci` for the completed slice; it includes
formatting, workspace compilation, Clippy, tests and documentation. Run the existing ignored
generation test explicitly if compatibility changes touch that path. Finish with `git diff --check`
and review the generated output. No redundant `check`/`test`/`build` sequence is needed.

## Template workflow and scope

No new product crate is necessary for this slice. Product responsibilities begin as modules
in `codetwin`, with the existing `codetwin-ir` dependency. Keep private automation in `tools/xtask`
and share external dependency requirements through root `[workspace.dependencies]`.

Verified planning probe: `cargo xtask scaffold cli --entrypoint primary --dry-run` refuses the
existing `codetwin-cli` workspace dependency because it differs from the generated declaration.
Adapt the existing files deliberately; do not delete them or force the scaffold. For later
roadmap changes requiring new crates, preview and apply the template scaffold commands.

This slice does not implement deeper language parsing, runtime call graphs, other languages,
watching, snapshots, comparison, browser serving, release changes or automatic CI commits of
generated artifacts. The existing packaging problem remains a separate release decision.
