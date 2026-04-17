//! Python driver (tree-sitter-based).
//!
//! TODO(Phase 1.b): port existing Python extraction; include imports,
//!                  decorators, `__all__`, and type annotations.

use std::path::{Path, PathBuf};

use anyhow::Result;

use super::Driver;
use crate::ir::CodeModel;

/// Python driver (detects `pyproject.toml` or `setup.py`).
#[derive(Default)]
pub struct PythonDriver;

impl Driver for PythonDriver {
    fn name(&self) -> &'static str {
        "python"
    }

    fn detect(&self, project_root: &Path) -> bool {
        project_root.join("pyproject.toml").is_file()
            || project_root.join("setup.py").is_file()
            || project_root.join("setup.cfg").is_file()
    }

    fn parse(&self, _paths: &[PathBuf]) -> Result<CodeModel> {
        // TODO(Phase 1.b): extract classes, functions, imports, and
        //                  build Edges via `import` statements.
        Ok(CodeModel::new(self.name()))
    }
}
