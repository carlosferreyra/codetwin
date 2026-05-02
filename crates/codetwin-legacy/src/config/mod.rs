//! `codetwin.toml` schema and loading.
//!
//! The config is optional — [`Config::load_or_default`] never fails on a
//! missing file, matching the "zero-config" promise in NEW_ROADMAP Phase 1.e.

mod format;
mod layer;

pub use format::OutputFormat;
pub use layer::LayerSpec;

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// Default path for the project-local config.
pub const DEFAULT_CONFIG_PATH: &str = "codetwin.toml";

/// Runtime configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    /// Directories to scan for source files.
    pub source_dirs: Vec<PathBuf>,
    /// Output file (default layout writes one file; see `multi_file`).
    pub output_file: PathBuf,
    /// Layout name; must be registered in [`crate::layouts::LayoutRegistry`].
    pub layout: String,
    /// Output format (Markdown by default).
    pub format: OutputFormat,
    /// Patterns to exclude during discovery (glob syntax).
    pub exclude_patterns: Vec<String>,
    /// Optional explicit layer declarations (used by the architecture-map
    /// layout). When omitted, layers are auto-detected from directory
    /// structure.
    pub layers: Vec<LayerSpec>,
    /// Optional explicit driver list, overriding auto-detection.
    pub drivers: Option<Vec<String>>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            source_dirs: vec![PathBuf::from("src")],
            output_file: PathBuf::from("docs/architecture.md"),
            layout: "project-overview".to_string(),
            format: OutputFormat::Markdown,
            exclude_patterns: vec![
                "**/target/**".to_string(),
                "**/node_modules/**".to_string(),
                "**/.git/**".to_string(),
                "**/dist/**".to_string(),
            ],
            layers: Vec::new(),
            drivers: None,
        }
    }
}

impl Config {
    /// Load from `codetwin.toml` in the current working directory, falling
    /// back to defaults when the file is missing.
    pub fn load_or_default() -> Result<Self> {
        Self::load_or_default_from(DEFAULT_CONFIG_PATH)
    }

    /// Load from an explicit path, falling back to defaults on `NotFound`.
    pub fn load_or_default_from(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        match std::fs::read_to_string(path) {
            Ok(text) => {
                toml::from_str(&text).with_context(|| format!("failed to parse {}", path.display()))
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(Self::default()),
            Err(err) => {
                Err(anyhow::Error::new(err).context(format!("failed to read {}", path.display())))
            }
        }
    }

    /// Serialize and write the config to the default path.
    pub fn save_to_default_path(&self) -> Result<()> {
        self.save_to(DEFAULT_CONFIG_PATH)
    }

    /// Serialize and write the config to an explicit path.
    pub fn save_to(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        let text = toml::to_string_pretty(self).context("failed to serialize config")?;
        std::fs::write(path, text)
            .with_context(|| format!("failed to write {}", path.display()))?;
        Ok(())
    }
}
