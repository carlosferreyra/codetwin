# Contributing

Requires Rust 1.93 or newer. `rust-toolchain.toml` selects stable Rust with Clippy and rustfmt;
the package manifests retain the 1.93 minimum supported version.

Read [AGENTS.md](AGENTS.md) before changing behavior. Use [ROADMAP.md](ROADMAP.md) for project
context and add tests for behavior changes.

```sh
cargo xtask check
cargo xtask test
cargo xtask build
cargo xtask ci
```

`check` runs formatting, compilation, and Clippy across the workspace and all targets. `test`
includes `check`; `build` includes `test` and adds a release build. CI runs `ci`, which includes
tests and workspace documentation. Choose the command that covers your change without repeating
the checks it already includes.

The generation integration test is currently ignored by the ordinary suite. Run it explicitly
when changing the generation pipeline:

```sh
cargo test -p codetwin-legacy --test pipeline -- --ignored
```

Use `cargo xtask scaffold --help` for capabilities required by the task. Product crates use
semantic `codetwin-*` names and live under `crates/`. Development commands belong to private
`tools/xtask`. Keep behavior together until a separate responsibility earns a crate.

Scaffolds support `--dry-run`, validate planned paths before writes, and refuse conflicting
user-owned files. `cargo xtask doctor` reports optional tool availability. Tool installation is
explicit through `cargo xtask tools sync <group>`; it installs tools locally and updates the
shared stable Rust toolchain. Routine checks do not install tools.

The repository uses lean CI. Full CI, audit schedules, and Dependabot remain opt-in capabilities.
The existing release pipeline is separate from this migration; do not run release scaffolds to
replace its configuration without approval.
