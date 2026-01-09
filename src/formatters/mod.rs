pub mod folder_markdown;
pub mod one_per_file;
pub mod trait_def;

pub use trait_def::Formatter;

use crate::config::Config;
use folder_markdown::FolderMarkdownFormatter;
use one_per_file::OnePerFileFormatter;

pub fn get_formatter(config: &Config) -> Result<Box<dyn Formatter>, String> {
    match config.formatter.as_str() {
        "folder_markdown" | "markdown" => Ok(Box::new(FolderMarkdownFormatter::new(
            config.main_diagram.clone(),
        ))),
        "one_per_file" => Ok(Box::new(OnePerFileFormatter::new())),
        other => Err(format!("Unknown formatter '{}'.", other)),
    }
}
