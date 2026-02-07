use super::trait_def::Driver;
/// Tree-sitter logic for TypeScript
use crate::core::ir::Blueprint;
use crate::drivers::LanguageTerminology;
use anyhow::{Result, anyhow};

pub struct TypeScriptDriver;

impl Driver for TypeScriptDriver {
    fn parse(&self, _content: &str) -> Result<Blueprint> {
        Err(anyhow!("TypeScriptDriver::parse: Not implemented yet"))
    }

    fn generate(&self, _blueprint: &Blueprint) -> Result<String> {
        Err(anyhow!("TypeScriptDriver::generate: Not implemented yet"))
    }

    /// TypeScript/JavaScript-specific terminology (future Phase 3)
    fn terminology(&self) -> LanguageTerminology {
        LanguageTerminology {
            element_type_singular: "class".to_string(),
            element_type_plural: "classes".to_string(),
            function_label: "method".to_string(),
            function_label_plural: "methods".to_string(),
            return_type_default: "void".to_string(),
            property_label: "property".to_string(),
            property_label_plural: "properties".to_string(),
        }
    }
}
