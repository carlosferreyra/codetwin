use super::trait_def::Driver;
/// Tree-sitter logic for Python
use crate::ir::Blueprint;

pub struct PythonDriver;

impl Driver for PythonDriver {
    fn parse(&self, _content: &str) -> Result<Blueprint, String> {
        Err("PythonDriver::parse: Not implemented yet".to_string())
    }

    fn generate(&self, _blueprint: &Blueprint) -> Result<String, String> {
        Err("PythonDriver::generate: Not implemented yet".to_string())
    }
}
