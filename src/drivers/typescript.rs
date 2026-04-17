//! TypeScript driver — NEW_ROADMAP Phase 5.a.

use std::path::{Path, PathBuf};

use anyhow::Result;

use super::Driver;
use crate::ir::CodeModel;

/// TypeScript driver (detects `tsconfig.json` or `package.json` with a
/// TypeScript dependency).
#[derive(Default)]
pub struct TypeScriptDriver;

impl Driver for TypeScriptDriver {
    fn name(&self) -> &'static str {
        "typescript"
    }

    fn detect(&self, project_root: &Path) -> bool {
        // TODO(Phase 5.a): also inspect package.json for a `typescript`
        //                  entry in dependencies / devDependencies.
        project_root.join("tsconfig.json").is_file()
    }

    fn parse(&self, _paths: &[PathBuf]) -> Result<CodeModel> {
        // TODO(Phase 5.a): use `tree-sitter-typescript` to extract
        //                  classes, interfaces, functions, exports.
        Ok(CodeModel::new(self.name()))
    }
}
