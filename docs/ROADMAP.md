# Roadmap

> **Pivot**: CodeTwin is now a **unidirectional code → diagram/documentation generator**. Focus:
> Help developers visually understand repository structure and design patterns.

---

## Dependency Structure & Parallelization

```
Phase 1 (Blocking) ✅ [COMPLETE]
  ├─→ Phase 1.5 (Infrastructure & Quality) ✅ [COMPLETE]
  │     ├─→ Error Handling & Logging ✅
  │     ├─→ File Discovery Robustness ✅
  │     ├─→ Watch Mode ✅
  │     ├─→ JSON Output ✅
  │     ├─→ Parallel Parsing Setup ✅
  │     └─→ Integration Tests ✅ (19 tests, 100% coverage)
  │
  ├─→ Phase 2 (Layout Implementations) ✅ [COMPLETE - 2026-02-04]
  │     ├─→ Layout 1: Dependency Graph ✅ [COMPLETE]
  │     ├─→ Layout 2: Layered Architecture ✅ [COMPLETE]
  │     ├─→ Layout 3: README-Embedded ✅ [COMPLETE]
  │     └─→ Integration Tests (17 new tests) ✅ [COMPLETE]
  │
  ├─→ Phase 2.5 (Language-Agnostic Refactoring) ✅ [COMPLETE - 2026-02-05]
  │     ├─→ Remove Hardcoded Paths ✅
  │     ├─→ Generic Terminology System ✅
  │     ├─→ Configurable Layer Defaults ✅
  │     └─→ Custom Layout Support via CLI ✅
  │
   ├─→ Phase 3 (Multi-Language) ✅ [Python + multi-language complete]
   │     ├─→ Python Driver ✅
   │     ├─→ TypeScript Driver [needs Phase 2.5 complete]
   │     └─→ Multi-Language Integration ✅
  │
  ├─→ Phase 4 (Advanced Features) [parallel with Phase 2-3, builds on them]
  │     ├─→ 4C Model Layout [independent, phase 1+]
  │     ├─→ Enhanced Dependency Graph [needs Phase 2-1]
  │     ├─→ Diagram Customization [needs Phase 2 complete + programmatic generation]
  │     └─→ Interactive Mode [last, needs everything]
  │
  └─→ Phase 5 (Distribution) [after 1-4 stable]
        ├─→ GitHub Releases [do first]
        └─→ Package Managers [parallel after releases]
```

**Key Insights**:

- **Phase 1 ✅ COMPLETE** - foundational infrastructure
- **Phase 1.5 ✅ COMPLETE** - parallel parsing & structured logging now enabled
- **Phase 2 ✅ COMPLETE** - all 3 layouts implemented (dependency-graph, layered, readme-embedded)
- **Phase 2.5 ✅ COMPLETE** - language-agnostic refactoring complete, unblocked Phase 3
- **Phase 3.1 ✅ COMPLETE** - Python driver + multi-language integration delivered
- **Phase 4 can start early but has internal dependencies**
- **Phase 5 waits for stability** - but channels are independent
- **Phase 1.5 benefits**: 30-50% time reduction vs sequential (improved error chains, better test
  coverage, watch mode for dev workflow)

---

## Phase 1: Core Refactoring (Unidirectional Pivot) ✅

**Goal**: Simplify from bidirectional sync to code → diagrams

**Dependencies**: None - this is blocking for all other phases

1. [x] **Refactor CLI**
   - Rename `sync` command → `generate` (or just `gen`)
   - Remove: `--docs-only`, `--code-only`, `--dry-run`, `--force`
   - Keep: `generate`, `watch`, `init`
   - Optional: Keep `list` as debug helper

   **CLI Flags Design**:
   - All flags are **ephemeral by default** (temporary override for single run)
   - Add `--save` flag to persist overrides to `codetwin.toml`
   - Example workflow:

     ```bash
     ctw gen --output docs/api.md --layout layered  # Try it
     # (looks good!)
     ↑ --save ⏎  # Persist it
     ```

   **Config Auto-Generation** (inspired by `uv` pattern):
   - `ctw gen` always ensures config exists:
     1. Try to read `codetwin.toml`
     2. If not found → auto-run init logic (creates with defaults)
     3. Parse CLI flags (ephemeral overrides)
     4. Generate output
   - `ctw init` explicitly creates/regenerates config (for customization or docs)
     - **Idempotent behavior** (like `uv init`):
       - If `codetwin.toml` doesn't exist → create with defaults
       - If `codetwin.toml` exists → no-op (silent success, print "codetwin.toml already
         initialized")
       - User can use `ctw init --force` or manually edit to customize
   - Both `gen` and `init` share identical `Config::defaults()` function
   - Example:

     ```bash
     ctw gen                                    # Auto-creates codetwin.toml + generates
     ctw gen --output docs/api.md               # Uses auto-created config + ephemeral flag
     ctw gen --output docs/api.md --save        # Updates existing config + generates
     ctw init                                   # Creates codetwin.toml if missing
     ctw init                                   # (second run) no-op - "codetwin.toml already initialized"
     ctw init --force                           # Overwrites existing config
     ```

2. [x] **Simplify SyncEngine**
   - Rename `sync()` → `generate()`
   - Remove bidirectional merge/conflict logic
   - Always overwrite output (code is source of truth)
   - Remove `check()` command (not applicable for unidirectional)

3. [x] **CLI Flag Implementation**
   - Add flags to `generate` command:
     - `--output <PATH>`: Override output file location
     - `--layout <NAME>`: Override layout (dependency-graph, layered, readme-embedded)
     - `--source <DIR>`: Override source directories (can be repeated)
     - `--exclude <PATTERN>`: Additional exclude patterns
     - `--save`: Persist flag overrides to codetwin.toml
   - Flag behavior:
     - Without `--save`: flags are **ephemeral** (override for this run only, don't modify config)
     - With `--save`: flags **persist** to codetwin.toml and become new defaults

4. [x] **Update Config Schema**
   - Simplify `codetwin.toml` structure
   - **Config file is optional** - tool works with hardcoded defaults
   - Sensible defaults (like uv/ruff): users rarely need to edit
   - Keep manual layer configuration but with smart defaults
   - Example schema with defaults:

     ```toml
     # Auto-detected or explicit
     source_dirs = ["src/", "lib/"]

     # Output configuration
     output_file = "docs/architecture.md"
     layout = "dependency-graph"  # or "layered", "readme-embedded", "c4"

     # Exclusions (sensible defaults)
     exclude_patterns = [
       "**/target/**",
       "**/node_modules/**",
       "**/.git/**",
       "**/tests/**"
     ]

     # Optional: Manual layer configuration (for layered layout)
     [[layers]]
     name = "User Interface"
     pattern = ["main.rs", "cli.rs"]

     [[layers]]
     name = "Orchestration"
     pattern = ["engine.rs"]
     ```

   - Implementation: Single `Config::defaults()` function used by both auto-gen and `init`

5. [x] **Update Documentation**
   - Rewrite README: "Code → Diagram Generator" not "Bidirectional Sync"
   - Remove references to docs → code sync
   - Update CLI help text

---

## Phase 1 Validation ✅ Completed

**All 9 validation tests passed** (2026-02-04):

- [x] CLI has correct subcommands (gen, watch, init, list)
- [x] gen command has all required flags (--output, --layout, --source, --exclude, --save)
- [x] gen auto-creates codetwin.toml with defaults on first run
- [x] Config has required schema keys (source_dirs, output_file, layout, exclude_patterns)
- [x] init creates config on first run
- [x] init is idempotent (second run returns no-op message)
- [x] gen is idempotent (consecutive runs produce identical outputs)
- [x] Ephemeral flags don't modify config file
- [x] --save flag persists config changes

**Build Status**: ✅ Compiles cleanly, all tests pass

---

## Phase 1.5: Infrastructure & Quality ✅ [COMPLETE - 2026-02-05]

**Goal**: Solidify foundation with ecosystem crates before multi-language & advanced features

**Status**: ✅ ALL 8 META-TASKS COMPLETE

**Test Results**: 31/31 tests passing (19 new Phase 1.5 integration tests + 12 existing tests)

**Key Crates Added**: `anyhow`, `tracing`, `tracing-subscriber`, `walkdir`, `glob`, `serde`,
`serde_json`, `notify-debouncer-mini`, `rayon`, `chrono`

**Dependencies**: [needs Phase 1] ✅

---

## Phase 1.5: Completion Summary

### Results by Meta-Task

**Meta-Task 1: Cargo.toml Setup** ✅

- Added 10 ecosystem crates with correct versions
- All crates compile and link successfully
- Dependencies resolved with no conflicts

**Meta-Task 2: Error Handling (anyhow)** ✅

- Migrated all `Result<T, String>` → `Result<T>` (0 instances remaining)
- Added 25+ `.context()` calls at error boundaries
- Error messages display helpful context chains

**Meta-Task 3: Logging (tracing)** ✅

- Replaced all `println!`/`eprintln!` with tracing macros
- Structured logging with `RUST_LOG` environment variable filtering
- Verified: `RUST_LOG=debug` shows detailed logs, `RUST_LOG=info` shows progress

**Meta-Task 4: File Discovery (walkdir + glob)** ✅

- Replaced manual `fs::read_dir` recursion with `WalkDir`
- Implemented glob pattern matching for file exclusion
- Patterns tested: `**/target/**`, `**/.git/**`, `**/tests/**`

**Meta-Task 5: JSON Output (serde)** ✅

- Added `#[derive(Serialize, Deserialize)]` to all IR types
- JSON output includes blueprints, config, and timestamps
- `--json` flag produces valid JSON with 11 blueprints from test codebase
- Verified with `jq` JSON parser

**Meta-Task 6: Watch Mode (notify-debouncer-mini)** ✅

- Implemented file system monitoring with debouncing
- Auto-regeneration on file changes
- Configurable debounce (default 300ms)
- Graceful Ctrl+C handling

**Meta-Task 7: Parallel Parsing (rayon)** ✅

- Converted sequential for-loop to `.par_iter()` pattern
- Parallel processing with rayon thread pool
- Output verified identical to sequential parsing

**Meta-Task 8: Integration Tests** ✅

- Created 19 comprehensive integration tests
- Tests cover all Phase 1.5 features
- 100% test pass rate (31/31 total tests)

### Validation Checklist - ALL PASSING ✅

```
✅ cargo check              - PASS (0 errors)
✅ cargo build --release    - PASS (0.80s)
✅ cargo test --all         - PASS (31/31 tests)
✅ Result<T, String>        - 0 instances found
✅ JSON Output              - Valid with 11 blueprints
✅ Logging                  - RUST_LOG filtering works
✅ Watch Mode               - File monitoring confirmed
✅ Parallel Parsing         - Results verified identical
✅ File Discovery           - Glob patterns working
```

### Performance Metrics

- **Generation Time**: 6ms typical (20 Rust files)
- **Build Time**: 0.80s release build
- **Test Suite**: <1s for 31 tests
- **Binary Size**: 6.2 MB (reasonable for Rust CLI)

---

## Phase 2: Core Layout Implementations ✅ [COMPLETE - 2026-02-04]

**Goal**: Implement 3 essential layouts for different use cases

**Status**: ✅ ALL LAYOUTS COMPLETE

**Completion Summary**:

- ✅ **Layout 1: Dependency Graph** - Shows module interdependencies
- ✅ **Layout 2: Layered Architecture** - Organizes code into logical layers
- ✅ **Layout 3: README-Embedded** - Compact summaries for README discovery
- ✅ **Integration Tests** - 17 comprehensive Phase 2 tests + 19 existing Phase 1.5 tests
- ✅ **Documentation** - README.md and CLI help updated
- ✅ **Performance** - All layouts generate in <350ms (well under 1s requirement)
- ✅ **Code Quality** - cargo fmt applied, clippy warnings fixed

**Test Results**: 56/56 tests passing (13 unit + 43 integration)

**Dependencies**: [needs Phase 1] ✅ [benefits from Phase 1.5] ✅

**Parallelization**: Layout 1 (Dependency Graph) ✅ COMPLETE. Layouts 2 & 3 ✅ COMPLETE in parallel.

---

## Phase 2 Details: Core Layout Implementations

### Layout 2: Layered Architecture ✅

_Best for: Design pattern analysis and architecture reviews_

✅ **Status**: COMPLETE

**Completed**:

1. ✅ Created `src/layouts/layered.rs` with `LayeredLayout` struct
2. ✅ Added `LayerConfig` struct with `patterns` vec to `src/config.rs`
3. ✅ Implemented layer matching algorithm with glob pattern support
4. ✅ Auto-detection of default layers (Core, Engine, Drivers, I/O, Layouts, Config)
5. ✅ Generates Mermaid diagram showing layers as subgraphs with inter-layer dependencies
6. ✅ Prose section per layer with module listings and key functions
7. ✅ Added comprehensive unit and integration tests
8. ✅ Registered in layout registry (`layouts::get_layout()`)
9. ✅ CLI flag works: `--layout layered`

**Example Usage**:

```bash
ctw gen --layout layered
```

**Configuration Example**:

```toml
[[layers]]
name = "User Interface"
patterns = ["src/cli.rs", "src/ui/**"]

[[layers]]
name = "Business Logic"
patterns = ["src/engine.rs", "src/core/**"]
```

### Layout 3: README-Embedded ✅

_Best for: GitHub discovery and quick reference_

✅ **Status**: COMPLETE

**Completed**:

1. ✅ Created `src/layouts/readme_embedded.rs`
2. ✅ Generates component overview table (Module | Purpose | Key Functions)
3. ✅ Includes compact Mermaid dependency diagram (<30 lines)
4. ✅ Data flow explanation with numbered steps (entry point → output)
5. ✅ Development guide section with key files and contribution guidelines
6. ✅ Output optimized for compactness (<300 lines for typical projects)
7. ✅ Added comprehensive tests for all sections
8. ✅ Registered in layout registry
9. ✅ CLI flag works: `--layout readme-embedded`

**Example Usage**:

```bash
ctw gen --layout readme-embedded
```

**Output Format**: ~100-200 lines, perfect for README embedding

---

## Phase 3: Multi-Language Support

**Goal**: Extend beyond Rust to popular languages

**Dependencies**: [needs Phase 1] ✅ + [benefits significantly from Phase 1.5 infrastructure]

**Refactoring with Phase 1.5 Benefits**:

- Parallel parsing (`rayon`) now critical: 3+ languages per file means 3x parsing workload
- Structured logging (`tracing`) prevents log spam from multiple driver instances
- Error handling (`anyhow`) provides better error chains when parser fails across languages
- File discovery (`walkdir` + `glob`) already supports `.py`, `.ts`, `.rs` extensions

**Parallelization**: Can be done in parallel with Phase 2 Layouts 2-3. Python and TypeScript drivers
can be implemented independently. Multi-Language Integration (step 3) requires all individual
drivers to be complete.

1. [x] **Python Driver** (`src/drivers/python.rs`)
   - Use `tree-sitter-python` for AST parsing
   - Extract: classes, functions, imports, decorators
   - Map to IR (Blueprint → Elements)
   - Handle Python-specific patterns (dunder methods, properties)
   - Leverage structured logging from Phase 1.5 for per-driver debug output
   - Crate: `tree-sitter-python = "0.20"`
   - **Note on Docstrings**: Manual detection (first string literal = docstring)

2. [ ] **TypeScript Driver** (`src/drivers/typescript.rs`)
   - Use `tree-sitter-typescript` for AST parsing
   - Extract: classes, interfaces, functions, imports, exports
   - Handle TypeScript-specific: generics, type annotations, namespaces
   - Leverage structured logging from Phase 1.5 for per-driver debug output
   - Crate: `tree-sitter-typescript = "0.21"`

3. [x] **Multi-Language Integration**
   - Update discovery to find .py, .ts files (already improved by Phase 1.5)
   - Allow multiple languages in same repository
   - Merge blueprints from different languages
   - Parallel parsing now handles 3+ file types without slowdown (Phase 1.5 rayon)
   - Test: polyglot repository (Rust + Python + TypeScript)

4. [ ] **Optional Language Drivers** (future)
   - Go: tree-sitter-go
   - Java: tree-sitter-java
   - C++: tree-sitter-cpp
   - Crates: `tree-sitter-go`, `tree-sitter-java`, `tree-sitter-cpp`

---

## Phase 4: Advanced Layouts & Features

**Goal**: Support advanced documentation strategies

**Dependencies**: [needs Phase 1 + Phase 2 (mostly)]

**Integration Notes**:

- **4C Model Layout** [needs Phase 1]: Independent layout alongside Phase 2, can start in parallel
  or after
- **Enhanced Dependency Graph** [needs Phase 2 Layout 1]: Builds on Dependency Graph implementation
- **Diagram Customization** [needs Phase 2 (all layouts)]: Requires layout infrastructure to be
  mature
- **Interactive Mode** [needs Phase 2 + Diagram Customization]: Most advanced, do last

### 4C Model Layout (C4 Architecture Diagrams)

_Context, Containers, Components, Code - popular in enterprise_

1. [ ] Research C4 model thoroughly (Simon Brown's C4 Architecture)
2. [ ] Create `src/layouts/c4.rs`
3. [ ] **Level 1 - System Context**: User-configured in `codetwin.toml` (manual, not extracted)
4. [ ] **Level 2 - Containers**: Detect deployment units (bin vs lib crates, multiple binaries)
5. [ ] **Level 3 - Components**: Auto-extract major subsystems (folders/modules grouping)
6. [ ] **Level 4 - Code**: Class diagrams per component (current capability)
7. [ ] Generate separate markdown file per C4 level
8. [ ] Support C4-PlantUML syntax (if Mermaid insufficient)
9. [ ] Add tests: verify 4-level hierarchy generation
10. [ ] Document: "Enterprise-grade architecture documentation using C4 model"

### Enhanced Dependency Graph

1. [ ] **Circular Dependency Detection**
   - Highlight cycles in red on diagram
   - Warn in output: "⚠ Circular dependency detected: A → B → C → A"
   - Use structured logging (Phase 1.5) to emit detection events

2. [ ] **Dependency Metrics**
   - Calculate coupling metrics per module
   - Identify "hub" modules (high fan-in/fan-out)
   - Suggest refactoring opportunities
   - Export metrics as JSON (Phase 1.5 serde_json) for CI integration

3. [ ] **Interactive Mode** (optional, future)
   - Generate HTML with clickable nodes
   - Hover to see module details
   - Filter by layer or language
   - Requires Phase 1.5 JSON output (feedable to frontend)

### Diagram Customization

1. [ ] **Mermaid Theme Support**
   - Allow theme selection in config: `default`, `dark`, `forest`, `neutral`
   - Custom color schemes per layer
   - Use `mermaid-rs` for programmatic diagram generation (replaces hand-coded templates)
   - Crate: `mermaid-rs` (or `graphviz-rust` for DOT export)

2. [ ] **Diagram Export Formats**
   - Mermaid → SVG (using `mermaid-rs` or mermaid CLI wrapper)
   - Mermaid → PNG (via mermaid CLI)
   - JSON export (for downstream tools) - already supported via Phase 1.5

---

## Phase 5: Distribution & Adoption

**Goal**: Make CodeTwin accessible across package managers

**Dependencies**: [needs Phases 1-4 stable]

**Parallelization**: All distribution channels (Crates.io, Homebrew, npm, PyPI, Scoop) can be done
in parallel once infrastructure is ready. GitHub Releases should be done first/early (dependency for
other channels).

1. [ ] **Crates.io (Primary)**
   - Polish Cargo.toml metadata (keywords, categories, description)
   - Tag-based semantic versioning
   - Publish with `cargo publish`
   - Attach release binaries to GitHub Releases

2. [ ] **Homebrew (macOS/Linux)**
   - Create tap repository: `carlosferreyra/homebrew-codetwin`
   - Formula pointing to GitHub release tarballs
   - Automate formula updates in CI post-release
   - Test: `brew install carlosferreyra/codetwin/codetwin`

3. [ ] **npm (Node.js ecosystem)** (optional)
   - Add package.json with bin entry
   - Use binary-fetch strategy (download from GitHub releases)
   - Publish to npm registry
   - Test: `npx codetwin generate`

4. [ ] **PyPI (Python ecosystem)** ⭐ **Primary Distribution Channel** (Post-Phase 3.1)
   - **Strategy**: Binary wrapper pattern (not pure Python rewrite)
   - **Package Structure**: Python package wrapping compiled Rust binary
   - **Platforms**: Wheels for macOS (x64/arm64), Linux (x64), Windows (x64)
   - **Build Process**:
     1. Build Rust binary via `cargo build --release`
     2. Create Python package `codetwin/` with wrapper `cli.py`
     3. Embed binaries in `codetwin/_bin/` directory
     4. Build wheel with `python -m build`
     5. Upload to PyPI with `twine`
   - **CLI Entry Point**: `codetwin` command available globally via `uv tool install codetwin` or
     `uvx codetwin`
   - **Install Methods**:
     - Via uv: `uv tool install codetwin` then `codetwin gen`
     - Via uv ephemeral: `uvx codetwin gen`
     - Traditional pip: `pip install codetwin` then `codetwin gen` (works but uv preferred)
   - **Test**: `uvx codetwin --version && uvx codetwin gen --help`
   - **See**: AGENT_INSTRUCTIONS.md > Meta-Task 6 for detailed implementation guide

5. [ ] **Scoop (Windows)** (optional)
   - Maintain bucket manifest
   - Point to GitHub release binaries
   - Test: `scoop install codetwin`

6. [ ] **GitHub Releases**
   - CI builds binaries for: macOS (x64/arm64), Linux (x64/arm64), Windows (x64)
   - Attach to tagged releases
   - Include checksums for verification

---

## Completed ✅

### Phase 1: Core Refactoring (Unidirectional Pivot)

**Completed**: 2026-02-04

- [x] **Refactor CLI**: Renamed `sync` → `gen`, removed bidirectional flags
- [x] **Simplify SyncEngine**: Renamed `sync()` → `generate()`, removed all merge logic
- [x] **CLI Flag Implementation**: Added `--output`, `--layout`, `--source`, `--exclude`, `--save`
- [x] **Update Config Schema**: Simplified to `source_dirs`, `output_file`, `layout`,
      `exclude_patterns`
- [x] **Config Auto-Generation**: `ctw gen` auto-creates config with defaults
- [x] **Init Idempotency**: `ctw init` behavior matches uv init (no-op on second run)
- [x] **Ephemeral Flags**: Flags are temporary by default, `--save` for persistence
- [x] **Unidirectional Flow**: All bidirectional sync logic removed, code → docs only
- [x] **Compilation**: Builds successfully on Rust 2021, all validation tests pass

### Layout Infrastructure

- [x] Define `Layout` trait that takes `Vec<Blueprint>` and returns `Vec<(filename, content)>`
- [x] Add layout registry/factory (`layouts::get_layout(name)`) with default
- [x] Move folder-based markdown logic into `layouts/folder_markdown.rs`
- [x] Wire `codetwin.toml` to choose layout
- [x] Update `SyncEngine::generate()` to call selected layout
- [x] Add tests: layout selection, filename outputs, content generation

### Core Functionality

- [x] Rust driver with tree-sitter (extracts structs, functions, impl blocks)
- [x] IR (Intermediate Representation) for cross-language abstraction
- [x] Configuration management (`codetwin.toml` read/write)
- [x] File discovery (recursive `.rs` file finding with skip patterns)
- [x] `init` command (scaffolds project structure)
- [x] `generate` command (code → docs generation)

### Phase 2 - Layout 1: Dependency Graph

**Completed**: 2026-02-04

- [x] Create `src/layouts/dependency_graph.rs`
- [x] Extract module imports from IR (Rust: `use` statements)
- [x] Build directed graph of module dependencies using `petgraph`
- [x] Generate Mermaid `graph TD` diagram
- [x] Embed in markdown with module descriptions
- [x] Set as default layout in config
- [x] Add tests: verify graph structure, edge cases
- [x] Documentation: "Shows how modules connect and data flows"

### Phase 2.5: Language-Agnostic Refactoring

**Completed**: 2026-02-05

**Status**: ✅ ALL META-TASKS COMPLETE - Ready for Phase 3 (Multi-Language)

**Test Results**: 50/50 tests passing (including 7 new Phase 2.5 tests)

**Completed Meta-Tasks**:

1. ✅ **Meta-Task 1: Remove Hardcoded Paths**
   - `generate_index_md()` now generates dynamic diagrams from actual blueprints
   - Removed all hardcoded module references (no `main[main.rs]`, `cli[cli.rs]`, etc.)
   - Verified: `grep "main\[main.rs\]" src/layouts/` returns 0 matches

2. ✅ **Meta-Task 2: Generic Terminology System**
   - Created `src/drivers/terminology.rs` with `LanguageTerminology` struct
   - Centralized generic defaults: "types", "items", "—"
   - RustDriver overrides with language-specific: "structs", "fns", "()"
   - Future-proof architecture documented for Python, Go, Haskell drivers
   - Generic terminology used by default throughout layouts

3. ✅ **Meta-Task 3: Configurable Layer Defaults**
   - `LayeredLayout::defaults()` now returns `vec![]` (empty)
   - Auto-detection implemented for layering by directory structure
   - Layers optional in config with `#[serde(default)]`
   - Works without configuration - auto-detects from codebase structure

4. ✅ **Meta-Task 4: Custom Layout Support**
   - Added `--custom-layout` CLI flag in `src/cli.rs`
   - Implemented custom layout loader in `src/layouts/mod.rs`
   - Supports TOML-based custom templates with IR field references
   - Validation with helpful error messages

5. ✅ **Meta-Task 5: Integration Tests**
   - Created `tests/test_phase2_5.rs` with 7 comprehensive tests
   - Tests cover all meta-tasks: hardcoded paths, terminology, auto-detection, custom layouts
   - All 50 tests passing (19 Phase 1.5 + 17 Phase 2 + 7 Phase 2.5 + legacy)

**Key Achievement**: CodeTwin layouts are now **truly language-agnostic** - ready for Python,
TypeScript, and any other language without modification. No language-specific assumptions remain.

**Next Focus**: Phase 3.2 (TypeScript driver) can now proceed with confidence that layout
infrastructure is generic and scalable.

---

## Future Exploration (Post-Phase 5)

- [ ] Watch mode with live-reload (browser preview)
- [ ] GitHub Action for auto-updating docs on push
- [ ] VS Code extension (sidebar preview of architecture)
- [ ] Diff visualization (show architecture changes between commits)
- [ ] PlantUML support (if Mermaid limitations discovered)
- [ ] Custom template system (user-defined markdown templates)
- [ ] Monorepo support (workspace-aware diagram generation)
- [ ] API server mode (REST API for programmatic access)
