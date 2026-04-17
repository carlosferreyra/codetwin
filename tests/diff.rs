//! Structural diff smoke tests (NEW_ROADMAP Phase 4.b).

use codetwin::diff::diff;
use codetwin::ir::CodeModel;

#[test]
fn diffing_identical_models_returns_no_changes() {
    let model = CodeModel::new("rust");
    let report = diff(&model, &model);
    assert!(report.changes.is_empty());
}

// TODO(Phase 4.b): assert added / removed modules are detected.
// TODO(Phase 4.b): assert renames are detected via symbol-level fuzzy match.
// TODO(Phase 4.b): assert cosmetic changes (whitespace, comment diffs) are ignored.
