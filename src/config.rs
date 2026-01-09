/// Configuration management - reads/writes codetwin.toml
use std::fs;
use std::path::Path;
use toml::Table;

#[derive(Debug, Clone)]
pub struct Config {
    pub output_dir: String,
    pub source_dirs: Vec<String>,
    pub main_diagram: String,
    pub watch_pattern: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            output_dir: "docs".to_string(),
            source_dirs: vec!["src".to_string()],
            main_diagram: "STRUCT.md".to_string(),
            watch_pattern: "**/*.rs".to_string(),
        }
    }
}

impl Config {
    /// Generate TOML string representation
    pub fn to_toml(&self) -> String {
        let source_dirs = self
            .source_dirs
            .iter()
            .map(|d| format!("\"{}\"", d))
            .collect::<Vec<_>>()
            .join(", ");

        format!(
            r#"[codetwin]
# Output directory for generated documentation
output_dir = "{}"

# Source directories to scan
source_dirs = [{}]

# Main diagram filename
main_diagram = "{}"

# Watch pattern for file monitoring
watch_pattern = "{}"
"#,
            self.output_dir, source_dirs, self.main_diagram, self.watch_pattern
        )
    }

    /// Write config to codetwin.toml
    pub fn save(&self) -> Result<(), String> {
        let path = Path::new("codetwin.toml");

        if path.exists() {
            return Err(
                "codetwin.toml already exists. Remove it first if you want to reinitialize."
                    .to_string(),
            );
        }

        let content = self.to_toml();
        fs::write(path, content).map_err(|e| format!("Failed to write codetwin.toml: {}", e))?;

        Ok(())
    }

    /// Load config from codetwin.toml
    pub fn load(path: &str) -> Result<Self, String> {
        let content =
            fs::read_to_string(path).map_err(|e| format!("Failed to read {}: {}", path, e))?;

        let table: Table = content
            .parse()
            .map_err(|e| format!("Failed to parse {}: {}", path, e))?;

        let codetwin = table
            .get("codetwin")
            .and_then(|v| v.as_table())
            .ok_or_else(|| "Missing [codetwin] section in config".to_string())?;

        let output_dir = codetwin
            .get("output_dir")
            .and_then(|v| v.as_str())
            .unwrap_or("docs")
            .to_string();

        let source_dirs = codetwin
            .get("source_dirs")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_else(|| vec!["src".to_string()]);

        let main_diagram = codetwin
            .get("main_diagram")
            .and_then(|v| v.as_str())
            .unwrap_or("STRUCT.md")
            .to_string();

        let watch_pattern = codetwin
            .get("watch_pattern")
            .and_then(|v| v.as_str())
            .unwrap_or("**/*.rs")
            .to_string();

        Ok(Config {
            output_dir,
            source_dirs,
            main_diagram,
            watch_pattern,
        })
    }
}

/// Generate template STRUCT.md content
pub fn struct_md_template() -> String {
    r#"# Project Architecture

> Auto-generated documentation. Run `codetwin sync` to update.

## Module Dependencies

```mermaid
graph TD
    main[main.rs]
    cli[cli.rs]
    engine[engine.rs]
    ir[ir.rs]
    drivers[drivers/]

    main --> cli
    cli --> engine
    engine --> drivers
    engine --> ir
```

---

## Files

_Documentation will appear here after running `codetwin sync`_
"#
    .to_string()
}
