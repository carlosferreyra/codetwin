//! Workspace packages and their declared local dependencies.

use serde::{Deserialize, Serialize};

/// Stable package identity: its repository-relative manifest path.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PackageId(pub String);

/// A build target declared by a package.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PackageTarget {
    /// Target name.
    pub name: String,
    /// Cargo target kinds, such as `lib`, `bin`, or `test`.
    pub kinds: Vec<String>,
    /// Repository-relative source entrypoint.
    pub source_path: String,
}

/// A package belonging to the analyzed workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Package {
    /// Identity independent of the machine's absolute paths.
    pub id: PackageId,
    /// Name declared in the manifest.
    pub name: String,
    /// Repository-relative manifest path.
    pub manifest_path: String,
    /// Declared build targets in canonical order.
    pub targets: Vec<PackageTarget>,
}

/// The use of a declared package dependency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageDependencyKind {
    /// Dependency of ordinary package code.
    Normal,
    /// Dependency used by tests or examples.
    Dev,
    /// Dependency used by a build script.
    Build,
}

/// A declared local relationship, without evaluating features or target conditions.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PackageDependency {
    /// Package declaring the dependency.
    pub from: PackageId,
    /// Workspace package depended on.
    pub to: PackageId,
    /// Normal, development, or build dependency.
    pub kind: PackageDependencyKind,
    /// Dependency alias, if renamed in the declaring manifest.
    pub alias: Option<String>,
    /// Whether a feature must enable this dependency.
    pub optional: bool,
    /// Target condition, if the declaration is conditional.
    pub target: Option<String>,
}
