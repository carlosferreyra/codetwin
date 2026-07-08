# AGENTS.md

Operating rules for AI agents working in this repo. Read before any non-trivial change.

## 1. Spec-driven, roadmap-bound

- `ROADMAP.md` is the source of truth. Every change must map to a current-phase step or a
  `TODO(Phase N.x)` marker.
- **No work without a spec**: new layouts, drivers, features, or scope expansions require an
  existing issue or roadmap entry. If one doesn't exist, stop and ask.
- Out-of-phase work is rejected even if the code is good.

## 2. One session = one feature

- A session produces **1–2 commits max**. If the task spans more, decompose it first and propose the
  split before writing code.
- Each commit must be independently reviewable and leave the tree green (`cargo test --all`
  passing).
- Don't batch unrelated changes. Surgical diffs only.

## 3. Tests are first-class

- Tests are not an afterthought — their **design, validation, and execution** weigh equally with the
  code.
- TDD where practical: failing test → implementation → green. See `tests/README.md`.
- Every behavior change needs a test. Every bug fix starts with a reproducing test.
- Unit and integration tests both count. Don't skip integration coverage because unit tests pass.

## 4. Determinism is non-negotiable

The same `(CodeModel, Config)` **must** render byte-identical output across runs. This unlocks
snapshot tests and `codetwin diff`.

- No `HashMap`/`HashSet` iteration in render or serialization paths. Use `BTreeMap`/`BTreeSet`, or
  sort before emit.
- No timestamps, absolute paths, OS-specific separators, or `rand` in output.
- No parallelism that affects output ordering (`rayon` is fine for compute, not for emit).
- If you can't prove determinism, write a snapshot test that runs twice and diffs.

## 5. Pre-commit gate (all must pass)

```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test --all
```

No `--no-verify`. If a hook fails, fix the cause.

## 6. Rust style

- No `unwrap()` / `expect()` outside tests and `main.rs` startup. Use `?` with `anyhow::Result` in
  app/CLI code, `thiserror` for library error types.
- No `unsafe`.
- Match existing formatting in `Cargo.toml` (aligned keys) and source files. Don't reformat adjacent
  code.
- Edition 2024, MSRV 1.93. Don't bump either casually.

## 7. Off-limits without explicit approval

- Release pipeline: `.github/workflows/release*.yml`, `release.toml`, `cargo-dist` config,
  `scripts/release_*`.
- Adding new layouts or drivers without a roadmap step or issue.
- Adding network/HTTP dependencies. CodeTwin runs offline.
- Bumping `version` in `Cargo.toml` or wrapper manifests.

## 8. Surgical changes

- Touch only what the task requires. Don't "improve" adjacent code, comments, formatting, or
  imports.
- Clean up orphans **your** changes created; leave pre-existing dead code alone (mention it, don't
  delete).
- Every changed line must trace to the request.

## 9. Commits & PRs

- **Conventional Commits required**: `feat(scope):`, `fix(scope):`, `refactor(scope):`,
  `chore(scope):`, `docs(scope):`, `test(scope):`. Scope = module (`drivers`, `ir`, `cli`, `render`,
  ...).
- Subject ≤ 72 chars, imperative mood, no trailing period.
- Body explains **why**, not what.
- **Never squash-merge.** History is preserved. Use merge commits or rebase-merge with all commits
  intact.
- Reference the roadmap step or issue in the body when applicable: `Refs: Phase 1.a`.

## 10. When in doubt

Stop and ask. Surfacing a tradeoff costs one message; an out-of-scope rewrite costs a session.

## Agent skills

### Issue tracker

Issues tracked in GitHub Issues at `carlosferreyra/codetwin` via the `gh` CLI. See
`docs/agents/issue-tracker.md`.

### Triage labels

Default canonical labels (`needs-triage`, `needs-info`, `ready-for-agent`, `ready-for-human`,
`wontfix`). See `docs/agents/triage-labels.md`.

### Domain docs

Single-context: `CONTEXT.md` + `docs/adr/` at the repo root. See `docs/agents/domain.md`.
