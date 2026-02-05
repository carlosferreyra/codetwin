# Agent Instructions: Phase 1.5 Implementation

**Time Estimate**: 12-18 hours **Status**: Ready for agent handoff **Current Project State**: Phase
1 ✅ + Phase 2 Layout 1 ✅

---

## Quick Start

1. Read this entire file (5 min)
2. Start **Meta-Task 1** below
3. After each meta-task, run the **Validation Checks**
4. Move to next meta-task only after validation passes
5. Use ROADMAP.md for detailed implementation specs

**Critical**: Do NOT skip validation checks. Each one confirms success before proceeding.

---

## What is Phase 1.5?

Transform CodeTwin codebase to use modern Rust ecosystem crates before Phase 3 (multi-language
support). This adds:

- **Better error handling** (`anyhow`)
- **Structured logging** (`tracing`)
- **Robust file discovery** (`walkdir`, `glob`)
- **JSON output** (`serde_json`)
- **Watch mode** (`notify-debouncer-mini`)
- **Parallel parsing** (`rayon`)

**Why now?** Phase 3 needs these for 2-3x speedup and better error chains when parsing multiple
languages.

---

## Meta-Task Checklist

- [ ] **Meta-Task 1**: Cargo.toml Setup (30 min)
- [ ] **Meta-Task 2**: Error Handling (`anyhow`) (2-3 hrs)
- [ ] **Meta-Task 3**: Logging (`tracing`) (2-3 hrs)
- [ ] **Meta-Task 4**: File Discovery (`walkdir`/`glob`) (1.5-2 hrs)
- [ ] **Meta-Task 5**: JSON Output (`serde_json`) (1.5-2 hrs)
- [ ] **Meta-Task 6**: Watch Mode (`notify-debouncer-mini`) (2-3 hrs)
- [ ] **Meta-Task 7**: Parallel Parsing (`rayon`) (1-1.5 hrs)
- [ ] **Meta-Task 8**: Integration Tests (2-3 hrs)

**Total: 12-18 hours**

---

## Meta-Task 1: Cargo.toml Dependency Setup

**Goal**: Add 9 ecosystem crates to Cargo.toml

**Files to modify**: `Cargo.toml`

### Subtasks

1. Add these to `[dependencies]` section:

```toml
anyhow = "1.0"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
walkdir = "2.4"
glob = "0.3"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
notify-debouncer-mini = "0.4"
rayon = "1.7"
```

2. Run `cargo check` to verify compilation

### Validation Checks ✅

```bash
# Must all pass:
cargo check                          # No errors
cargo tree | grep anyhow             # Verify present
cargo build --release                # Release build works
```

**✅ PASS**: All commands succeed, no warnings **❌ FAIL**: Any compilation error → fix before
proceeding

---

## Meta-Task 2: Error Handling Refactor → `anyhow`

**Goal**: Replace all `Result<T, String>` with `Result<T>` + add error context

**Files to modify**: `src/lib.rs`, `src/main.rs`, `src/engine.rs`, `src/discovery.rs`,
`src/drivers/*.rs`, `src/config.rs`, `src/io/*.rs`, `src/ir.rs`

### Key Changes

1. Add to each file:

   ```rust
   use anyhow::{Context, Result};
   ```

2. Replace all occurrences:
   - `Result<T, String>` → `Result<T>`
   - `Err("message".to_string())` → `Err(anyhow::anyhow!("message"))`
   - Add `.context("descriptive message")` at file I/O boundaries

3. In `src/main.rs` error handling:
   ```rust
   let result: Result<()> = match cli.command { ... };
   if let Err(err) = result {
       eprintln!("{:#}", err);  // Pretty-print error chain
       std::process::exit(1);
   }
   ```

### Validation Checks ✅

```bash
# Critical validations:
cargo check                          # Compiles
cargo build --release                # Release builds
grep -r "Result<.*String>" src/      # Should return 0 matches
grep -r "\.context(" src/            # Should find 15-25+ matches

# Functional:
cargo test                           # All tests pass
cargo run -- gen --source nonexistent 2>&1 | head -5
# Should show contextual error (not just "Failed to read")
```

**✅ PASS**: No `Result<T, String>` found, tests pass, errors have context **❌ FAIL**: Found
`Result<T, String>` or tests fail → fix before proceeding

---

## Meta-Task 3: Structured Logging → `tracing`

**Goal**: Replace `println!`/`eprintln!` with `tracing` macros

**Files to modify**: `src/main.rs`, `src/engine.rs`, `src/discovery.rs`, `src/drivers/*.rs`

### Key Changes

1. In `src/main.rs`, add to start of `main()`:

   ```rust
   use tracing::{info, warn, error, debug};

   tracing_subscriber::fmt()
       .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
       .init();
   ```

2. Replace patterns:
   - `println!("📖 Config loaded...")` → `info!("Config loaded: layout={}", config.layout)`
   - `println!("🔍 Discovering...")` → `debug!("Discovering source files")`
   - `eprintln!("Error: ...")` → `error!("...")`

3. In each file, add imports:

   ```rust
   use tracing::{info, debug, warn, error};
   ```

4. Remove manual `--verbose` handling (keep flag for compat, just ignore it)

### Validation Checks ✅

```bash
# Critical validations:
cargo check
cargo build --release
cargo test                           # All tests pass

# Logging verification:
RUST_LOG=info cargo run -- gen 2>&1 | grep -q "Config\|Generation\|discovered"
# Should see info-level logs

RUST_LOG=debug cargo run -- gen 2>&1 | grep -q "Discovering\|Parsing"
# Should see debug-level logs

RUST_LOG=warn cargo run -- gen 2>&1 | wc -l
# Should be minimal output (only warnings)

# Backwards compat:
cargo run -- gen --verbose 2>&1     # Should work without error
```

**✅ PASS**: RUST_LOG env var controls output, tests pass **❌ FAIL**: Logs don't respond to
RUST_LOG or tests fail → fix before proceeding

---

## Meta-Task 4: File Discovery → `walkdir` + `glob`

**Goal**: Replace manual recursion with `walkdir` and brittle patterns with `glob`

**Files to modify**: `src/discovery.rs`, `src/config.rs`

### Key Changes

1. In `src/discovery.rs`:
   - Add imports: `use walkdir::WalkDir;` + `use glob::Pattern;`
   - Replace `fn find_rs_files_recursive()` to use `WalkDir::new(dir).into_iter()`
   - Replace `fn should_skip()` to use glob patterns from config

2. Logic:

   ```rust
   let walker = WalkDir::new(dir)
       .into_iter()
       .filter_map(Result::ok)
       .filter(|e| !should_skip(e.path(), &exclude_patterns))
       .filter_map(|e| {
           if e.path().extension()?.to_str()? == "rs" {
               Some(e.path().to_path_buf())
           } else {
               None
           }
       });
   ```

3. In `src/config.rs`, verify defaults are valid glob syntax:
   ```rust
   exclude_patterns: vec![
       "**/target/**".to_string(),
       "**/node_modules/**".to_string(),
       "**/.git/**".to_string(),
       "**/tests/**".to_string(),
   ]
   ```

### Validation Checks ✅

```bash
# Critical validations:
cargo check
cargo test test_discovery            # Discovery tests pass
cargo test                           # All tests pass

# Functional - create test structure:
mkdir -p /tmp/test_co/src /tmp/test_co/target/debug
touch /tmp/test_co/src/lib.rs /tmp/test_co/target/debug/main.rs
# Manually verify with a quick test that target/ is excluded

# Pattern verification:
# Verify in tests that glob patterns work:
# - "**/target/**" excludes nested target/ dirs
# - "**/.git/**" excludes .git/ everywhere
# - ".rs" files ARE found in src/
```

**✅ PASS**: Tests pass, target/ files excluded, .rs files found **❌ FAIL**: Tests fail or patterns
don't work → fix before proceeding

---

## Meta-Task 5: JSON Output → `serde`

**Goal**: Serialize IR to JSON and wire `--json` CLI flag

**Files to modify**: `src/ir.rs`, `src/config.rs`, `src/engine.rs`, `src/main.rs`

### Key Changes

1. In `src/ir.rs`, add derives to ALL types:

   ```rust
   use serde::{Serialize, Deserialize};

   #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
   pub struct Blueprint { ... }

   // Apply to: Blueprint, Element, Class, Function, Method, Property,
   // Module, Signature, Parameter, Visibility, Documentation
   ```

2. In `src/config.rs`:

   ```rust
   #[derive(Serialize, Deserialize)]
   pub struct Config { ... }

   #[derive(Serialize, Deserialize)]
   pub struct Layer { ... }
   ```

3. In `src/engine.rs`, add JSON mode:

   ```rust
   if output_should_be_json {  // Check from config or flag
       let json_output = serde_json::json!({
           "blueprints": blueprints,
           "config": config,
           "generated_at": chrono::Local::now().to_rfc3339()
       });
       fs::write(&output_file, json_output.to_string_pretty())?;
   } else {
       // Normal Markdown flow
   }
   ```

4. In `src/main.rs`, wire `--json` flag to engine

### Validation Checks ✅

```bash
# Critical validations:
cargo check
cargo build --release
cargo test                           # All tests pass

# JSON output verification:
cargo run -- gen --json 2>&1 | head -20
# Should see JSON output, not Markdown

cargo run -- gen --json 2>&1 | jq . > /tmp/out.json
# Should parse as valid JSON

jq '.blueprints[0]' /tmp/out.json | grep -q '"name"'
# JSON should have expected structure

# Backwards compat:
cargo run -- gen 2>&1 | head -5
# Should still produce Markdown (default behavior)
```

**✅ PASS**: JSON output valid and structured, Markdown still works by default **❌ FAIL**: JSON
invalid or Markdown broken → fix before proceeding

---

## Meta-Task 6: Watch Mode → `notify-debouncer-mini`

**Goal**: Implement file watcher with debouncing that auto-regenerates docs

**Files to modify**: `src/engine.rs`, `src/main.rs`

### Key Changes

1. In `src/engine.rs`, implement `watch()`:

   ```rust
   use notify_debouncer_mini::new_debouncer;
   use std::sync::mpsc;
   use std::time::Duration;

   pub fn watch(&self, config: &Config, debounce_ms: u64) -> Result<()> {
       let (tx, rx) = mpsc::channel();
       let mut debouncer = new_debouncer(Duration::from_millis(debounce_ms), tx)?;

       for source_dir in &config.source_dirs {
           debouncer.watch(Path::new(source_dir), RecursiveMode::Recursive)?;
       }

       loop {
           match rx.recv() {
               Ok(_) => {
                   info!("File change detected, regenerating...");
                   self.generate(config)?;
               }
               Err(_) => break,
           }
       }
   }
   ```

2. In `src/main.rs` Commands::Watch handler:
   ```rust
   Commands::Watch { output, layout, source, debounce, exclude } => {
       let mut config = Config::load_or_defaults("codetwin.toml");
       // Apply ephemeral overrides (same as gen)
       engine.watch(&config, debounce)?
   }
   ```

### Validation Checks ✅

```bash
# Critical validations:
cargo check
cargo build --release
cargo test                           # All tests pass

# Watch mode functional test:
cargo run -- watch &
WATCH_PID=$!
sleep 2

# Modify a source file
touch src/lib.rs

# Wait for regeneration
sleep 2

# Check output was updated
ls -lart docs/ | tail -1

# Verify recency
kill $WATCH_PID
wait

# Debouncing verification:
# Add test in tests/: rapid writes should trigger only 1 regen
# Can be manual: write 5 files in 100ms, verify single regeneration
```

**✅ PASS**: Watch detects changes, debouncing works, graceful exit on Ctrl+C **❌ FAIL**: Watch
hangs, no debouncing, or crashes → fix before proceeding

---

## Meta-Task 7: Parallel Parsing → `rayon`

**Goal**: Convert sequential file parsing to parallel for 2-3x speedup

**Files to modify**: `src/engine.rs`

### Key Changes

1. In `src/engine.rs`, in `generate()` function:

   ```rust
   use rayon::prelude::*;

   // OLD:
   // for file_path in files {
   //     let source = fs::read_to_string(&file_path)?;
   //     ...
   // }

   // NEW:
   let blueprints: Vec<Blueprint> = files
       .par_iter()
       .filter_map(|file_path| {
           let source = fs::read_to_string(file_path).ok()?;
           if let Some(driver) = drivers::get_driver_for_file(file_path) {
               driver.parse(&source).ok()
           } else {
               None
           }
       })
       .collect();
   ```

### Validation Checks ✅

```bash
# Critical validations:
cargo check
cargo build --release
cargo test                           # All tests pass

# Consistency check:
cargo run -- gen > /tmp/output1.md 2>&1
cargo run -- gen > /tmp/output2.md 2>&1
diff /tmp/output1.md /tmp/output2.md
# Should be identical (order-independent comparison if needed)

# Performance (informational):
time cargo run --release -- gen
# Should complete in <5 seconds for typical repo
```

**✅ PASS**: Outputs identical, tests pass, no performance regression **❌ FAIL**: Outputs differ or
tests fail → fix before proceeding

---

## Meta-Task 8: Integration Tests

**Goal**: Write comprehensive tests for all Phase 1.5 features

**Files to create/modify**: `tests/test_phase1_5.rs` (new file)

### Key Tests to Add

1. **Error Context**: Verify error chains include helpful context
2. **Logging**: Set `RUST_LOG` env vars and verify output
3. **Discovery**: Test glob patterns exclude correctly
4. **JSON**: Parse JSON output and verify structure
5. **Watch**: Test debouncing (rapid writes = 1 regen)
6. **Parallel**: Verify parallel output matches sequential
7. **Integration**: Full end-to-end with all features

### Validation Checks ✅

```bash
# Critical validations:
cargo test --all 2>&1 | tail -20
# Should see "test result: ok" at end

cargo test --all -- --nocapture 2>&1 | grep -E "passed|failed"
# All tests should pass

cargo test --release 2>&1
# Release build tests also pass

cargo clippy 2>&1 | grep -i warning
# Should return 0 (no warnings from new code)
```

**✅ PASS**: All tests pass, no warnings, >95% of new code covered **❌ FAIL**: Tests fail or
warnings present → fix before proceeding

---

## Final Phase 1.5 Completion Check

After all 8 meta-tasks, run this checklist:

```bash
# BUILD & COMPILATION
✅ cargo check              # Compiles
✅ cargo build --release    # Release build works
✅ cargo clippy             # No warnings

# TESTING
✅ cargo test --all         # All tests pass
✅ cargo test --release     # Release tests pass

# FEATURE VERIFICATION
✅ grep -r "Result<.*String>" src/       # 0 matches (all converted)
✅ cargo run -- gen 2>&1 | jq .          # JSON output works
✅ RUST_LOG=debug cargo run -- gen 2>&1  # Logging works
✅ cargo run -- watch &                  # Watch starts

# PERFORMANCE
✅ time cargo run --release -- gen       # <5 seconds typical

# DEPENDENCIES
✅ cargo tree | grep -E "anyhow|tracing|rayon|walkdir"  # All present
```

---

## Troubleshooting

### Issue: `cargo check` fails with "cannot find crate"

**Solution**: Run `cargo update` and retry. Ensure Cargo.toml has all 9 crates.

### Issue: Tests fail after error handling changes

**Solution**: Verify all `Err()` calls use `anyhow::anyhow!()` or `.context()`. Check that trait
implementations are updated.

### Issue: Logs not appearing

**Solution**: Ensure `main.rs` has `tracing_subscriber::fmt().init()` at start. Verify no
`RUST_LOG=off`.

### Issue: Watch mode hangs

**Solution**: Verify signal handling for Ctrl+C. Check notify crate is initialized correctly.

### Issue: Parallel parsing crashes

**Solution**: Verify tree-sitter `Parser` is thread-local. May need to create new Parser per thread
with try_lock patterns.

---

## Quick Links

- **Detailed Specs**: See `ROADMAP.md` section "Phase 1.5: Agent Implementation Breakdown"
- **Current Status**: Phase 1 ✅, Phase 2 Layout 1 ✅
- **Next After Phase 1.5**: Phase 2 Layouts 2-3 & Phase 3 (multi-language)

---

## Summary

| Meta-Task         | Time          | Status | Pass Condition                 |
| ----------------- | ------------- | ------ | ------------------------------ |
| 1. Cargo Setup    | 30 min        | ⏳     | cargo check ✅                 |
| 2. Error Handling | 2-3 hrs       | ⏳     | No Result<T, String>, tests ✅ |
| 3. Logging        | 2-3 hrs       | ⏳     | RUST_LOG works ✅              |
| 4. Discovery      | 1.5-2 hrs     | ⏳     | Patterns work ✅               |
| 5. JSON Output    | 1.5-2 hrs     | ⏳     | Valid JSON ✅                  |
| 6. Watch Mode     | 2-3 hrs       | ⏳     | Debouncing works ✅            |
| 7. Parallel       | 1-1.5 hrs     | ⏳     | Outputs match ✅               |
| 8. Tests          | 2-3 hrs       | ⏳     | cargo test --all ✅            |
| **TOTAL**         | **12-18 hrs** |        | **All green ✅**               |

**Ready to start! Begin with Meta-Task 1.**
