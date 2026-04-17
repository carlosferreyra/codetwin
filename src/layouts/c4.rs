//! C4 Model layout — NEW_ROADMAP Phase 6.a.

use anyhow::Result;

use super::{Layout, OutputFile};
use crate::config::Config;
use crate::ir::CodeModel;

/// C4 Model layout (System Context / Container / Component / Code).
#[derive(Default)]
pub struct C4Layout;

impl Layout for C4Layout {
    fn name(&self) -> &'static str {
        "c4"
    }

    fn render(&self, _model: &CodeModel, _config: &Config) -> Result<Vec<OutputFile>> {
        // TODO(Phase 6.a): emit one section per C4 level; support optional
        //                  C4-PlantUML syntax for users who need it.
        anyhow::bail!("layout `c4` is not implemented yet (NEW_ROADMAP Phase 6.a)")
    }
}
