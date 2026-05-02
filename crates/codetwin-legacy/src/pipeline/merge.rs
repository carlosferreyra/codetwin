//! Merge multiple driver outputs into a single [`CodeModel`].

use crate::ir::CodeModel;

/// Fold `models` into one. Language becomes `"polyglot"` if mixed.
pub fn merge_all(models: Vec<CodeModel>) -> CodeModel {
    models.into_iter().fold(CodeModel::default(), |mut acc, m| {
        acc.merge(m);
        acc
    })
}
