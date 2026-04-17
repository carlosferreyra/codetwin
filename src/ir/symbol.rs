//! Symbol-level IR node.

use serde::{Deserialize, Serialize};

use super::Visibility;

/// What kind of construct a [`Symbol`] represents.
///
/// New variants are additive — old consumers should still compile against
/// a newer IR thanks to `#[non_exhaustive]`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum SymbolKind {
    /// Free function.
    Function,
    /// Struct / class / dataclass.
    Struct,
    /// Enum / sum type.
    Enum,
    /// Trait / interface / protocol.
    Trait,
    /// Constant / static.
    Constant,
    /// Type alias.
    TypeAlias,
    /// Module declaration (for languages that declare submodules inline).
    Module,
}

/// A top-level symbol declared inside a module.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Symbol {
    /// Local name (`fn foo` → `"foo"`).
    pub name: String,
    /// Kind (function, struct, ...).
    pub kind: SymbolKind,
    /// Visibility (public / private / ...).
    pub visibility: Visibility,
    /// Source line (1-indexed) — handy for cross-linking to IDEs/GitHub.
    pub line: u32,
    /// Doc comment, if any.
    pub doc: Option<String>,
    /// Textual signature for display (e.g. `"fn foo(x: u32) -> bool"`).
    ///
    /// TODO(Phase 1.a): decide whether to also store a structured signature
    ///                  (parameters + return type) for richer layouts.
    pub signature: Option<String>,
}
