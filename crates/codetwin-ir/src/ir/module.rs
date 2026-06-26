//! Module-level IR node.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::Symbol;

/// Stable identifier for a module within a [`CodeModel`](super::CodeModel).
///
/// Using a dedicated newtype (rather than a bare `String`) lets us swap the
/// representation later without touching every consumer.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ModuleId(pub String);

impl From<&str> for ModuleId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for ModuleId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

/// A single source file or logical namespace.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Module {
    /// Stable identifier (usually a dotted path — `"crate::cli::gen"`).
    pub id: ModuleId,
    /// Display name shown in documentation output.
    pub name: String,
    /// Source path (relative to project root) this module was produced from.
    pub path: PathBuf,
    /// Symbols declared in this module.
    pub symbols: Vec<Symbol>,
    /// Doc comment text extracted from the file header, if any.
    pub doc: Option<String>,
}
