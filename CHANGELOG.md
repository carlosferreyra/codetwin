## [0.1.4] - 2026-02-07

### ⚙️ Miscellaneous Tasks

- Include non-conventional commits in changelog
## [0.1.3] - 2026-02-07

### 💼 Other

- Add releases generator
- Add git-cliff changelog automation
- Update README for ct CLI and changelog
- Fix git-cliff hook and add hotfix group

### 📚 Documentation

- Add changelog

### ⚙️ Miscellaneous Tasks

- Release codetwin version 0.1.2
- Release codetwin version 0.1.3
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

### 💼 Other

- Add formatter abstraction with configurable layouts
- Update roadmap with subcommand status
- Prepare crate metadata and manual release workflow
- Phase 1.5 Infrastructure Planning + Phase 2 Layout 1 Implementation

- Add Phase 1.5 detailed breakdown with 8 meta-tasks for ecosystem integration
- Integrate anyhow, tracing, walkdir, glob, serde_json, notify-debouncer-mini, rayon
- Create AGENT_INSTRUCTIONS.md for structured agent implementation
- Mark Phase 2 Layout 1 (Dependency Graph) as complete ✅
- Implement dependency graph layout with Mermaid diagram generation
- Add layouts module with trait-based layout system
- Update ROADMAP.md with new dependency structure and validation criteria
- Add integration tests for layouts
- Refactor discovery module for upcoming Phase 1.5 work
- Respect gitignore in discovery
- Remove legacy discovery module
- Update docs and config
- Update workflows and packaging

### 🚜 Refactor

- *(cli)* Show help when no subcommand provided
- Refactor module structure

### 📚 Documentation

- Add README and set Cargo readme metadata
- Update documentation for Phase 2.5 completion
- *(generated)* Regenerate architecture and remove stale docs

### 🧪 Testing

- *(phase1.5/2/2.5)* Add comprehensive integration test suite

### ⚙️ Miscellaneous Tasks

- *(license)* Add MIT license (c) 2026 carlosferreyra
- Release codetwin version 0.1.1
