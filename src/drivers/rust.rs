//! Rust driver (tree-sitter-based).
//!
//! TODO(Phase 1.b): port the existing tree-sitter extraction to the new
//!                  [`crate::ir::CodeModel`] contract. Until then this
//!                  driver detects Rust projects but produces an empty
//!                  model so the pipeline still runs end-to-end.

use std::path::{Path, PathBuf};

use anyhow::Result;

use super::Driver;
use crate::ir::CodeModel;

/// Rust driver (detects `Cargo.toml`).
#[derive(Default)]
pub struct RustDriver;

impl Driver for RustDriver {
    fn name(&self) -> &'static str {
        "rust"
    }

    fn detect(&self, project_root: &Path) -> bool {
        project_root.join("Cargo.toml").is_file()
    }

    fn parse(&self, _paths: &[PathBuf]) -> Result<CodeModel> {
        // TODO(Phase 1.b): load tree-sitter-rust grammar, walk each path,
        //                  extract modules/symbols/edges, return the model.
        Ok(CodeModel::new(self.name()))
    }
}
