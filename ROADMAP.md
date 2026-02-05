# Roadmap

> **Pivot**: CodeTwin is now a **unidirectional code → diagram/documentation generator**. Focus:
> Help developers visually understand repository structure and design patterns.

---

## Dependency Structure & Parallelization

```
Phase 1 (Blocking)
  ├─→ Phase 1.5 (Infrastructure & Quality)
  │     ├─→ Error Handling & Logging
  │     ├─→ File Discovery Robustness
  │     ├─→ Watch Mode
  │     ├─→ JSON Output
  │     └─→ Parallel Parsing Setup
  │
  ├─→ Phase 2 (Layout Implementations)
  │     ├─→ Layout 1: Dependency Graph ✅ [COMPLETE]
  │     └─→ Layouts 2 & 3 [parallel]
  │
  ├─→ Phase 3 (Multi-Language) [parallel with Phase 2 Layouts 2-3]
  │     ├─→ Python Driver [independent]
  │     ├─→ TypeScript Driver [independent]
  │     └─→ Multi-Language Integration [needs all drivers + Phase 1.5]
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

- **Phase 1 must complete first** - foundational infrastructure
- **Phase 1.5 must complete before Phase 3** - enables parallel parsing & structured logging
- **Phases 2 (Layout 2-3) & 3 can run in parallel** - independent feature areas (after Layout 1)
- **Phase 4 can start early but has internal dependencies**
- **Phase 5 waits for stability** - but channels are independent
- **Estimated parallelization savings**: 30-50% time reduction vs sequential (improved with Phase
  1.5)

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

## Phase 1.5: Infrastructure & Quality

**Goal**: Solidify foundation with ecosystem crates before multi-language & advanced features

**Dependencies**: [needs Phase 1] ✅

**Key Crates**: `anyhow`, `tracing`, `tracing-subscriber`, `walkdir`, `glob`, `serde`, `serde_json`,
`notify-debouncer-mini`, `rayon`

---

## Phase 1.5: Agent Implementation Breakdown

This section provides structured subtasks for implementation. Each meta-task includes:

- **What/Goal**: What the roadmap wants to achieve
- **Subtasks**: Specific implementation steps
- **Validation Checks**: Tests/confirmations to verify completion

### Meta-Task 1: Cargo.toml Dependency Setup

**What/Goal**: Add all 9 ecosystem crates to `Cargo.toml` with correct versions and features

**Subtasks**:

1. [ ] Add to `[dependencies]`:
   - `anyhow = "1.0"`
   - `tracing = "0.1"`
   - `tracing-subscriber = { version = "0.3", features = ["env-filter"] }`
   - `walkdir = "2.4"`
   - `glob = "0.3"`
   - `serde = { version = "1.0", features = ["derive"] }`
   - `serde_json = "1.0"`
   - `notify-debouncer-mini = "0.4"`
   - `rayon = "1.7"`

2. [ ] Run `cargo check` to verify all crates compile
3. [ ] Verify Cargo.lock is updated

**Validation Checks**:

```bash
# ✅ Must pass:
cargo check                          # No compilation errors
cargo tree | grep anyhow             # Verify anyhow is present
cargo tree | grep tracing            # Verify tracing is present
cargo tree | grep rayon              # Verify rayon is present
# Count dependencies: should be ~15-18 top-level (was ~5 before)
cargo tree --depth=0 | wc -l
```

---

### Meta-Task 2: Error Handling Refactor → `anyhow`

**What/Goal**: Replace all `Result<T, String>` with `Result<T, anyhow::Error>` and add error context
at boundaries

**Subtasks**:

1. [ ] In `src/lib.rs`: Add `use anyhow::{Context, Result};`
2. [ ] In `src/ir.rs`: Replace `Result<>` type signatures (if any)
3. [ ] In `src/discovery.rs`:
   - Replace `Result<Vec<PathBuf>, String>` → `Result<Vec<PathBuf>>`
   - Replace `Result<(), String>` → `Result<()>`
   - Add `.context("Failed to read directory: ...")` at file I/O
   - Change error strings to use `.context()` method
4. [ ] In `src/drivers/trait_def.rs`: Update trait signatures from `Result<T, String>` → `Result<T>`
5. [ ] In `src/drivers/rust.rs`:
   - Update all `Result<Blueprint, String>` → `Result<Blueprint>`
   - Replace error returns with `.context("Failed to parse Rust...")`
6. [ ] In `src/drivers/python.rs` (if exists): Same treatment
7. [ ] In `src/drivers/typescript.rs` (if exists): Same treatment
8. [ ] In `src/engine.rs`:
   - Replace `Result<(), String>` → `Result<()>`
   - Add context to all error sites (file read, layout format, write)
9. [ ] In `src/config.rs`: Replace error handling
10. [ ] In `src/io/fs_utils.rs`: Replace all error signatures
11. [ ] In `src/main.rs`:
    - Update match on `result` to display error chain: `eprintln!("{:?}", err);` or
      `eprintln!("{:#}", err);` for better error chains

**Validation Checks**:

```bash
# ✅ Must pass:
cargo check                          # Compiles with no errors
cargo build --release               # Release build succeeds
grep -r "Result<.*String>" src/     # Should return 0 matches (all converted)
grep -r "\.context(" src/           # Should find ~15-25 context additions

# ✅ Functional tests:
cargo test test_discovery           # Discovery tests still pass
cargo test test_rust_parser         # Parser tests still pass
cargo test                          # All tests pass

# ✅ Error chain verification:
# Run a command that should fail and verify error chain is helpful:
cargo run -- gen --source nonexistent 2>&1 | grep -q "Failed\|caused by"
# Should show contextual error message
```

---

### Meta-Task 3: Structured Logging → `tracing`

**What/Goal**: Replace all `println!`/`eprintln!` with `tracing` macros and wire up
environment-based filtering

**Subtasks**:

1. [ ] In `src/main.rs`:
   - Add `use tracing::{info, warn, error};`
   - Remove imports of `colored` or `ansi_term` (if any)
   - Initialize tracing subscriber:

     ```rust
     tracing_subscriber::fmt()
         .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
         .init();
     ```

   - Replace all `eprintln!("[verbose mode enabled]")` with `info!("Verbose mode enabled")`
   - Replace `eprintln!("Error: ...")` with `error!("...")`

2. [ ] In `src/engine.rs`:
   - Add `use tracing::{debug, info, warn};`
   - Replace `println!("📖 Config loaded...")` → `info!("Config loaded: layout={}", config.layout)`
   - Replace `println!("🔍 Discovering...")` → `debug!("Discovering source files")`
   - Replace `println!("🔨 Parsing...")` → `debug!("Parsing code")`
   - Replace `println!("   ✓ Parsed...")` → `debug!("Parsed file: path={} items={}", ...)`
   - Replace `eprintln!("   ⚠ Failed...")` → `warn!("Failed to parse: path={} error={}", ...)`
   - Replace `println!("📝 Writing...")` → `debug!("Writing outputs")`
   - Replace `println!("✅ Successfully...")` → `info!("Generation complete: output_dir={}", ...)`

3. [ ] In `src/discovery.rs`:
   - Add `use tracing::{debug, warn};`
   - Replace error logging with structured fields:

     ```rust
     warn!(directory = ?dir, "Directory not found");
     debug!(file_count = files.len(), "Discovery complete");
     ```

4. [ ] In `src/drivers/rust.rs`:
   - Add `use tracing::debug;`
   - Add debug logging at key parse points (function found, struct found, impl found)

5. [ ] Remove `--verbose` flag processing from main.rs (now controlled by `RUST_LOG`)
   - Delete: `if cli.verbose { eprintln!(...) }`
   - Leave CLI option defined (for backwards compat) but ignore it

**Validation Checks**:

```bash
# ✅ Must pass:
cargo check                          # Compiles
cargo test                           # All tests pass

# ✅ Logging verification (critical):
RUST_LOG=info cargo run -- gen 2>&1 | grep -q "Config loaded\|Generation complete"
# Should see info-level logs

RUST_LOG=debug cargo run -- gen 2>&1 | grep -q "Discovering\|Parsing"
# Should see debug-level logs

RUST_LOG=codetwin=warn cargo run -- gen 2>&1
# Should only see warn/error, fewer logs

# ✅ Backwards compat (--verbose is ignored gracefully):
cargo run -- gen --verbose 2>&1     # Should work without error
RUST_LOG=off cargo run -- gen 2>&1  # Should suppress logs
```

---

### Meta-Task 4: File Discovery Robustness → `walkdir` + `glob`

**What/Goal**: Replace manual recursion and brittle pattern matching with standard `walkdir` and
`glob` crates

**Subtasks**:

1. [ ] In `src/discovery.rs`:
   - Add imports: `use walkdir::WalkDir;` and `use glob::Pattern;`
   - Replace `fn find_rs_files_recursive()` entirely:
     - Use `WalkDir::new(dir).into_iter()` instead of `fs::read_dir()`
     - Keep `.rs` extension filter
     - Use glob patterns from config for excluding

   - Replace `fn should_skip()` logic:
     - Build glob patterns from `exclude_patterns` config (e.g., `"**/target/**"`)
     - Test each file path against patterns
     - More robust than hardcoded names

2. [ ] In `src/config.rs`:
   - Verify default `exclude_patterns` are correct glob syntax:

     ```rust
     "**/target/**",
     "**/node_modules/**",
     "**/.git/**",
     "**/tests/**",
     ```

3. [ ] Update tests in `tests/test_discovery.rs`:
   - Test pattern `"**/target/**"` skips target/ recursively
   - Test pattern `"**/.git/**"` skips .git/ files
   - Test that regular `.rs` files are still found

**Validation Checks**:

```bash
# ✅ Must pass:
cargo check
cargo test test_discovery            # Discovery tests pass

# ✅ Functional verification:
# Create temp structure
mkdir -p /tmp/test_discover/src /tmp/test_discover/target/debug
touch /tmp/test_discover/src/lib.rs /tmp/test_discover/target/debug/main.rs

# Run discovery (creates minimal test or uses existing)
# Should find src/lib.rs but NOT target/debug/main.rs
cargo test test_discovery -- --nocapture | grep -q "Found.*src"

# ✅ Pattern matching verification:
# Verify glob patterns compile and work (add test):
# Test: "src/**/*.rs" matches "src/drivers/rust.rs"
# Test: "**/target/**" skips any target/ dir
# Test: "**/.git/**" skips .git/ entirely
```

---

### Meta-Task 5: JSON Output Implementation → `serde`

**What/Goal**: Make IR types serializable and wire `--json` CLI flag to output structured JSON
instead of Markdown

**Subtasks**:

1. [ ] In `src/ir.rs`:
   - Add derive: `#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]`
   - To all types: `Blueprint`, `Element`, `Class`, `Function`, `Method`, `Property`, `Module`,
     `Signature`, `Parameter`, `Visibility`, `Documentation`
   - Add imports: `use serde::{Serialize, Deserialize};`

2. [ ] In `src/config.rs`:
   - Add `#[derive(Serialize, Deserialize)]` to `Config` and `Layer`

3. [ ] In `src/engine.rs`:
   - Add JSON output mode to `generate()` function
   - Check if `config.json_output` flag is set (or pass as param)
   - If JSON: instead of calling `layout.format()`, serialize blueprints to JSON:

     ```rust
     let json_output = serde_json::json!({
         "blueprints": blueprints,
         "config": config,
         "generated_at": chrono::Local::now().to_rfc3339()
     });
     fs::write(&output_file, json_output.to_string_pretty())?;
     ```

4. [ ] In `src/main.rs`:
   - In the `Gen` command handler, check if `--json` flag is set (or `config.json_output`)
   - If true, pass to engine or handle JSON serialization there

5. [ ] In `src/cli.rs`:
   - (Confirm `--json` flag exists already from Phase 1)
   - Add to `Gen` command or make it global if not present

**Validation Checks**:

```bash
# ✅ Must pass:
cargo check
cargo test                           # All tests pass

# ✅ Serialization verification:
cargo test --lib | grep -q "serde"  # If any serde tests exist

# ✅ Functional JSON output:
cargo run -- gen --json 2>&1 | jq . > /tmp/out.json
# Should produce valid JSON
jq '.blueprints[0]' /tmp/out.json | grep -q '"name"'
# JSON should have expected structure

# ✅ Output format:
cargo run -- gen --json 2>&1 | head -20
# Should show JSON with "blueprints", "config", "generated_at" fields

# ✅ Backwards compat:
cargo run -- gen 2>&1                # Should still produce Markdown (default)
```

---

### Meta-Task 6: Watch Mode Implementation → `notify-debouncer-mini`

**What/Goal**: Implement file system watcher that detects changes in source files and
auto-regenerates documentation with debouncing

**Subtasks**:

1. [ ] In `src/engine.rs`:
   - Implement `pub fn watch(&self, config: &Config) -> Result<()>` (currently returns "Not
     implemented")
   - Use `notify_debouncer_mini::new_debouncer()` to create watcher
   - Watch directories from `config.source_dirs`
   - Set debounce duration (200ms default, override from CLI)
   - On file change event:
     - Detect if it's a `.rs`/`.py`/`.ts` file
     - Call `self.generate(config)` to regenerate
     - Log output with timestamp

   - Implement graceful shutdown (Ctrl+C handling):

     ```rust
     std::thread::park(); // Blocks until loop breaks (Ctrl+C)
     ```

2. [ ] In `src/main.rs`:
   - In `Commands::Watch` handler:
     - Create config (load or defaults)
     - Apply ephemeral flag overrides (like `gen` does)
     - Call `engine.watch(&config)?`

3. [ ] Add watch debounce timing logic:
   - Default: 300ms (from CLI default in Phase 1)
   - Rapid file writes (within 300ms) → debounce, single regen
   - Test: write 5 files in 100ms → only 1 regen triggered

**Validation Checks**:

```bash
# ✅ Must pass:
cargo check
cargo build                          # Compiles without error

# ✅ Watch mode functional test:
# Create a test integration test or manual verification
cargo run -- watch &
PID=$!

# Modify a source file
sleep 1 && touch src/lib.rs

# Wait for regeneration
sleep 2

# Check docs were updated
ls -l docs/architecture.md | tail -5

# Cleanup
kill $PID

# ✅ Debouncing verification:
# Create test that writes multiple files quickly
# Verify only single regeneration (not 5)
# Can add test: `tests/test_watch_debounce.rs`
# That writes temp files and counts regeneration count

# ✅ Signal handling:
cargo run -- watch &
PID=$!
sleep 1
kill $PID  # Should exit cleanly, no panic
wait      # Should finish without hanging
```

---

### Meta-Task 7: Parallel File Parsing → `rayon`

**What/Goal**: Convert sequential file parsing loop to parallel iteration for 2-3x speedup on
multi-file projects

**Subtasks**:

1. [ ] In `src/engine.rs`:
   - Add import: `use rayon::prelude::*;`
   - In `generate()` function, find the loop:

     ```rust
     for file_path in files { ... }
     ```

   - Replace with:

     ```rust
     let blueprints: Vec<Blueprint> = files
         .par_iter()
         .filter_map(|file_path| {
             let source = fs::read_to_string(file_path).ok()?;
             let driver = drivers::get_driver_for_file(file_path)?;
             driver.parse(&source).ok()?
         })
         .collect();
     ```

   - Ensure proper error handling (collect Results, filter Errors if acceptable, or gather them)

2. [ ] Add benchmarking:
   - Create simple benchmark: `cargo build --release && time cargo run --release -- gen`
   - Must not be slower than sequential (rayon actually helps on 10+ files)

3. [ ] Verify thread safety:
   - Each thread gets its own `Parser` instance (tree-sitter is not Send by default)
   - May need to use thread-local storage or re-parse per thread

**Validation Checks**:

```bash
# ✅ Must pass:
cargo check
cargo build --release               # Release build (rayon optimized)

# ✅ Compilation with rayon:
cargo tree | grep rayon             # Verify rayon is in deps

# ✅ Functional test:
cargo test                          # All tests pass

# ✅ Output consistency:
# Sequential parse (in separate build):
# cargo run -- gen 2>&1 > /tmp/seq_output.txt
# Parallel parse (current):
# cargo run -- gen 2>&1 > /tmp/par_output.txt
# diff /tmp/seq_output.txt /tmp/par_output.txt  # Should be identical

# ✅ Performance check (informational):
time cargo run --release -- gen 2>&1
# Should complete in <5s for typical repo (vs ~10-15s sequential on large repos)
# Can add benchmark task if time allows

# ✅ Thread safety verification:
# Run multiple times in quick succession:
for i in {1..5}; do cargo run -- gen > /dev/null 2>&1; done
# No crashes or deadlocks
```

---

### Meta-Task 8: Integration Tests for Phase 1.5

**What/Goal**: Comprehensive test suite to validate all Phase 1.5 features work correctly together

**Subtasks**:

1. [ ] Create new tests or add to existing `tests/` directory:

   **Test: Error Context Chains**

   ```rust
   #[test]
   fn test_error_context_nonexistent_dir() {
       // Try to generate from nonexistent directory
       // Verify error message includes "Failed to read directory" context
       // Check error display includes helpful path info
   }
   ```

   **Test: Logging Output (RUST_LOG)**

   ```rust
   #[test]
   fn test_logging_info_level() {
       // Set RUST_LOG=info
       // Run generate
       // Capture logs, verify "Config loaded" appears
   }

   #[test]
   fn test_logging_debug_level() {
       // Set RUST_LOG=debug
       // Run generate
       // Verify "Discovering" and "Parsing" appear
   }
   ```

   **Test: File Discovery with Glob Patterns**

   ```rust
   #[test]
   fn test_discover_excludes_target_dir() {
       // Create temp structure with target/ subdir
       // Run discovery
       // Verify target/ files not included
   }

   #[test]
   fn test_discover_with_nested_patterns() {
       // Test "**/.git/**", "src/**/*.rs"
       // Verify patterns work correctly
   }
   ```

   **Test: JSON Output Format**

   ```rust
   #[test]
   fn test_json_output_format() {
       // Generate with --json flag
       // Parse output as JSON
       // Verify structure has "blueprints", "config", "generated_at"
       // Verify no Markdown syntax present
   }

   #[test]
   fn test_json_serialization_roundtrip() {
       // Serialize IR to JSON
       // Deserialize back
       // Verify equality
   }
   ```

   **Test: Watch Mode Stability**

   ```rust
   #[test]
   fn test_watch_mode_debouncing() {
       // Spawn watch in background/thread
       // Write 5 files rapidly (50ms apart)
       // Wait 500ms
       // Verify only 1 regeneration triggered (not 5)
   }

   #[test]
   fn test_watch_mode_signal_handling() {
       // Spawn watch
       // Send Ctrl+C (SIGINT)
       // Verify graceful exit (no panic, no hang)
   }
   ```

   **Test: Parallel Parsing Consistency**

   ```rust
   #[test]
   fn test_parallel_vs_sequential_output() {
       // Parse files sequentially → output_seq
       // Parse files in parallel → output_par
       // Verify output_seq == output_par (order-independent comparison)
   }
   ```

2. [ ] Run full test suite:

   ```bash
   cargo test --all
   ```

**Validation Checks**:

```bash
# ✅ Must pass:
cargo test --all --release 2>&1 | grep -E "test result:|passed"
# All tests pass (should show "test result: ok" at end)

# ✅ Individual test coverage:
cargo test test_error_context --      # Error handling test
cargo test test_logging --             # Logging test
cargo test test_discover --            # Discovery test
cargo test test_json --                # JSON test
cargo test test_watch --               # Watch mode test
cargo test test_parallel --            # Parallel parsing test

# ✅ Coverage report (optional):
# Can add tarpaulin or llvm-cov for coverage percentage

# ✅ No warnings:
cargo test --all 2>&1 | grep -i "warning"
# Should return 0 matches (clean build)
```

---

### Phase 1.5 Completion Checklist

After all 8 meta-tasks complete, verify:

```
PHASE 1.5 COMPLETION CRITERIA
=============================

[Cargo.toml]
✅ 9 new crates added to [dependencies]
✅ cargo check passes
✅ cargo build --release succeeds

[Error Handling]
✅ No `Result<T, String>` in src/ (grep returns 0)
✅ ~15-25 `.context()` calls added
✅ All existing tests still pass

[Logging]
✅ No `println!` or `eprintln!` in src/ except main errors
✅ RUST_LOG=info works and shows logs
✅ RUST_LOG=debug shows additional detail
✅ Logs contain useful context (file names, counts, etc.)

[File Discovery]
✅ Uses `walkdir` instead of manual recursion
✅ Glob patterns work: "**/**/target/**" correctly skips nested targets
✅ test_discovery passes

[JSON Output]
✅ --json flag produces valid JSON
✅ JSON has "blueprints", "config", "generated_at"
✅ Default (no flag) still produces Markdown
✅ Serialization round-trip works

[Watch Mode]
✅ cargo run -- watch starts without error
✅ Watch detects file changes
✅ Debouncing works (5 rapid writes = 1 regen)
✅ Graceful exit on Ctrl+C

[Parallel Parsing]
✅ cargo build --release compiles
✅ Parallel and sequential outputs match
✅ Performance benchmark shows <5s for typical repo

[Integration Tests]
✅ 8+ tests in tests/ directory
✅ All tests pass: cargo test --all
✅ No compilation warnings

[Overall]
✅ cargo test --all passes (100% of tests)
✅ cargo build --release succeeds
✅ cargo clippy reports no warnings
✅ README/docs updated if needed

ESTIMATED COMPLETION TIME: 12-18 hours for competent agent
```

---

### 1. Error Handling & Logging Refactor

**Why**: Current `Result<T, String>` scattered throughout codebase; manual `println!`/`eprintln!`
lacks context.

1. [ ] **Replace error handling with `anyhow`**
   - Change all `Result<T, String>` → `Result<T>` (anyhow::Result)
   - Add `.context("description")` at error boundaries
   - Enables error chains: `Error: Failed to parse rust.rs caused by: Unexpected token`
   - Files to refactor: `engine.rs`, `drivers/*.rs`, `io/*.rs`, `discovery.rs`
   - Crate: `anyhow = "1.0"`

2. [ ] **Structured logging with `tracing`**
   - Replace `println!`/`eprintln!` with `tracing::info!`, `tracing::warn!`, `tracing::debug!`
   - Initialize subscriber in `main.rs`: `tracing_subscriber::fmt().with_env_filter(...).init()`
   - Remove manual `--verbose` flag checks; use `RUST_LOG` env var instead
   - Benefits: `RUST_LOG=debug ctw gen` shows parse-level details; `RUST_LOG=codetwin=info`
     per-module control
   - Files to refactor: `main.rs`, `engine.rs`, `drivers/*.rs`, `discovery.rs`
   - Crates: `tracing = "0.1"`,
     `tracing-subscriber = { version = "0.3", features = ["env-filter"] }`

3. [ ] **Add tests for error contexts**
   - Verify error messages include helpful context
   - Update `tests/` with error propagation checks

### 2. File Discovery Robustness

**Why**: Current manual directory walk in `discovery.rs` is error-prone; exclude patterns are
brittle string matching.

1. [ ] **Replace manual recursion with `walkdir`**
   - Simplify `find_rs_files_recursive()` → use `walkdir::WalkDir`
   - Fewer bugs, better error handling, standard library
   - Files to refactor: `src/discovery.rs`
   - Crate: `walkdir = "2.4"`

2. [ ] **Robust pattern matching with `glob`**
   - Replace hardcoded `should_skip()` logic with glob pattern matching
   - Supports `**/*.rs`, `**/target/**`, etc. correctly
   - Update config to store glob patterns (already there: `exclude_patterns`)
   - Files to refactor: `src/discovery.rs`, `src/config.rs`
   - Crate: `glob = "0.3"`

3. [ ] **Test discovery with varied patterns**
   - Add tests for complex exclude patterns
   - Verify `**/.git/**`, `**/node_modules/**` work correctly
   - Test: symlink handling, deeply nested structures

### 3. JSON Output Implementation

**Why**: CLI has `--json` flag (Phase 1) but no implementation.

1. [ ] **Serialize IR with `serde`**
   - Derive `Serialize` on: `Blueprint`, `Element`, `Class`, `Function`, `Method`, etc.
   - Output JSON representation of parsed code structure
   - Files to refactor: `src/ir.rs`, `src/engine.rs`, `src/main.rs`
   - Crates: `serde = { version = "1.0", features = ["derive"] }`, `serde_json = "1.0"`

2. [ ] **Wire `--json` CLI flag**
   - When `--json` set, instead of Markdown output, write JSON to stdout or file
   - Format: `{ "blueprints": [...], "config": {...} }`
   - Update `SyncEngine::generate()` to support JSON output mode
   - Files to refactor: `src/cli.rs`, `src/engine.rs`, `src/main.rs`

3. [ ] **Add tests for JSON output**
   - Parse JSON output and verify structure
   - Test: all IR types serialize/deserialize correctly

### 4. Watch Mode Implementation

**Why**: CLI has `watch` command (Phase 1) but returns "Not implemented"; users need
auto-regeneration.

1. [ ] **File system monitoring with `notify-debouncer-mini`**
   - Watch directories from config for changes: `.rs`, `.py`, `.ts` files
   - Debounce rapid changes (default 300ms from CLI)
   - Trigger `generate()` on file event
   - Files to refactor: `src/engine.rs` (add `watch()` impl), `src/main.rs`
   - Crate: `notify-debouncer-mini = "0.4"`

2. [ ] **Watch loop with graceful shutdown**
   - Loop: watch → detect change → debounce → regenerate → loop
   - Handle Ctrl+C gracefully
   - Log each regeneration event with timestamps

3. [ ] **Test watch mode**
   - Create temp files, verify regeneration triggered
   - Test debouncing (rapid writes don't trigger multiple regen)

### 5. Parallel File Parsing Setup

**Why**: Multi-language (Phase 3) will parse Python + TypeScript + Rust simultaneously; sequential
is too slow.

1. [ ] **Add `rayon` for parallel parsing**
   - In `SyncEngine::generate()`, replace: `for file_path in files` with
     `files.par_iter().map(|file_path|`
   - Thread-local Blueprint accumulation
   - Files to refactor: `src/engine.rs`
   - Crate: `rayon = "1.7"`

2. [ ] **Benchmark: sequential vs parallel**
   - Test on 50+ file codebase (e.g., full `src/` of codetwin)
   - Document speedup ratio

3. [ ] **Ensure thread safety**
   - Verify drivers (tree-sitter instances) are thread-safe
   - Add tests with parallel flag

### 6. Integration Tests for Phase 1.5

1. [ ] **Test error context chains**
   - Verify `anyhow` error messages include file path, line number if available

2. [ ] **Test logging output**
   - Set `RUST_LOG=debug` and verify driver logs appear

3. [ ] **Test discovery with excludes**
   - Verify glob patterns work as expected

4. [ ] **Test watch mode stability**
   - Run watch for 30+ events, no hangs

5. [ ] **Test parallel parsing consistency**
   - Parse same files sequentially and in parallel; outputs match

**Build Status After Phase 1.5**: All tests pass, improved error messages, structured logs, watch
mode works, 2-3x faster parsing on multi-file projects

---

## Phase 2: Core Layout Implementations

**Goal**: Implement 3 essential layouts for different use cases

**Dependencies**: [needs Phase 1] ✅ [benefits from Phase 1.5]

**Parallelization**: Layout 1 (Dependency Graph) ✅ COMPLETE. Layouts 2 & 3 can be implemented in
parallel.

**Note**: Layout 1 validation complete. Current status in [Completed](#completed-) section below.

### Layout 2: Layered Architecture

_Best for: Design pattern analysis and architecture reviews_

**Dependencies**: [needs Phase 2 Layout 1]

1. [ ] Create `src/layouts/layered.rs`
2. [ ] Define layer schema in `codetwin.toml`:

   ```toml
   [[layers]]
   name = "User Interface"
   pattern = ["main.rs", "cli.rs"]

   [[layers]]
   name = "Orchestration"
   pattern = ["engine.rs"]

   # ... etc with sensible defaults
   ```

3. [ ] Auto-detect layers if not configured (group by directory depth)
4. [ ] Generate Mermaid diagram with layer boxes + arrows
5. [ ] Add prose section per layer explaining responsibilities
6. [ ] List modules in each layer with their key functions
7. [ ] Add tests: layer grouping, default detection
8. [ ] Document: "Shows architectural layers and separation of concerns"

### Layout 3: README-Embedded

_Best for: GitHub discovery and quick reference_

1. [ ] Create `src/layouts/readme_embedded.rs`
2. [ ] Generate compact summary section for README.md
3. [ ] Embed dependency graph (Mermaid code block)
4. [ ] Add component table (module | purpose | key functions)
5. [ ] Include "Data Flow" step-by-step
6. [ ] Add "Development Guide" with contribution patterns
7. [ ] Support README template with placeholders
8. [ ] Add tests: verify README structure, diagram embedding
9. [ ] Document: "Single source of truth for GitHub visitors"

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

1. [ ] **Python Driver** (`src/drivers/python.rs`)
   - Use `tree-sitter-python` for AST parsing
   - Extract: classes, functions, imports, decorators
   - Map to IR (Blueprint → Elements)
   - Handle Python-specific patterns (dunder methods, properties)
   - Leverage structured logging from Phase 1.5 for per-driver debug output
   - Crate: `tree-sitter-python = "0.21"`

2. [ ] **TypeScript Driver** (`src/drivers/typescript.rs`)
   - Use `tree-sitter-typescript` for AST parsing
   - Extract: classes, interfaces, functions, imports, exports
   - Handle TypeScript-specific: generics, type annotations, namespaces
   - Leverage structured logging from Phase 1.5 for per-driver debug output
   - Crate: `tree-sitter-typescript = "0.21"`

3. [ ] **Multi-Language Integration**
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

4. [ ] **PyPI (Python ecosystem)** (optional, if demand exists)
   - Create Python wrapper using maturin/pyo3
   - Build wheels for Linux/macOS/Windows
   - Publish to PyPI
   - Test: `pip install codetwin`

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

**Next Focus**: Phase 1.5 Infrastructure enables faster Phase 3 (multi-language) with parallel
parsing and structured logging

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
