pub mod dependency_graph;
pub mod folder_markdown;
pub mod layered;
pub mod one_per_file;
pub mod readme_embedded;
pub mod trait_def;

pub use trait_def::Layout;

use crate::config::Config;
use crate::ir::Blueprint;
use anyhow::{anyhow, Context, Result};
use dependency_graph::DependencyGraphLayout;
use folder_markdown::FolderMarkdownLayout;
use layered::LayeredLayout;
use one_per_file::OnePerFileLayout;
use readme_embedded::ReadmeEmbeddedLayout;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub fn get_layout(config: &Config) -> Result<Box<dyn Layout>> {
    match config.layout.as_str() {
        "dependency-graph" => Ok(Box::new(DependencyGraphLayout)),
        "folder_markdown" | "markdown" => Ok(Box::new(FolderMarkdownLayout::new(
            "architecture.md".to_string(),
        ))),
        "one_per_file" => Ok(Box::new(OnePerFileLayout::new())),
        "layered" => Ok(Box::new(LayeredLayout::new(config.layers.clone()))),
        "readme-embedded" => Ok(Box::new(ReadmeEmbeddedLayout)),
        other => Err(anyhow!("Unknown layout '{}'", other)),
    }
}

/// Load a custom layout from a TOML file
pub fn load_custom_layout(path: impl AsRef<Path>) -> Result<Box<dyn Layout>> {
    let content =
        std::fs::read_to_string(path.as_ref()).context("Failed to read custom layout file")?;

    let config: CustomLayoutConfig =
        toml::from_str(&content).context("Invalid custom layout TOML format")?;

    Ok(Box::new(CustomLayout::new(config)))
}

/// Custom layout configuration loaded from TOML
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomLayoutConfig {
    pub name: String,
    pub output_file: String,
    pub template: TemplateConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateConfig {
    pub header: Option<String>,
    #[serde(default)]
    pub module: Option<String>,
    #[serde(default)]
    pub class: Option<String>,
    #[serde(default)]
    pub function: Option<String>,
    pub footer: Option<String>,
}

/// Custom layout implementation using user-defined templates
pub struct CustomLayout {
    config: CustomLayoutConfig,
}

impl CustomLayout {
    pub fn new(config: CustomLayoutConfig) -> Self {
        CustomLayout { config }
    }

    /// Simple template variable substitution
    fn substitute(template: &str, vars: &[(&str, &str)]) -> String {
        let mut result = template.to_string();
        for (key, value) in vars {
            result = result.replace(&format!("{{{{{}}}}}", key), value);
        }
        result
    }
}

impl Layout for CustomLayout {
    fn format(&self, blueprints: &[Blueprint]) -> Result<Vec<(String, String)>> {
        let mut content = String::new();

        // Add header
        if let Some(header) = &self.config.template.header {
            content.push_str(&header);
            content.push('\n');
        }

        // Add modules
        for blueprint in blueprints {
            if let Some(module_template) = &self.config.template.module {
                let module_name = blueprint
                    .source_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown");

                let substituted = Self::substitute(
                    module_template,
                    &[
                        ("module.name", module_name),
                        (
                            "module.source_path",
                            &blueprint.source_path.display().to_string(),
                        ),
                        ("module.language", &blueprint.language),
                    ],
                );
                content.push_str(&substituted);
                content.push('\n');
            }
        }

        // Add footer
        if let Some(footer) = &self.config.template.footer {
            content.push_str(&footer);
            content.push('\n');
        }

        Ok(vec![(self.config.output_file.clone(), content)])
    }
}
