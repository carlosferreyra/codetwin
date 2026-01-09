use super::trait_def::Driver;
/// Tree-sitter logic for Rust
use crate::ir::{
    Blueprint, Class, Documentation, Element, Function, Method, Parameter, Property, Signature,
    Visibility,
};
use std::path::PathBuf;

pub struct RustDriver;

impl Driver for RustDriver {
    fn parse(&self, content: &str) -> Result<Blueprint, String> {
        // TODO: Use tree-sitter-rust to parse
        // For now, return a minimal blueprint
        Ok(Blueprint {
            source_path: PathBuf::from("unknown.rs"),
            language: "rust".to_string(),
            elements: vec![],
        })
    }

    fn generate(&self, _blueprint: &Blueprint) -> Result<String, String> {
        Err(
            "RustDriver::generate: Not implemented yet (Rust is a source, not a target)"
                .to_string(),
        )
    }
}
