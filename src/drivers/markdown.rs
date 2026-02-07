use super::trait_def::Driver;
/// Markdown generator - Blueprint → file-level .rs.md with class diagram
use crate::core::ir::Blueprint;
use crate::layouts::folder_markdown::generate_file_md;
use anyhow::{Result, anyhow};
use std::slice;

pub struct MarkdownDriver;

impl Driver for MarkdownDriver {
    fn parse(&self, _content: &str) -> Result<Blueprint> {
        Err(anyhow!(
            "MarkdownDriver::parse: Not implemented yet (Markdown is a target, not a source for now)"
        ))
    }

    fn generate(&self, blueprint: &Blueprint) -> Result<String> {
        // Wrap single blueprint in a slice
        generate_file_md(slice::from_ref(blueprint))
    }
}
