## [0.1.2] - 2026-02-07

### 💼 Other

- Fix git-cliff hook and add hotfix group
## [0.1.1] - 2026-02-07

### 🚀 Features

- Initial commit of codetwin skeleton
- *(cli)* Implement complete CLI skeleton with all subcommands
- *(ir)* Implement UML-inspired IR for function/class signatures
- *(drivers)* Implement Rust driver stub and Markdown generator
- *(engine)* Wire up sync command to generate STRUCT.md
- *(markdown)* Add Mermaid class diagram generation
- *(layout)* Implement hierarchical documentation structure
- Implement init subcommand with codetwin.toml scaffolding
- *(phase1)* File discovery and config loading
- *(phase2)* Implement real Rust parsing with tree-sitter
- *(phase3)* Wire real parsing into sync() command - dog-fooding!
- *(phase2.5)* Implement language-agnostic terminology system
- *(phase2.5)* Implement dynamic layouts (layered, readme-embedded)
- *(phase2.5)* Add --custom-layout CLI flag and config support

### 🐛 Bug Fixes

- One diagram per folder, not per file
- Correct file paths in STRUCT.md index links

### 🚜 Refactor

- *(cli)* Show help when no subcommand provided

### 📚 Documentation

- Add README and set Cargo readme metadata
- Update documentation for Phase 2.5 completion
- *(generated)* Regenerate architecture and remove stale docs

### 🧪 Testing

- *(phase1.5/2/2.5)* Add comprehensive integration test suite

### ⚙️ Miscellaneous Tasks

- *(license)* Add MIT license (c) 2026 carlosferreyra
- Release codetwin version 0.1.1
