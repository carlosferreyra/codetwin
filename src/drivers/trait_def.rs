/// The Driver trait definition
use crate::core::ir::Blueprint;
use anyhow::Result;

// Defined in terminology.rs
pub use crate::drivers::LanguageTerminology;

pub trait Driver {
    fn parse(&self, content: &str) -> Result<Blueprint>;
    fn generate(&self, blueprint: &Blueprint) -> Result<String>;

    /// Provide language-specific terminology for layouts
    /// Default: uses generic terminology (works for all languages)
    /// Drivers override for language-accurate output
    fn terminology(&self) -> LanguageTerminology {
        LanguageTerminology::generic()
    }
}
