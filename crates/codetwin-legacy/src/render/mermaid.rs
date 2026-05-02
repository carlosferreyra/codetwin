//! Mermaid diagram helpers.

use crate::ir::{CodeModel, EdgeKind};

/// Render `model.edges` as a Mermaid `graph TD` block.
///
/// TODO(Phase 2.c): support subgraphs for layers, Mermaid class styling
///                  for diff colour-coding (Phase 4.c), and themes via
///                  `%%{init:}%%` (Phase 6.c).
pub fn graph_td(model: &CodeModel) -> String {
    let mut out = String::from("graph TD\n");
    for edge in &model.edges {
        let arrow = match edge.kind {
            EdgeKind::Import => "-->",
            EdgeKind::Uses => "-.->",
            EdgeKind::Implements => "==>",
            EdgeKind::Extends => "==>",
            EdgeKind::Calls => "-->",
        };
        out.push_str(&format!(
            "  {} {} {}\n",
            sanitize(&edge.from.0),
            arrow,
            sanitize(&edge.to.0)
        ));
    }
    out
}

fn sanitize(id: &str) -> String {
    id.replace([':', '-', '.', '/', '\\'], "_")
}
