# Agent Instructions: Phase 2.5 (Language-Agnostic Refactoring)

**Time Estimate**: 12-16 hours **Status**: 🚀 READY TO START **Current Project State**: Phase 1 ✅ +
Phase 1.5 ✅ + Phase 2 (All Layouts) ✅ + Phase 2.5 🚀

---

## Quick Start

1. Read this entire file (5 min)
2. Review the **Hardcoded Text Audit** section below to understand what needs fixing
3. Start **Meta-Task 1: Remove Hardcoded Paths** below
4. After each meta-task, run the **Validation Checks**
5. Move to next meta-task only after validation passes
6. Use ROADMAP.md for Phase 2.5 specifications

**Critical**: Do NOT skip validation checks. Each one confirms success before proceeding.

---

## What is Phase 2.5 (Language-Agnostic Refactoring)?

Refactor all three existing layouts to work with ANY programming language and ANY project structure:

- **Remove hardcoded paths** (e.g., folder_markdown.rs has codetwin-specific nodes)
- **Eliminate project-specific assumptions** (e.g., layered.rs default layers assume Rust paths like
  `src/drivers/**`)
- **Use generic terminology** ("types" instead of "structs", "returns" instead of "void", "items"
  instead of "functions")
- **Make layouts dynamic** (remove hardcoded module purpose mappings in readme_embedded.rs)
- **Enable custom layouts** (CLI flag + IR-compliant file so developers can add their own layout
  templates)

**Why now?** Phase 2 (all 3 layouts) is complete but contains hardcoded text specific to
codetwin/Rust. Before Phase 3 (multi-language support), layouts must be truly language-agnostic.
This unblocks Python, TypeScript, and other language drivers from inheriting Rust-specific
assumptions.

**Goal**: Make CodeTwin a universal code documentation tool that works for Java, Python, TypeScript,
Go, or any other language without modification.

---

## Meta-Task Checklist

- [ ] **Meta-Task 1**: Remove Hardcoded Paths from Layouts (3-4 hrs)
- [ ] **Meta-Task 2**: Implement Generic Terminology System (3-4 hrs)
- [ ] **Meta-Task 3**: Make Layered Defaults Configurable (2-3 hrs)
- [ ] **Meta-Task 4**: Add Custom Layout Support via CLI (3-4 hrs)
- [ ] **Meta-Task 5**: Integration Tests & Validation (2-3 hrs)

**Total: 12-16 hours**

---

## Hardcoded Text Audit

Before starting, understand what needs fixing:

### 🔴 Critical Issues

1. **folder_markdown.rs (L94-104)**: `generate_index_md()` has hardcoded codetwin-specific diagram:
   - Nodes: `main[main.rs]`, `cli[cli.rs]`, `engine[engine.rs]`, `ir[ir.rs]`, `drivers[drivers/]`
   - **Fix**: Make diagram dynamic based on actual discovered modules

2. **layered.rs (L19-42)**: Default layers assume Rust/codetwin structure:
   - Patterns: `src/lib.rs`, `src/engine.rs`, `src/drivers/**`, `src/io/**`
   - **Fix**: Remove defaults OR make them example templates in config only

3. **readme_embedded.rs (L68-88)**: `infer_purpose()` has hardcoded module name→purpose mapping:
   - `"cli"` → "Command-line interface", `"engine"` → "Core generation engine"
   - **Fix**: Remove hardcoded mappings, use docstrings or generic descriptions

4. **readme_embedded.rs (L299-310)**: Development guide assumes Rust `src/` structure:
   - `"1. Create src/drivers/my_language.rs"`, `"1. Create src/layouts/my_layout.rs"`
   - **Fix**: Remove or make guide dynamic based on project structure

### 🟡 Moderate Issues

1. **dependency_graph.rs (L133), layered.rs (L120)**: Uses "structs" and "functions"
   - **Fix**: Use "types" and "items" (language-neutral)

2. **dependency_graph.rs (L143-146)**: Displays `(struct)` and `(function)` labels
   - **Fix**: Use generic labels like `(type)` and `(callable)`

3. **folder_markdown.rs (L167)**: Hardcoded `"void"` as default return type
   - **Fix**: Use generic term like `"none"` or `"—"`

---

## Meta-Task 1: Remove Hardcoded Paths from Layouts

**Goal**: Make all layouts generate dynamic content based on actual discovered code, not hardcoded
assumptions

**Files to modify**: `src/layouts/folder_markdown.rs`, `src/layouts/readme_embedded.rs`,
`src/layouts/layered.rs`

### Subtasks

1. **Fix `folder_markdown.rs` - Remove hardcoded index diagram**:
   - Replace `generate_index_md()` function (lines 94-121)
   - Current issue: Hardcoded nodes like `main[main.rs]`, `cli[cli.rs]`, `engine[engine.rs]`
   - **Solution**: Generate diagram dynamically from blueprints:

     ````rust
     fn generate_index_md(blueprints: &[Blueprint]) -> Result<String> {
         let mut output = String::new();
         output.push_str("# Project Architecture\n\n");
         output.push_str("## Module Dependencies\n\n");
         output.push_str("```mermaid\ngraph TD\n");

         // Add nodes dynamically from blueprints
         for blueprint in blueprints {
             let module_name = extract_module_name(&blueprint.source_path);
             output.push_str(&format!("    {}[{}]\n",
                 sanitize_id(&module_name), module_name));
         }

         // Add edges from dependencies
         for blueprint in blueprints {
             let from = extract_module_name(&blueprint.source_path);
             for dep in &blueprint.dependencies {
                 output.push_str(&format!("    {} --> {}\n",
                     sanitize_id(&from), sanitize_id(dep)));
             }
         }

         output.push_str("```\n\n");
         // ... rest of function
     }
     ````

   - Update function signature: `generate_index_md(blueprints: &[Blueprint])` instead of
     `modules: &[&str]`
   - Update caller in `format()` method to pass `blueprints` instead of `modules`

2. **Fix `readme_embedded.rs` - Remove hardcoded development guide**:
   - Remove hardcoded paths from `generate_dev_guide()` (lines 290-320)
   - Current issue: Assumes `src/drivers/`, `src/layouts/` structure
   - **Solution**: Make guide generic or remove entirely:

     ```rust
     fn generate_dev_guide(blueprints: &[Blueprint]) -> String {
         let mut guide = String::new();
         guide.push_str("### Development Guide\n\n");
         guide.push_str("#### Key Files\n\n");

         // List actual discovered files (no hardcoded paths)
         for blueprint in blueprints.iter().take(5) {
             guide.push_str(&format!("- `{}`\n",
                 blueprint.source_path.display()));
         }

         guide.push_str("\n#### General Guidelines\n\n");
         guide.push_str("- Follow the existing code style\n");
         guide.push_str("- Add tests for new functionality\n");
         guide.push_str("- Document public APIs\n\n");

         guide
     }
     ```

   - Remove all references to `src/drivers/my_language.rs` and `src/layouts/my_layout.rs`

3. **Fix `layered.rs` - Make defaults optional**:
   - Current issue: `LayeredLayout::defaults()` returns hardcoded Rust/codetwin layers
   - **Solution**: Make defaults() return EMPTY list, move examples to config docs only:

     ```rust
     pub fn defaults() -> Vec<Layer> {
         vec![]  // Start with no layers - let user configure or auto-detect
     }
     ```

   - Add auto-detection logic when layers are empty:

     ```rust
     impl Layout for LayeredLayout {
         fn format(&self, blueprints: &[Blueprint]) -> Result<Vec<(String, String)>> {
             let layers = if self.layers.is_empty() {
                 // Auto-detect by directory structure
                 auto_detect_layers(blueprints)
             } else {
                 self.layers.clone()
             };
             // ... rest of format
         }
     }

     fn auto_detect_layers(blueprints: &[Blueprint]) -> Vec<Layer> {
         // Group by parent directory
         let mut layers = HashMap::new();
         for bp in blueprints {
             let dir = bp.source_path.parent()
                 .and_then(|p| p.file_name())
                 .and_then(|n| n.to_str())
                 .unwrap_or("root");
             layers.entry(dir).or_insert_with(Vec::new).push(bp.clone());
         }

         // Convert to Layer structs
         layers.into_iter().map(|(name, _)| Layer {
             name: name.to_string(),
             patterns: vec![format!("{}/**", name)],
         }).collect()
     }
     ```

### Validation Checks ✅

```bash
# Build & compilation:
cargo check                          # No errors

# Verify no hardcoded paths remain:
grep -n "main\[main.rs\]" src/layouts/folder_markdown.rs  # Should be 0 matches
grep -n "src/drivers/my_language" src/layouts/readme_embedded.rs  # Should be 0 matches
grep -n "src/lib.rs" src/layouts/layered.rs  # Should be 0 matches in defaults()

# Test dynamic generation:
cargo run -- gen --layout folder-markdown
# Verify generated diagram has actual project modules, not hardcoded ones

cargo run -- gen --layout readme-embedded
# Verify dev guide doesn't mention "src/drivers/" or "src/layouts/"

cargo run -- gen --layout layered
# Verify works even with empty layer config (auto-detection)
```

**✅ PASS**: No hardcoded paths found, layouts generate dynamic content **❌ FAIL**: Compilation
errors or hardcoded text found → fix before proceeding

---

## Meta-Task 2: Implement Language-Aware Terminology System

**Goal**: Create a future-proof terminology system that centralizes generic terms but allows each
language driver to override with accurate language-specific terminology

**Architecture**: Centralized terminology structure + per-driver overrides = works for OOP languages
now, scales to non-OOP paradigms later

**Files to create/modify**: `src/drivers/terminology.rs` (NEW), `src/layouts/dependency_graph.rs`,
`src/layouts/layered.rs`, `src/layouts/folder_markdown.rs`, `src/drivers/mod.rs`,
`src/drivers/trait_def.rs`

### Subtasks

1. **Create centralized terminology module** `src/drivers/terminology.rs`:
   - Define struct with generic defaults:

     ```rust
     /// Language-agnostic terminology for code elements
     /// Drivers override these for language-specific accuracy
     #[derive(Debug, Clone)]
     pub struct LanguageTerminology {
         /// Singular: "type", "struct", "class", "data type"
         pub element_type_singular: String,
         /// Plural: "types", "structs", "classes"
         pub element_type_plural: String,

         /// "item", "function", "def", "fn", "func"
         pub function_label: String,
         /// "items", "functions", "defs", "fns", "funcs"
         pub function_label_plural: String,

         /// Default return type: "—", "void", "None", "()"
         pub return_type_default: String,

         /// "property", "field", "attribute", "member"
         pub property_label: String,
         /// "properties", "fields", "attributes", "members"
         pub property_label_plural: String,
     }

     impl LanguageTerminology {
         /// Generic defaults - works for all languages
         pub fn generic() -> Self {
             LanguageTerminology {
                 element_type_singular: "type".to_string(),
                 element_type_plural: "types".to_string(),
                 function_label: "item".to_string(),
                 function_label_plural: "items".to_string(),
                 return_type_default: "—".to_string(),
                 property_label: "field".to_string(),
                 property_label_plural: "fields".to_string(),
             }
         }
     }
     ```

2. **Add trait to driver trait definition** `src/drivers/trait_def.rs`:
   - Add method to `Driver` trait:

     ```rust
     pub trait Driver: Send + Sync {
         fn parse(&self, source: &str, language: &str) -> Result<Blueprint>;

         /// NEW: Provide language-specific terminology
         fn terminology(&self) -> LanguageTerminology {
             LanguageTerminology::generic()  // Default: use centralized generics
         }
     }
     ```

3. **Implement per-driver terminology** `src/drivers/rust.rs`:
   - Override in RustDriver:

     ```rust
     impl Driver for RustDriver {
         fn parse(&self, source: &str, _language: &str) -> Result<Blueprint> {
             // ... existing code
         }

         fn terminology(&self) -> LanguageTerminology {
             LanguageTerminology {
                 element_type_singular: "struct".to_string(),
                 element_type_plural: "structs".to_string(),
                 function_label: "fn".to_string(),
                 function_label_plural: "fns".to_string(),
                 return_type_default: "()".to_string(),
                 property_label: "field".to_string(),
                 property_label_plural: "fields".to_string(),
             }
         }
     }
     ```

4. **Document override pattern for future drivers** in comments:
   - Python example (future Phase 3):

     ```rust
     /// Example for Python driver (Phase 3)
     fn terminology(&self) -> LanguageTerminology {
         LanguageTerminology {
             element_type_singular: "class".to_string(),
             element_type_plural: "classes".to_string(),
             function_label: "def".to_string(),
             function_label_plural: "defs".to_string(),
             return_type_default: "None".to_string(),
             property_label: "attribute".to_string(),
             property_label_plural: "attributes".to_string(),
         }
     }

     /// Example for Go driver (future)
     fn terminology(&self) -> LanguageTerminology {
         LanguageTerminology {
             element_type_singular: "type".to_string(),
             element_type_plural: "types".to_string(),
             function_label: "func".to_string(),
             function_label_plural: "funcs".to_string(),
             return_type_default: "error".to_string(),
             property_label: "field".to_string(),
             property_label_plural: "fields".to_string(),
         }
     }

     /// Example for non-OOP (Haskell - future)
     fn terminology(&self) -> LanguageTerminology {
         LanguageTerminology {
             element_type_singular: "data type".to_string(),
             element_type_plural: "data types".to_string(),
             function_label: "function".to_string(),
             function_label_plural: "functions".to_string(),
             return_type_default: "IO ()".to_string(),
             property_label: "constructor".to_string(),
             property_label_plural: "constructors".to_string(),
         }
     }
     ```

5. **Update layouts to use terminology from IR context**:
   - Layouts don't hardcode terminology
   - Instead, they receive terminology from active driver:

     ```rust
     // In dependency_graph.rs
     pub fn format(&self, blueprints: &[Blueprint], terminology: &LanguageTerminology) -> Result<Vec<(String, String)>> {
         // Use: terminology.element_type_plural, terminology.function_label_plural
         format!(
             "**Contents**: {} {}, {} {}\n\n",
             type_count,
             terminology.element_type_plural,
             function_count,
             terminology.function_label_plural
         )
     }
     ```

6. **Update `src/engine.rs` to pass terminology**:
   - Get terminology from active driver and pass to layout:

     ```rust
     pub fn generate(&self, config: &Config) -> Result<()> {
         let blueprints = self.discover_and_parse()?;
         let driver = self.get_active_driver();
         let terminology = driver.terminology();

         let layout = layouts::get_layout(&config.layout)?;
         let output = layout.format(&blueprints, &terminology)?;
         // ... write output
     }
     ```

### Validation Checks ✅

```bash
# Verify terminology structure exists:
grep -n "struct LanguageTerminology" src/drivers/terminology.rs

# Verify Rust driver overrides terminology:
grep -A 10 "fn terminology" src/drivers/rust.rs | grep "struct"

# Test that generic terminology is used by default:
cargo run -- gen --layout dependency-graph
# Verify output shows "types" and "items" (generic defaults)

# Future-proof validation:
# Verify pattern is documented for new drivers (read comments):
grep -B 2 "Example for Python" src/drivers/terminology.rs

# Build without errors:
cargo check
```

**✅ PASS**: Terminology system works, drives show language-appropriate terms when driver added,
generic defaults for now **❌ FAIL**: Terminology doesn't override or layouts can't access it → fix
before proceeding

### Future-Proofness

This design ensures:

- ✅ **New OOP languages** (Java, C#, Go): Add driver → override terminology fields → same layout
  code works
- ✅ **Non-OOP languages** (Haskell, Lisp): Add driver → override with different primitives → scales
  automatically
- ✅ **No layout changes needed**: Layouts ask driver for terminology, not hardcoded
- ✅ **Per-driver granularity**: Each language shows native terminology
- ✅ **Backwards compatible**: Current generic terminology works until drivers are added

**Example outputs when drivers added**:

```markdown
# Phase 2.5 (Rust only):

**Contents**: 5 types, 8 items

# Phase 3+ (Multi-language):

Rust output: **Contents**: 5 structs, 8 fns Python output: **Contents**: 5 classes, 8 defs Go
output: **Contents**: 5 types, 8 funcs Haskell output: **Contents**: 5 data types, 8 functions
```

All from same layout code + same IR, different terminology per driver. ✨

---

## Meta-Task 3: Make Layered Defaults Configurable

**Goal**: Remove hardcoded codetwin-specific layer defaults, make user-configurable or auto-detect

**Files to modify**: `src/layouts/layered.rs`, `src/config.rs`, `codetwin.toml`

### Subtasks

1. **Update `config.rs` to support optional layers**:
   - Modify `Config` struct to have optional layers:

     ```rust
     #[derive(Debug, Clone, Serialize, Deserialize)]
     pub struct Config {
         // ... existing fields ...

         #[serde(default)]
         pub layers: Vec<Layer>,  // Empty by default
     }

     #[derive(Debug, Clone, Serialize, Deserialize)]
     pub struct Layer {
         pub name: String,
         pub patterns: Vec<String>,
     }
     ```

2. **Remove hardcoded defaults from `layered.rs`**:
   - Change `LayeredLayout::defaults()` to return empty:

     ```rust
     pub fn defaults() -> Vec<Layer> {
         vec![]
     }
     ```

   - Document example layers in comments only (not as defaults):

     ````rust
     /// Example layer configuration (add to codetwin.toml):
     /// ```toml
     /// [[layers]]
     /// name = "Core"
     /// patterns = ["src/lib.rs", "src/main.rs"]
     ///
     /// [[layers]]
     /// name = "Services"
     /// patterns = ["src/services/**"]
     /// ```
     ````

3. **Implement auto-detection when no layers configured**:
   - Already implemented in Meta-Task 1 (auto_detect_layers function)
   - Ensure it groups by directory structure dynamically

4. **Update `codetwin.toml` example**:
   - Remove hardcoded layer defaults
   - Add commented example:

     ```toml
     # Optional: Define custom layers (auto-detected if omitted)
     # [[layers]]
     # name = "Core"
     # patterns = ["src/lib.rs", "src/main.rs"]
     ```

### Validation Checks ✅

```bash
# Verify defaults are empty:
grep -A 20 "pub fn defaults()" src/layouts/layered.rs
# Should return vec![] not hardcoded layers

# Test with no layer config:
# Remove layers section from codetwin.toml
cargo run -- gen --layout layered
# Should auto-detect layers from directory structure

# Test with custom layers:
# Add custom layers to codetwin.toml
cargo run -- gen --layout layered
# Should use configured layers
```

**✅ PASS**: LayeredLayout works with empty defaults, auto-detects or uses config **❌ FAIL**: Still
has hardcoded defaults or doesn't auto-detect → fix before proceeding

---

## Meta-Task 4: Add Custom Layout Support via CLI

**Goal**: Allow developers to specify custom layout templates via CLI flag + IR-compliant
configuration file

**Files to modify**: `src/cli.rs`, `src/engine.rs`, `src/layouts/mod.rs`, `src/config.rs`

### Subtasks

1. **Add `--custom-layout` CLI flag**:
   - Update `src/cli.rs` to add new flag:

     ```rust
     #[arg(long, value_name = "FILE")]
     /// Path to custom layout configuration file (IR-compliant)
     custom_layout: Option<PathBuf>,
     ```

2. **Define custom layout file format**:
   - Create example `custom_layout.toml` format:

     ```toml
     # Custom Layout Definition
     name = "my-custom-layout"
     output_file = "docs/custom.md"

     # Template sections (referencing Blueprint IR fields)
     [template]
     header = "# {{project_name}} Architecture\n\n"

     [template.module]
     format = "## {{module.name}}\n\n**File**: {{module.source_path}}\n\n"

     [template.class]
     format = "### {{class.name}}\n\n{{class.documentation.summary}}\n\n"

     [template.function]
     format = "- `{{function.name}}({{params}})` - {{function.documentation.summary}}\n"
     ```

3. **Create custom layout loader**:
   - Add to `src/layouts/mod.rs`:

     ```rust
     pub fn load_custom_layout(path: impl AsRef<Path>) -> Result<Box<dyn Layout>> {
         let content = std::fs::read_to_string(path.as_ref())
             .context("Failed to read custom layout file")?;

         let config: CustomLayoutConfig = toml::from_str(&content)
             .context("Invalid custom layout TOML")?;

         Ok(Box::new(CustomLayout::new(config)))
     }

     #[derive(Debug, Clone, Serialize, Deserialize)]
     pub struct CustomLayoutConfig {
         pub name: String,
         pub output_file: String,
         pub template: TemplateConfig,
     }

     pub struct CustomLayout {
         config: CustomLayoutConfig,
     }

     impl Layout for CustomLayout {
         fn format(&self, blueprints: &[Blueprint]) -> Result<Vec<(String, String)>> {
             // Parse templates and apply to blueprints
             // Use simple string interpolation for {{field}} placeholders
             // ...implementation...
         }
     }
     ```

4. **Wire custom layout to CLI**:
   - Update `src/engine.rs` to handle custom layout:

     ```rust
     pub fn generate(&self, config: &Config, custom_layout: Option<PathBuf>) -> Result<()> {
         let layout: Box<dyn Layout> = if let Some(custom_path) = custom_layout {
             layouts::load_custom_layout(&custom_path)?
         } else {
             layouts::get_layout(&config.layout)?
         };

         // ... rest of generation logic ...
     }
     ```

5. **Add validation for custom layouts**:
   - Ensure template fields reference valid Blueprint IR fields
   - Add helpful error messages for invalid templates
   - Document available IR fields in comments/docs

### Validation Checks ✅

```bash
# Create example custom layout file:
cat > my_layout.toml << 'EOF'
name = "simple"
output_file = "docs/simple.md"

[template]
header = "# Project Overview\n\n"

[template.module]
format = "## {{module_name}}\n\n"
EOF

# Test custom layout flag:
cargo run -- gen --custom-layout my_layout.toml
# Should generate docs/simple.md using custom template

# Verify error handling:
cargo run -- gen --custom-layout nonexistent.toml
# Should show helpful error about missing file

# Verify invalid fields are caught:
# Create layout with invalid IR field reference
cargo run -- gen --custom-layout invalid_layout.toml
# Should show error about invalid field
```

**✅ PASS**: Custom layouts work via CLI, good error messages, validates IR fields **❌ FAIL**:
Custom layout doesn't work or poor errors → fix before proceeding

---

## Meta-Task 5: Integration Tests & Validation

**Goal**: Ensure all refactoring works correctly with comprehensive tests

**Files to create/modify**: `tests/test_phase2_5.rs`

### Subtasks

1. **Create Phase 2.5 test suite**:
   - Add `tests/test_phase2_5.rs`:

     ```rust
     #[test]
     fn test_no_hardcoded_paths_in_layouts() {
         // Test that layouts don't contain codetwin-specific paths
         let blueprints = create_generic_blueprints();

         // Test folder_markdown
         let layout = FolderMarkdownLayout::new("README.md");
         let output = layout.format(&blueprints).unwrap();
         assert!(!output[0].1.contains("main[main.rs]"));
         assert!(!output[0].1.contains("cli[cli.rs]"));

         // Test readme_embedded
         let layout = ReadmeEmbeddedLayout;
         let output = layout.format(&blueprints).unwrap();
         assert!(!output[0].1.contains("src/drivers/"));
         assert!(!output[0].1.contains("src/layouts/"));
     }

     #[test]
     fn test_generic_terminology() {
         let blueprints = create_test_blueprints();

         let layout = DependencyGraphLayout;
         let output = layout.format(&blueprints).unwrap();

         // Should use generic terms
         assert!(output[0].1.contains("types"));
         assert!(output[0].1.contains("items"));

         // Should NOT use language-specific terms
         assert!(!output[0].1.contains("structs"));
         assert!(!output[0].1.contains("functions"));
         assert!(!output[0].1.contains("void"));
     }

     #[test]
     fn test_layered_auto_detection() {
         let blueprints = create_multi_dir_blueprints();

         let layout = LayeredLayout::new(vec![]);  // Empty layers
         let output = layout.format(&blueprints).unwrap();

         // Should auto-detect layers from directory structure
         assert!(output[0].1.contains("Layer:"));
     }

     #[test]
     fn test_custom_layout_loading() {
         let temp_file = create_temp_custom_layout();

         let layout = load_custom_layout(&temp_file).unwrap();
         let output = layout.format(&create_test_blueprints()).unwrap();

         assert!(output.len() > 0);
         assert_eq!(output[0].0, "docs/simple.md");
     }
     ```

2. **Run full test suite**:

   ```bash
   cargo test test_phase2_5
   cargo test --all
   ```

3. **Manual validation checklist**:
   - [ ] Generate with each layout (dependency-graph, layered, readme-embedded)
   - [ ] Verify no hardcoded paths in output
   - [ ] Verify generic terminology used
   - [ ] Test with Python codebase (once Python driver exists)
   - [ ] Test custom layout file
   - [ ] All 31+ existing tests still pass

### Validation Checks ✅

```bash
# Run all tests:
cargo test --all

# Specific Phase 2.5 tests:
cargo test test_phase2_5

# Manual generation tests:
cargo run -- gen --layout dependency-graph
cargo run -- gen --layout layered
cargo run -- gen --layout readme-embedded

# Verify output is generic:
grep -n "main\[main.rs\]" docs/*.md  # Should be 0 matches
grep -n "structs" docs/*.md  # Should be 0 matches
grep -n "types, items" docs/*.md  # Should find these
```

**✅ PASS**: All tests pass, manual validation confirms generic output **❌ FAIL**: Tests fail or
output contains hardcoded text/terms → fix before proceeding

---

## Phase 2.5 Completion Checklist

Before marking Phase 2.5 complete, verify:

- [ ] **Meta-Task 1 Complete**: No hardcoded paths in any layout
- [ ] **Meta-Task 2 Complete**: Generic terminology used throughout
- [ ] **Meta-Task 3 Complete**: Layered defaults are empty/configurable
- [ ] **Meta-Task 4 Complete**: Custom layout support via CLI works
- [ ] **Meta-Task 5 Complete**: All tests pass (existing + new)
- [ ] **Build Status**: `cargo build --release` succeeds
- [ ] **Documentation**: README updated with Phase 2.5 features
- [ ] **Manual Testing**: Generated output verified for 3+ layouts

**Completion Criteria**:

- Zero hardcoded project-specific paths or module names
- Zero language-specific terminology in output
- Custom layout support functional
- All layouts work with any language/project structure
- Ready for Phase 3 multi-language support

---

## Next Steps

After Phase 2.5 completion:

1. **Update ROADMAP.md**: Mark Phase 2.5 as complete
2. **Prepare for Phase 3**: Python and TypeScript drivers can now be implemented
3. **Test with other languages**: Try generating docs for Python/JavaScript projects
4. **Community feedback**: Share generic layouts with early adopters

**Phase 3 Preview**: With language-agnostic layouts complete, Phase 3 will add:

- Python driver (tree-sitter-python)
- TypeScript driver (tree-sitter-typescript)
- Multi-language project support
- Language-specific driver registration
