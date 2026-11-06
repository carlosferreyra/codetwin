# Repository consolidation

Plan recorded on 2026-11-06, starting from `main` at `9c41fbc`.

## Branch and work disposition

| Item | Evidence | Disposition |
| --- | --- | --- |
| Local and remote `main` | Both point to `9c41fbc` before consolidation | Retain as the sole branch; commit and push verified local progress here |
| Local `codex/migrate-rust-template` at `8a581d5` | Ancestor of `main`, with no unique commits | Delete after consolidation is pushed |
| Remote branches | GitHub and `git ls-remote` list only `main` | Retain `main`; prune obsolete tracking refs |
| Pull requests | No open PRs; existing PRs #1–#5 are merged | No outstanding PR work |
| Working-tree visualization slice | Implementation, acceptance tests, domain glossary, plans and generated pages | Preserve, verify and commit together |
| Duplicate roadmap and baseline design documents | Older content predates the local visualization slice | Replace duplicate roadmap with a pointer; label baseline evidence and update current status |
| Release wrapper scripts | Read root package fields removed by workspace migration | Load the primary package and inherited workspace metadata; verify generated wrappers without publishing |
| Development Python lockfile | `uv.lock` still described Python >=3.8 and version 0.1.0 despite the development manifest requiring >=3.12 and version 0.1.10 | Refresh to match the existing development manifest; release versions come from Cargo metadata |
| crates.io packaging | Private path-only product dependencies have no publishable version requirements | Keep explicitly blocked pending publication-policy design; do not change crate visibility as housekeeping |

## Execution and verification

1. Reconcile status, architecture and contributor documentation with the implementation.
   Keep historical design evidence clearly dated and retain legacy TODOs as backlog.
2. Repair wrapper metadata loading and add regression coverage for the actual workspace layout.
3. Run `cargo xtask ci`, the ignored legacy generation test, wrapper generation checks and
   `git diff --check`. Regenerate the Visualization Set and verify its links and repeatability.
4. Review and commit all intended progress, push `main`, then delete the merged local branch.
5. Read back local/remote branches, working-tree status and open PRs. Success means only `main`
   remains locally and on GitHub, the working tree is clean, and no open PR is abandoned.

## Remaining product work

Workspace inventory and Markdown package views are implemented. Source-level extraction,
useful legacy layouts, snapshot capture and structural comparison remain incomplete.
The five successor crates are scaffolds, not implemented services. The older decomposition
is a proposal, not an instruction to create more empty boundaries. Publishing packages or a
new release is outside this consolidation.

## Local validation (2026-11-06)

- `cargo xtask ci`: formatting, compilation, Clippy, workspace tests and documentation passed.
- Explicit ignored legacy generation test passed.
- Wrapper fixture regression passed; both actual workspace wrappers generate `codetwin` 0.2.0.
- Regeneration produced ten byte-identical Markdown pages; all 78 links resolve. Inventory
  contains nine packages and eleven declared local dependencies.
- `git diff --check` passed.

The Python 3.12-specific probe could not run because that interpreter is not installed;
wrapper checks ran with installed Python 3.15.2. No tools were installed. An initial invocation
of the Python script with `uv run --locked` was rejected because the inline script has no script
lockfile; the documented `uv run scripts/release_pypi.py` invocation succeeded.
