## [0.1.2] - 2026-02-07

### 💼 Other

- Fix git-cliff hook and add hotfix group

## [0.1.1] - 2026-02-07

### 🚀 Features

- Initial commit of codetwin skeleton
- _(cli)_ Implement complete CLI skeleton with all subcommands
- _(ir)_ Implement UML-inspired IR for function/class signatures
- _(drivers)_ Implement Rust driver stub and Markdown generator
- _(engine)_ Wire up sync command to generate STRUCT.md
- _(markdown)_ Add Mermaid class diagram generation
- _(layout)_ Implement hierarchical documentation structure
- Implement init subcommand with codetwin.toml scaffolding
- _(phase1)_ File discovery and config loading
- _(phase2)_ Implement real Rust parsing with tree-sitter
- _(phase3)_ Wire real parsing into sync() command - dog-fooding!
- _(phase2.5)_ Implement language-agnostic terminology system
- _(phase2.5)_ Implement dynamic layouts (layered, readme-embedded)
- _(phase2.5)_ Add --custom-layout CLI flag and config support

### 🐛 Bug Fixes

- One diagram per folder, not per file
- Correct file paths in STRUCT.md index links

### 🚜 Refactor

- _(cli)_ Show help when no subcommand provided

### 📚 Documentation

- Add README and set Cargo readme metadata
- Update documentation for Phase 2.5 completion
- _(generated)_ Regenerate architecture and remove stale docs

### 🧪 Testing

- _(phase1.5/2/2.5)_ Add comprehensive integration test suite

### ⚙️ Miscellaneous Tasks

- _(license)_ Add MIT license (c) 2026 carlosferreyra
- Release codetwin version 0.1.1
