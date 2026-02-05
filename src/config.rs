/// Configuration management - reads/writes codetwin.toml
use std::fs;
use std::path::Path;
use toml::Table;

#[derive(Debug, Clone)]
pub struct Config {
    /// Source directories to scan for code
    pub source_dirs: Vec<String>,
    /// Output file for generated documentation (e.g., docs/architecture.md)
    pub output_file: String,
    /// Layout type: dependency-graph, layered, readme-embedded
    pub layout: String,
    /// Patterns to exclude from scanning
    pub exclude_patterns: Vec<String>,
    /// Optional layer configuration for layered layout
    pub layers: Vec<Layer>,
}

#[derive(Debug, Clone)]
pub struct Layer {
    pub name: String,
    pub patterns: Vec<String>,
}

impl Config {
    /// Returns hardcoded defaults (shared between auto-gen and init)
    pub fn defaults() -> Self {
        Config {
            source_dirs: vec!["src".to_string()],
            output_file: "docs/architecture.md".to_string(),
            layout: "dependency-graph".to_string(),
            exclude_patterns: vec![
                "**/target/**".to_string(),
                "**/node_modules/**".to_string(),
                "**/.git/**".to_string(),
                "**/tests/**".to_string(),
            ],
            layers: Vec::new(),
        }
    }

    /// Generate TOML string representation
    pub fn to_toml(&self) -> String {
        let source_dirs = self
            .source_dirs
            .iter()
            .map(|d| format!("\"{}\"", d))
            .collect::<Vec<_>>()
            .join(", ");

        let exclude_patterns = self
            .exclude_patterns
            .iter()
            .map(|p| format!("\"{}\"", p))
            .collect::<Vec<_>>()
            .join(", ");

        let mut toml = format!(
            r#"# CodeTwin Configuration
# Code → Diagram/Documentation Generator
# https://github.com/carlosferreyra/codetwin

# Source directories to scan
source_dirs = [{}]

# Output file for generated documentation
output_file = "{}"

# Layout: dependency-graph, layered, readme-embedded
layout = "{}"

# Patterns to exclude from scanning
exclude_patterns = [{}]
"#,
            source_dirs, self.output_file, self.layout, exclude_patterns
        );

        // Add layer configuration if present
        if !self.layers.is_empty() {
            toml.push_str("\n# Layer configuration (for layered layout)\n");
            for layer in &self.layers {
                let patterns = layer
                    .patterns
                    .iter()
                    .map(|p| format!("\"{}\"", p))
                    .collect::<Vec<_>>()
                    .join(", ");

                toml.push_str(&format!(
                    "\n[[layers]]\nname = \"{}\"\npatterns = [{}]\n",
                    layer.name, patterns
                ));
            }
        }

        toml
    }

    /// Write config to codetwin.toml (idempotent like uv init)
    pub fn save(&self, force: bool) -> Result<(), String> {
        let path = Path::new("codetwin.toml");

        if path.exists() && !force {
            return Err("codetwin.toml already initialized. Use --force to overwrite.".to_string());
        }

        let content = self.to_toml();
        fs::write(path, content).map_err(|e| format!("Failed to write codetwin.toml: {}", e))?;

        Ok(())
    }

    /// Load config from codetwin.toml, or return defaults if missing
    pub fn load_or_defaults(path: &str) -> Self {
        match Self::load(path) {
            Ok(config) => config,
            Err(_) => Self::defaults(),
        }
    }

    /// Load config from codetwin.toml
    pub fn load(path: &str) -> Result<Self, String> {
        let content =
            fs::read_to_string(path).map_err(|e| format!("Failed to read {}: {}", path, e))?;

        let table: Table = content
            .parse()
            .map_err(|e| format!("Failed to parse {}: {}", path, e))?;

        let source_dirs = table
            .get("source_dirs")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_else(|| vec!["src".to_string()]);

        let output_file = table
            .get("output_file")
            .and_then(|v| v.as_str())
            .unwrap_or("docs/architecture.md")
            .to_string();

        let layout = table
            .get("layout")
            .and_then(|v| v.as_str())
            .unwrap_or("dependency-graph")
            .to_string();

        let exclude_patterns = table
            .get("exclude_patterns")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_else(|| {
                vec![
                    "**/target/**".to_string(),
                    "**/node_modules/**".to_string(),
                    "**/.git/**".to_string(),
                    "**/tests/**".to_string(),
                ]
            });

        // Parse layers if present
        let layers = table
            .get("layers")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|item| item.as_table())
                    .map(|layer_table| {
                        let name = layer_table
                            .get("name")
                            .and_then(|v| v.as_str())
                            .unwrap_or("Unknown")
                            .to_string();
                        let patterns = layer_table
                            .get("patterns")
                            .and_then(|v| v.as_array())
                            .map(|arr| {
                                arr.iter()
                                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                    .collect()
                            })
                            .unwrap_or_default();
                        Layer { name, patterns }
                    })
                    .collect()
            })
            .unwrap_or_default();

        Ok(Config {
            source_dirs,
            output_file,
            layout,
            exclude_patterns,
            layers,
        })
    }
}
