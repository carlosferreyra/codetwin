# Tests — Cheatsheet

A field guide for writing tests against the CodeTwin codebase. Read this before adding a new test
so the layout stays coherent.

---

## What goes where

Rust has **three first-class test locations**. We use all of them for different reasons.

| Location                | What it tests                                       | How it's run                         |
| ----------------------- | --------------------------------------------------- | ------------------------------------ |
| `tests/` (this folder)  | **Integration tests** — public API only             | `cargo test --test <name>`           |
| `src/**/*.rs` (inline)  | **Unit tests** — private functions, fast feedback   | `cargo test --lib`                   |
| `examples/`             | **Executable examples** — compiled, human-facing    | `cargo run --example <name>`        |
| Doc comments (`///`)    | **Doctests** — usage snippets in public docs        | `cargo test --doc`                   |
| `benches/` *(future)*   | **Benchmarks** — perf regressions (`criterion`)     | `cargo bench`                        |

### Rules of thumb

- **Integration tests live in `tests/`.** They import `codetwin` as an external crate and use only
  `pub` items. One file per concern (e.g. `tests/ir.rs`, `tests/pipeline.rs`) — no `mod` wrappers.
- **Unit tests live next to the code they test,** inside a `#[cfg(test)] mod tests { ... }` block
  at the bottom of the file. They can reach into private items.
- **Fixtures go in `tests/fixtures/<name>/`** — treat them as read-only inputs, never mutate in
  place; copy to a `tempfile::TempDir` first.
- **Doctests prove your public docs work.** Include at least one in every `pub fn`/`pub struct`
  whose usage is not obvious.
- **Benches are not tests** — they measure; don't put assertions in them.

### When a test belongs in `tests/` vs. inline

```text
Does the test only need pub items?
├── yes → tests/<concern>.rs      (integration)
└── no  → src/<module>.rs `mod tests` (unit)

Does the test need a filesystem fixture?
├── yes → tests/<concern>.rs      (integration)
└── no  → either is fine
```

---

## File naming

- Lowercase, snake_case.
- One concern per file; avoid `test_` prefixes (being in `tests/` is signal enough).
- Prefer `ir.rs`, `discovery.rs`, `pipeline.rs` over `test_ir.rs`, `ir_tests.rs`.

---

## Running tests

```bash
cargo test --all                 # everything
cargo test --lib                 # unit tests only
cargo test --test ir             # single integration file
cargo test -- --nocapture        # print stdout/stderr
cargo test -- --include-ignored  # run #[ignore]-gated tests
cargo test ir::serde             # filter by path substring
```

Tests that depend on the filesystem or external tooling should be marked `#[ignore]` with a
comment explaining why; CI runs them with `--include-ignored`.

---

## Patterns we use

### Temp directories

```rust
use tempfile::TempDir;

let dir = TempDir::new().unwrap();
std::fs::write(dir.path().join("Cargo.toml"), "[package]\nname = \"x\"\nversion = \"0.1.0\"\n").unwrap();
```

The `TempDir` is cleaned up automatically on drop.

### Assert with pretty diffs

```rust
use pretty_assertions::assert_eq;

assert_eq!(actual, expected);
```

`pretty_assertions` is a `dev-dependency` — see `Cargo.toml`.

### Serde round-trips

```rust
let model = CodeModel::new("rust");
let json = serde_json::to_string(&model).unwrap();
let parsed: CodeModel = serde_json::from_str(&json).unwrap();
assert_eq!(model, parsed);
```

### Snapshot-style assertions (no extra crate yet)

```rust
let output = render(&model, &config).unwrap();
assert!(output.contains("## Modules"));
assert!(output.contains("```mermaid"));
```

When snapshot drift becomes painful, reach for `insta` and wire it via `cargo-insta`.

### `#[ignore]` gate for slow/fs tests

```rust
#[test]
#[ignore = "touches the real filesystem; run with --include-ignored"]
fn watches_real_directory() { /* ... */ }
```

---

## Workflow A — Test-Driven Development (TDD)

Use when: the behaviour is well-specified, the design is still fuzzy, or you're fixing a bug.

1. **Red.** Write the smallest failing test that captures the new behaviour.
   - Put it in `tests/<concern>.rs` if the API is public; otherwise inside a `mod tests { }` block.
   - Run `cargo test --test <concern>` — it must fail for the reason you expect.
2. **Green.** Make it pass with the least code possible. Don't generalise; don't fix unrelated
   warnings. Just pass.
3. **Refactor.** With the test as a safety net, clean up: extract helpers, rename, dedupe. Keep
   the test green between every edit.
4. Repeat for the next slice of behaviour. Commit often — one test + one impl per commit is
   ideal.
5. **Bug fixes**: start with a test that reproduces the bug *before* you touch any production
   code. If the test passes on the first run, you don't understand the bug yet.

### TDD anti-patterns to avoid

- Writing six tests upfront and then the implementation. That's not TDD, that's "writing tests
  first".
- Testing implementation details (private fields, exact log strings). Assert on behaviour.

---

## Workflow B — Non-TDD (exploratory / spike)

Use when: you're exploring a design, the shape is unclear, or you're integrating an external tool
and need to feel out the boundary.

1. **Prototype first.** Get a minimal version working end-to-end in a scratch branch. Don't write
   tests yet — you're learning.
2. **Freeze the boundary.** Identify the public surface (function signatures, config fields) that
   will stay stable. Everything below the line can still change.
3. **Backfill integration tests in `tests/`** — one per user-visible behaviour. These lock the
   public API so you can refactor freely.
4. **Backfill unit tests for risky branches** — error paths, edge cases, state transitions.
   These should be fast and numerous.
5. **Add at least one doctest** for every `pub fn` whose usage is not obvious from the signature.
6. Delete any scaffolding that exists purely to make the prototype work; the test suite is what
   proves the feature now.

### Non-TDD is not "no tests"

Both workflows end in the same place: a feature with comprehensive, fast tests. The difference is
when you write them. Non-TDD only wins when you genuinely don't know the design yet — don't use it
to dodge test-writing.

---

## Current test map

| File                      | What it covers                                              |
| ------------------------- | ----------------------------------------------------------- |
| `tests/ir.rs`             | IR serde round-trips, `CodeModel::merge` semantics          |
| `tests/config.rs`         | `codetwin.toml` parsing, defaults, missing-file fallback    |
| `tests/drivers.rs`        | `DriverRegistry` detection + lookup                         |
| `tests/layouts.rs`        | Layout registry + MVP layout smoke tests                    |
| `tests/pipeline.rs`       | Discovery + end-to-end gen in a `TempDir`                   |
| `tests/snapshot.rs`       | `SnapshotStore` round-trip                                  |
| `tests/diff.rs`           | `diff::diff` on empty + simple CodeModels                   |
| `tests/cli.rs`            | CLI argument parsing (no subprocess)                        |
| `tests/fixtures/`         | Read-only sample projects consumed by other test files      |

---

## Adding a new test file

1. Create `tests/<concern>.rs`.
2. `use codetwin::...;` — only `pub` items are reachable.
3. Prefer many small `#[test]` functions over one big one — failure messages become self-
   documenting.
4. Don't `pub mod` anything under `tests/`. Shared helpers live in `tests/common/mod.rs` (create
   when needed).
5. Update the **Current test map** table above.
