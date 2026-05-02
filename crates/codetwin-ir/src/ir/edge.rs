//! Dependency edges — first-class in the IR (NEW_ROADMAP Phase 1.a).

use serde::{Deserialize, Serialize};

use super::ModuleId;

/// Why two modules are connected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum EdgeKind {
    /// `import` / `use` / `require` — static dependency.
    Import,
    /// One module instantiates a type defined in another.
    Uses,
    /// One module implements a trait/interface declared in another.
    Implements,
    /// Inheritance / extension.
    Extends,
    /// Dynamic call detected via call-graph analysis (future work).
    Calls,
}

/// A directed edge between two modules in the [`crate::ir::CodeModel`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Edge {
    /// Source of the edge.
    pub from: ModuleId,
    /// Target of the edge.
    pub to: ModuleId,
    /// Nature of the relationship.
    pub kind: EdgeKind,
}
