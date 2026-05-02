//! Go driver — NEW_ROADMAP Phase 5.b.

use std::path::{Path, PathBuf};

use anyhow::Result;

use super::Driver;
use crate::ir::CodeModel;

/// Go driver (detects `go.mod`).
#[derive(Default)]
pub struct GoDriver;

impl Driver for GoDriver {
    fn name(&self) -> &'static str {
        "go"
    }

    fn detect(&self, project_root: &Path) -> bool {
        project_root.join("go.mod").is_file()
    }

    fn parse(&self, _paths: &[PathBuf]) -> Result<CodeModel> {
        // TODO(Phase 5.b): use `tree-sitter-go` to extract packages,
        //                  structs, interfaces, functions, and methods.
        Ok(CodeModel::new(self.name()))
    }
}
