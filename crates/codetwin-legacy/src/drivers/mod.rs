//! Language drivers + auto-detection registry (NEW_ROADMAP Phase 1.b).
//!
//! Every language integration implements [`Driver`]. A [`DriverRegistry`]
//! collects the available drivers and picks the ones whose [`Driver::detect`]
//! returns `true` for the current project root.

mod go;
mod python;
mod registry;
mod rust;
mod typescript;

pub use go::GoDriver;
pub use python::PythonDriver;
pub use registry::DriverRegistry;
pub use rust::RustDriver;
pub use typescript::TypeScriptDriver;

use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::ir::CodeModel;

/// Contract every language integration satisfies.
///
/// Drivers are expected to be cheap to construct: the registry creates one
/// instance per registered driver and calls [`Driver::detect`] on it.
pub trait Driver: Send + Sync {
    /// Short, stable identifier (e.g. `"rust"`, `"python"`). Used for CLI
    /// flags and `codetwin list --drivers` output.
    fn name(&self) -> &'static str;

    /// Return `true` if this driver can meaningfully parse the project at
    /// `project_root` (usually by sniffing manifest files).
    fn detect(&self, project_root: &Path) -> bool;

    /// Parse `paths` and produce a [`CodeModel`].
    ///
    /// Implementations may be called from multiple threads concurrently by
    /// the pipeline — see NEW_ROADMAP Phase 1.d.
    fn parse(&self, paths: &[PathBuf]) -> Result<CodeModel>;
}
