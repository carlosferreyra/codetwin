use super::trait_def::Driver;
use crate::formatters::folder_markdown::generate_file_md;
/// Markdown generator - Blueprint → file-level .rs.md with class diagram
use crate::ir::Blueprint;
use std::slice;

pub struct MarkdownDriver;

impl Driver for MarkdownDriver {
    fn parse(&self, _content: &str) -> Result<Blueprint, String> {
        Err("MarkdownDriver::parse: Not implemented yet (Markdown is a target, not a source for now)".to_string())
    }

    fn generate(&self, blueprint: &Blueprint) -> Result<String, String> {
        // Wrap single blueprint in a slice
        generate_file_md(slice::from_ref(blueprint))
    }
}
