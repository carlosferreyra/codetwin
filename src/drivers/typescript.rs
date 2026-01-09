/// Tree-sitter logic for TypeScript

use crate::ir::Blueprint;
use super::trait_def::Driver;

pub struct TypeScriptDriver;

impl Driver for TypeScriptDriver {
    fn parse(&self, _content: &str) -> Result<Blueprint, String> {
        Err("TypeScriptDriver::parse: Not implemented yet".to_string())
    }

    fn generate(&self, _blueprint: &Blueprint) -> Result<String, String> {
        Err("TypeScriptDriver::generate: Not implemented yet".to_string())
    }
}
