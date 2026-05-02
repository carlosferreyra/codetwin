//! Rendering strategies (NEW_ROADMAP Phase 1.c + Phase 2).
//!
//! Each layout is a stateless renderer that consumes a [`crate::ir::CodeModel`]
//! and produces one or more [`OutputFile`]s. Layouts are registered in a
//! [`LayoutRegistry`] so the CLI can select them by name.

mod architecture_map;
mod c4;
mod metrics;
mod output_file;
mod project_overview;
mod registry;

pub use architecture_map::ArchitectureMapLayout;
pub use c4::C4Layout;
pub use metrics::MetricsLayout;
pub use output_file::OutputFile;
pub use project_overview::ProjectOverviewLayout;
pub use registry::LayoutRegistry;

use anyhow::Result;

use crate::config::Config;
use crate::ir::CodeModel;

/// Contract every documentation layout satisfies.
pub trait Layout: Send + Sync {
    /// Short, stable identifier used by the CLI (`--layout`).
    fn name(&self) -> &'static str;

    /// Render `model` into one or more output files.
    ///
    /// Implementations should be deterministic: the same `(model, config)`
    /// must yield byte-identical output.
    fn render(&self, model: &CodeModel, config: &Config) -> Result<Vec<OutputFile>>;
}
