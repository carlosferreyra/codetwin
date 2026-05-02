//! Architecture diff (NEW_ROADMAP Phase 4.b + Phase 4.c).

mod engine;
mod report;

pub use engine::{Change, DiffReport, diff};
pub use report::render_markdown;

use anyhow::Result;

use crate::config::Config;

/// Top-level entry point for `codetwin diff`.
pub fn run(config: &Config, _ref_a: Option<&str>, _ref_b: Option<&str>, _json: bool) -> Result<()> {
    let _ = config;
    // TODO(Phase 4.b + 4.c):
    //   1. Resolve `ref_a` → CodeModel (default: most recent snapshot)
    //   2. Resolve `ref_b` → CodeModel (default: working tree)
    //   3. Diff via `engine::diff`
    //   4. Render via `report::render_markdown` or JSON
    anyhow::bail!("diff is not implemented yet (NEW_ROADMAP Phase 4.b)")
}
