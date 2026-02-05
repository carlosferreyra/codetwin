use super::trait_def::Driver;
use crate::drivers::LanguageTerminology;
/// Tree-sitter logic for Python
use crate::ir::Blueprint;
use anyhow::{anyhow, Result};

pub struct PythonDriver;

impl Driver for PythonDriver {
    fn parse(&self, _content: &str) -> Result<Blueprint> {
        Err(anyhow!("PythonDriver::parse: Not implemented yet"))
    }

    fn generate(&self, _blueprint: &Blueprint) -> Result<String> {
        Err(anyhow!("PythonDriver::generate: Not implemented yet"))
    }

    /// Python-specific terminology (future Phase 3)
    fn terminology(&self) -> LanguageTerminology {
        LanguageTerminology::python()
    }
}
