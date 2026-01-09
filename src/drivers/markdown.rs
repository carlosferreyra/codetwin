/// Tree-sitter logic for Docs (Reading/Writing blocks)

use crate::ir::Blueprint;
use super::trait_def::Driver;

pub struct MarkdownDriver;

impl Driver for MarkdownDriver {
    fn parse(&self, _content: &str) -> Result<Blueprint, String> {
        Err("MarkdownDriver::parse: Not implemented yet".to_string())
    }

    fn generate(&self, _blueprint: &Blueprint) -> Result<String, String> {
        Err("MarkdownDriver::generate: Not implemented yet".to_string())
    }
}
