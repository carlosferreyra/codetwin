# codetwin

A code-to-diagram/documentation generator in Rust.

> Status: Phase 2 ✅ - Multi-layout architecture generator with Rust support. Generates
> documentation in multiple formats (dependency graphs, layered architecture, README summaries).

## Overview

CodeTwin transforms your codebase into visual documentation through multiple layout strategies:

- **Dependency Graph**: Shows module interdependencies
- **Layered Architecture**: Organizes code into logical layers/tiers
- **README-Embedded**: Compact summaries perfect for GitHub discovery

Perfect for architecture reviews, onboarding, and design documentation.

## Installation

You can build and run locally with Cargo (Rust toolchain required):

```bash
# From the repository root
cargo install --path .
```

Or run directly:

```bash
cargo run -- gen --help
```

## Quick Start

Generate documentation for your Rust project:

```bash
# Generate dependency graph (default)
ctw gen

# Generate layered architecture
ctw gen --layout layered

# Generate README summary
ctw gen --layout readme-embedded

# Watch mode: auto-regenerate on file changes
ctw watch
```

## Layout Options

### Dependency Graph (Default)

Shows how modules depend on each other. Ideal for understanding coupling and module relationships.

```bash
ctw gen --layout dependency-graph --output docs/architecture.md
```

**Output includes**:

- Module-level dependency diagram (Mermaid)
- List of all modules with functions/structs
- Circular dependency detection

### Layered Architecture

Organizes code into logical tiers (UI, API, Business Logic, Database, etc.). Best for architecture
reviews.

```bash
ctw gen --layout layered --output docs/layers.md
```

**Output includes**:

- Layer definitions with glob patterns
- Modules grouped by layer
- Inter-layer dependency diagram
- Layer responsibilities and key functions

Configure layers in `codetwin.toml`:

```toml
[[layers]]
name = "User Interface"
patterns = ["src/cli.rs", "src/ui/**"]

[[layers]]
name = "Engine"
patterns = ["src/engine.rs"]

[[layers]]
name = "Data Layer"
patterns = ["src/db/**", "src/models/**"]
```

### README-Embedded

Compact summary designed for README files. Perfect for GitHub discovery and quick onboarding.

```bash
ctw gen --layout readme-embedded --output docs/architecture.md
```

**Output includes**:

- Component overview table (Module | Purpose | Key Functions)
- Dependency overview diagram (Mermaid)
- Data flow explanation (numbered steps)
- Development guide with key files and contribution guidelines

Keep output under 300 lines for easy README embedding.

## Configuration

Create `codetwin.toml` in your project root:

```toml
# Source directories to scan
source_dirs = ["src"]

# Output file for generated documentation
output_file = "docs/architecture.md"

# Layout: dependency-graph, layered, readme-embedded
layout = "dependency-graph"

# Patterns to exclude from scanning
exclude_patterns = [
  "**/target/**",
  "**/node_modules/**",
  "**/.git/**",
  "**/tests/**"
]

# Layer configuration (for layered layout)
[[layers]]
name = "Core"
patterns = ["src/lib.rs", "src/ir.rs"]

[[layers]]
name = "Engine"
patterns = ["src/engine.rs"]
```

## Development

- Rust edition: 2021
- Min Rust: 1.70+ stable
- Key deps: `tree-sitter`, `petgraph`, `serde`, `clap`

Common tasks:

```bash
# Format & lint
cargo fmt --all
cargo clippy --all-targets -- -D warnings

# Test
cargo test --all

# Release build
cargo build --release

# Watch for changes
cargo watch -x test
```

## Features

✅ **Multiple Layouts** - Choose the documentation style that fits your needs ✅ **Rust Support** -
Full tree-sitter-based AST parsing ✅ **Flexible Configuration** - Control layers, patterns, and
output formats ✅ **Watch Mode** - Auto-regenerate on file changes ✅ **JSON Export** - Structured
output for tooling integration ✅ **Mermaid Diagrams** - Embedded diagrams for visual understanding

## Repository

- GitHub: <https://github.com/carlosferreyra/codetwin>

## License

MIT
