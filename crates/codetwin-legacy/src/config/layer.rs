//! Layer spec for the architecture-map layout.

use serde::{Deserialize, Serialize};

/// A named group of source files.
///
/// Layers are used by the architecture-map layout to bucket modules into
/// layers (UI / Engine / Drivers / ...). Patterns follow `glob` syntax and
/// are matched against the source path relative to the project root.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerSpec {
    /// Human-readable layer name.
    pub name: String,
    /// Glob patterns describing which files belong to this layer.
    pub patterns: Vec<String>,
}
