//! Pipeline orchestration (NEW_ROADMAP Phase 1.d).
//!
//! Each stage is a standalone function so it can be unit-tested in isolation:
//!
//! ```text
//! discover → drivers.detect → parse (rayon) → merge → layout.render → write
//! ```

mod discover;
pub mod merge;
mod render;
mod write;

pub use discover::discover;
pub use render::render;
pub use write::write_outputs;

use anyhow::{Context, Result};
use rayon::prelude::*;

use crate::config::Config;
use crate::drivers::DriverRegistry;
use crate::ir::CodeModel;

/// Runtime options derived from CLI arguments, scrubbed of persistence
/// concerns (those are handled in the CLI layer).
#[derive(Debug, Clone, Default)]
pub struct GenOptions {
    /// Dump the merged IR as JSON instead of rendering.
    pub dump_ir: bool,
    /// One file per module/layer (NEW_ROADMAP Phase 6.d).
    pub multi_file: bool,
}

/// Run the full pipeline once.
///
/// This is the function invoked by `codetwin gen` and (via the watch loop)
/// on every filesystem change.
pub fn run(config: &Config, opts: &GenOptions, json: bool) -> Result<()> {
    let project_root = std::env::current_dir()?;
    let files = discover(config).context("file discovery failed")?;
    tracing::info!(count = files.len(), "discovered source files");

    let registry = DriverRegistry::default();
    let active: Vec<_> = match &config.drivers {
        Some(names) => names.iter().filter_map(|n| registry.get(n)).collect(),
        None => registry.detect_all(&project_root),
    };

    if active.is_empty() {
        tracing::warn!("no drivers matched the project; output will be empty");
    } else {
        tracing::info!(
            drivers = ?active.iter().map(|d| d.name()).collect::<Vec<_>>(),
            "active drivers"
        );
    }

    let models: Vec<CodeModel> = active
        .par_iter()
        .map(|driver| driver.parse(&files))
        .collect::<Result<Vec<_>>>()?;

    let merged = merge::merge_all(models);

    if opts.dump_ir || json {
        let json = serde_json::to_string_pretty(&merged)?;
        println!("{json}");
        return Ok(());
    }

    let outputs = render(&merged, config)?;

    if opts.multi_file {
        // TODO(Phase 6.d): split layout output into one file per module/layer.
        tracing::warn!("--multi-file not yet implemented; writing single file");
    }

    write_outputs(&outputs)
}
