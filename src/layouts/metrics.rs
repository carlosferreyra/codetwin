//! Coupling & metrics layout — NEW_ROADMAP Phase 6.b.

use anyhow::Result;

use super::{Layout, OutputFile};
use crate::config::Config;
use crate::ir::CodeModel;

/// Coupling / hub / circular-dependency report.
#[derive(Default)]
pub struct MetricsLayout;

impl Layout for MetricsLayout {
    fn name(&self) -> &'static str {
        "metrics"
    }

    fn render(&self, _model: &CodeModel, _config: &Config) -> Result<Vec<OutputFile>> {
        // TODO(Phase 6.b): compute fan-in / fan-out via petgraph, detect
        //                  circular deps (`tarjan_scc`), render tables.
        anyhow::bail!("layout `metrics` is not implemented yet (NEW_ROADMAP Phase 6.b)")
    }
}
