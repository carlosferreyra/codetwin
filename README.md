# codetwin

A bidirectional documentation and code synchronization tool.

> Status: early prototype. CLI scaffolding in place; functionality incoming.

## Overview

Codetwin aims to keep code and docs in sync, reducing drift by:

- Extracting code from docs to verified snippets
- Embedding code references back into docs
- Providing a simple CLI workflow for local repos

## Installation

You can build and run locally with Cargo (Rust toolchain required):

```bash
# From the repository root
cargo build
```

Or run directly:

```bash
cargo run -- --help
```

## Usage

Initial scaffold prints a greeting to verify the toolchain:

```bash
cargo run
```

As features land, the CLI will expose subcommands using `clap`.

## Formatter layouts

- folder_markdown (Available): One Markdown per source folder plus an index file; default layout.
- mirror_tree (Proposed): Mirror `src/` structure into `docs/` with one Markdown per source file or
  module for path parity.
- readme_append (Proposed): Append a generated section into README with markers to protect
  hand-written content.
- per_language (Proposed): Group outputs by language first, then folders for polyglot repos.
- site_bundle (Proposed): Emit Markdown plus minimal static-site-friendly index/metadata (e.g.,
  mkdocs/sidebar) without bundling a generator.
- api_ref_only (Proposed): Produce lean API signature references without diagrams for compact docs.
- diagram_first (Proposed): Emit diagram-focused summaries (Mermaid/PlantUML blocks) with minimal
  prose.

## Development

- Rust edition: 2024
- Min Rust: recent stable recommended
- Key deps: `clap` with `derive`

Common tasks:

```bash
# Format & lint
cargo fmt --all
cargo clippy --all-targets -- -D warnings

# Test
cargo test

# Release build
cargo build --release
```

## Repository

- GitHub: <https://github.com/carlosferreyra/codetwin>

## License

MIT
