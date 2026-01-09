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
