//! Structural diff between two [`CodeModel`]s.

use serde::{Deserialize, Serialize};

use crate::ir::{CodeModel, ModuleId};

/// A single structural change between snapshots.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Change {
    /// A module that exists only in the `after` snapshot.
    ModuleAdded(ModuleId),
    /// A module that exists only in the `before` snapshot.
    ModuleRemoved(ModuleId),
    /// A module that exists in both but whose contents differ.
    ModuleChanged {
        /// Module identifier.
        id: ModuleId,
        /// Short human-readable description of what changed.
        summary: String,
    },
}

/// Aggregated result of a diff.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DiffReport {
    /// Ordered list of changes.
    pub changes: Vec<Change>,
}

/// Compute the structural diff between `before` and `after`.
///
/// TODO(Phase 4.b): implement full structural diffing (renames, moved
///                  symbols, public-API surface changes), ignoring cosmetic
///                  differences like formatting and comments.
pub fn diff(before: &CodeModel, after: &CodeModel) -> DiffReport {
    let _ = (before, after);
    DiffReport::default()
}
