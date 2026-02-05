pub mod folder_markdown;
pub mod one_per_file;
pub mod trait_def;
pub mod dependency_graph;

pub use trait_def::Layout;

use crate::config::Config;
use folder_markdown::FolderMarkdownLayout;
use one_per_file::OnePerFileLayout;
use dependency_graph::DependencyGraphLayout;

pub fn get_layout(config: &Config) -> Result<Box<dyn Layout>, String> {
    match config.layout.as_str() {
        "dependency-graph" => Ok(Box::new(DependencyGraphLayout)),
        "folder_markdown" | "markdown" => Ok(Box::new(
            FolderMarkdownLayout::new("architecture.md".to_string()),
        )),
        "one_per_file" => Ok(Box::new(OnePerFileLayout::new())),
        "layered" | "readme-embedded" => Ok(Box::new(FolderMarkdownLayout::new(
            "architecture.md".to_string(),
        ))),
        other => Err(format!("Unknown layout '{}'.", other)),
    }
}
