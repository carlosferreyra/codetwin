//! Layout selection + invocation.

use anyhow::{Result, anyhow};

use crate::config::Config;
use crate::ir::CodeModel;
use crate::layouts::{LayoutRegistry, OutputFile};

/// Pick the configured layout and render `model`.
pub fn render(model: &CodeModel, config: &Config) -> Result<Vec<OutputFile>> {
    let registry = LayoutRegistry::default();
    let layout = registry
        .get(&config.layout)
        .ok_or_else(|| anyhow!(crate::Error::LayoutNotFound(config.layout.clone())))?;

    tracing::info!(layout = layout.name(), "rendering");
    layout.render(model, config)
}
