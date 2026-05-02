//! Human-readable rendering of a [`DiffReport`].

use super::DiffReport;
use crate::render::markdown::MarkdownBuilder;

/// Render `report` as a Markdown string.
///
/// TODO(Phase 4.c): produce before/after Mermaid diagrams with
///                  colour-coded nodes (green = added, red = removed,
///                  yellow = modified) using Mermaid `classDef`.
pub fn render_markdown(report: &DiffReport) -> String {
    let mut md = MarkdownBuilder::new();
    md.heading(1, "Architecture Diff")
        .paragraph(&format!("{} structural change(s).", report.changes.len()));
    md.finish()
}
