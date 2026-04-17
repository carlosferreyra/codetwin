//! Output-format enum.

use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// Supported output formats.
///
/// Only `Markdown` is implemented for the MVP; `Html` is reserved for
/// NEW_ROADMAP Phase 7.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    /// Inline-Mermaid Markdown (MVP).
    #[default]
    Markdown,
    /// Static-SPA HTML — TODO(Phase 7.a).
    Html,
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "markdown" | "md" => Ok(Self::Markdown),
            "html" => Ok(Self::Html),
            other => anyhow::bail!("unknown output format: {other}"),
        }
    }
}
