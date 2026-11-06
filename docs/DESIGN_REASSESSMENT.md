# CodeTwin design reassessment

Status: working proposal for discussion, not an approved replacement architecture.
Baseline: `9c41fbc` on `main`, after adopting the Rust template workflow.

Historical assessment: the observations and proposed commands below describe that baseline.
The subsequent `inspect`/`visualize` workspace slice is implemented; the executable now owns
analysis and Markdown visualization modules and the model includes package relationships.
See [the implementation record](IMPLEMENTATION_PLAN.md) and [current roadmap](../ROADMAP.md).
Broader replacement architecture and source-level detail remain proposals.

## Product contract

The current [domain glossary](../CONTEXT.md) describes this workflow:

```text
Repository → Code Model → Visualization Set
                            ├── Repository Overview
                            └── linked Detail Views
```

The developer should learn the repository's structure without reading every source file.
Markdown is the canonical presentation; diagrams and links make it navigable. The Code Model
holds source facts independently of presentation. Browser interaction remains undecided.

The existing roadmap and technical crate list are inputs to this reassessment, rather than
requirements to finish their decomposition. Dogfooding CodeTwin itself is now the agreed first
use case: as the tool improves, its own repository should become easier to understand.
The first expected overview and detail are drafted in [the dogfood target](DOGFOOD_TARGET.md).
The first workspace-level implementation and verification sequence is in
[the implementation plan](IMPLEMENTATION_PLAN.md).

## What exists today

| Area | Observed implementation | Assessment |
| --- | --- | --- |
| Template automation | Working private `tools/xtask`, scaffolding, checks, and CI | Retain |
| Executable | Thin entrypoint dispatching into `codetwin-legacy` | Retain the thin entrypoint approach |
| Product workspace | Eight product crates; five contain only intent comments | Reconsider boundaries before moving more modules |
| Code Model | Modules, symbols, module-to-module edges, serialization and append-only merge | Reuse as a starting point; verify it can express the chosen views |
| Configuration | Defaults, strict TOML loading, serialization, CLI overrides | Reuse useful mechanisms; narrow the schema to supported behavior |
| Discovery | Ignore-aware walk and sorted paths | Reuse; repository discovery, exclusions and duplicate paths need work |
| Language extraction | All four current drivers return empty models | Core analysis still needs implementation |
| Visualizations | Overview emits counts; architecture map emits a layer count | Actual overview and detail views still need implementation |
| Output helpers | Markdown builder, simple Mermaid arrows, filesystem writes | Reuse selectively; validate identifiers, ordering and links |
| Snapshots and comparison | JSON store works; capture and command comparison fail; model diff always returns no changes | Storage is reusable later; comparison is not implemented |
| Watch mode | Debounced callback loop exists | Keep as an optional adapter after one-shot generation works |

Implementation references: [CLI](../crates/codetwin-legacy/src/cli/mod.rs),
[pipeline](../crates/codetwin-legacy/src/pipeline/mod.rs),
[drivers](../crates/codetwin-legacy/src/drivers/),
[layouts](../crates/codetwin-legacy/src/layouts/),
[model](../crates/codetwin-ir/src/ir/),
[workspace index](../crates/README.md).

The published history also contains Rust and Python tree-sitter extraction under
`v0.1.16:src/drivers/{rust,python}.rs`, with parser tests. Those routines target the old
`Blueprint` representation. Treat them as salvage candidates: adapt and test selected routines,
rather than restoring the old application wholesale. They were inspected, not executed here.

## Command contract before command names

Subcommands should describe developer tasks. Internal stages such as discover, parse, merge,
and render can be independently testable functions without becoming public subcommands.

Proposed initial command surface, with names still open for discussion:

| User task | Proposed command | Result |
| --- | --- | --- |
| Understand a repository visually | `codetwin visualize [REPOSITORY] --output <DIR>` | A Markdown overview and linked detail views |
| Inspect extracted facts, including from scripts | `codetwin inspect [REPOSITORY] --json` | The Code Model, with extraction diagnostics on stderr |
| Save optional configuration | `codetwin init` | A starter configuration, refusing existing files unless explicitly forced |

`gen` could remain an alias if preserving existing invocation matters. Choosing the word
`visualize` versus `generate` is secondary to defining its artifact and behavior.
`init` is optional convenience, not a prerequisite for analysis. Bare invocation should show
help unless implicit generation is deliberately chosen; it currently writes output by default.

Snapshot storage, revision comparison, browser serving, and additional export formats should
earn commands through concrete use cases. The existing `list` command can remain a capability
query, but it should distinguish supported extraction from planned drivers and layouts.

Two important separations:

- `--json` selects a representation of a command's result. It should not silently turn
  visualization generation into a different operation. Currently global `--json` on `gen`
  bypasses rendering and emits the model, overlapping `--dump-ir`.
- Repository paths, output destinations and configuration precedence need explicit semantics.
  Resolve them at the application boundary; reusable analysis should accept a repository root
  rather than read or change the process working directory.

Invalid format names and unknown requested languages should fail explicitly. Unimplemented
flags should not claim success. Current `--save` and `--multi-file` warn and continue;
`init` ignores `--force` and can overwrite configuration. These are observed gaps, not fixes
included in this reassessment.

## Domain responsibilities before crate extraction

| Responsibility | Owns | Boundary rule |
| --- | --- | --- |
| Repository analysis | Repository inventory, source ownership, language extraction, relationship resolution, diagnostics | Produces facts without choosing presentation or writing output |
| Code Model | Stable identities, containment, source locations, relationships | Independent of Clap, filesystem discovery, Markdown and Mermaid |
| Visualizations | Overview grouping, detail selection, diagrams, links, Markdown artifacts | Consumes the model; does not reparse source |
| Application | CLI, configuration resolution, orchestration, artifact writes | Connects the responsibilities and handles developer interaction |

Start implementation in the primary `codetwin` crate, with modules for these responsibilities.
The existing `codetwin-ir` is a candidate shared model boundary; keep it where reuse helps,
and avoid a cosmetic rename before its contract is settled. Do not create replacement empty
crates for every responsibility.

If independent interfaces become useful, domain names such as `codetwin-model`,
`codetwin-analysis`, and `codetwin-visualization` express the intended ownership. They are
possible later extractions, not a proposed immediate three-crate scaffold. Keep CLI parsing,
configuration, watching and orchestration as application modules unless there is a concrete
independent consumer or dependency-isolation need. Language adapters can initially live
together; a language does not automatically need its own crate.

Any extraction should use `cargo xtask scaffold crate <semantic-name> --dry-run`, then the
appropriate scaffold command. Publication policy must be decided for actual dependencies:
the current crates.io binary depends on private, path-only `codetwin-legacy`, which prevents
packaging. Moving code into more private crates does not solve that publication boundary.

The model needs an explicit answer to what the overview groups: packages/build targets,
source modules, directory groups, or manually declared components. Package boundaries with
source-module detail are a plausible Rust starting point, not a universal language rule.
Current edges target only `ModuleId`; symbols have local names but no stable symbol identity.
That is insufficient to identify individual types or functions in symbol-level relationships.
Add those concepts only to satisfy the chosen detail view. Syntax extraction alone should
not be presented as proven runtime call flow or architecture intent.

## Recommendation and first implementation slice

Prefer a selective rebuild of the product behavior within this repository. Retain the template
foundation, history and useful utilities. Reconsider the command and model contracts before
continuing the old Phase 0 module moves. A wholesale repository reset would discard useful
work; completing the old decomposition would preserve boundaries that have not proved useful.

After specifying what the first test example should show:

1. Specify one expected overview and its linked detail view using real source facts.
2. Add a small fixture with multiple modules, an actual dependency, and an isolated component.
3. Implement one language's inventory and extraction into a tested Code Model. Evaluate old
   parser routines during this step. Route inputs by language; a packaging manifest alone
   should not make unrelated source files count as that language's analysis.
4. Generate the overview and detail artifacts, with resolvable links and source references.
5. Exercise the CLI in a subprocess against that fixture. Check exact expected relationships,
   repeated byte-identical output, invalid input errors, and that inputs remain unchanged.
6. Reassess crate boundaries using the implemented interfaces and dependency costs.

This sequence produces a useful path through the system before investing in more commands,
parallel parsing, plugin abstractions, configuration knobs or format backends.

## Evidence and open decisions

Verified at the baseline:

- `cargo xtask test`: formatting, compilation, Clippy and 34 active tests pass; one generation
  integration test is ignored by the ordinary suite. Most product tests cover scaffolding,
  defaults and serialization; the diff test cannot distinguish the current constant empty
  implementation from a working comparison engine.
- `gen --source crates/codetwin-ir/src --dump-ir`: a real source tree still produces zero
  modules and edges. The model is labelled `polyglot` through manifest detection.
- The same probe with `--drivers nonexistent` produces an empty model after a warning;
  `--format invalid` is silently ignored by CLI override parsing.
- Cargo metadata confirms that the executable uses `codetwin-legacy → codetwin-ir`; the five
  empty successor crates are not in the executable's active dependency chain.
- `cargo package -p codetwin --locked --offline` fails because the `codetwin-legacy`
  dependency lacks a version requirement. This is a publication issue separate from passing
  compilation and tests.
- Commit `9c41fbc` is pushed to GitHub, and its CI passes.

The next decision is what CodeTwin's first overview should explain about the test repository.
That determines component granularity and the required extraction. Also unresolved: how
deep detail views go, whether saved models are a user workflow, and whether existing command
names need compatibility. These decisions precede a finalized crate diagram or new roadmap.
