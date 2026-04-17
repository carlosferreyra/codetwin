//! `SnapshotStore` round-trip (NEW_ROADMAP Phase 4.a).

use codetwin::ir::{CodeModel, Module, ModuleId};
use codetwin::snapshot::SnapshotStore;
use pretty_assertions::assert_eq;
use tempfile::TempDir;

#[test]
fn save_then_load_returns_the_same_model() {
    let dir = TempDir::new().unwrap();
    let store = SnapshotStore::new(dir.path());

    let mut model = CodeModel::new("rust");
    model.modules.push(Module {
        id: ModuleId::from("crate::root"),
        name: "root".to_string(),
        path: "src/lib.rs".into(),
        symbols: Vec::new(),
        doc: None,
    });

    let path = store.save("abc1234", &model).unwrap();
    assert!(path.exists());

    let loaded = store.load("abc1234").unwrap();
    assert_eq!(loaded, model);
}

#[test]
fn load_missing_snapshot_errors() {
    let dir = TempDir::new().unwrap();
    let store = SnapshotStore::new(dir.path());
    assert!(store.load("deadbeef").is_err());
}

// TODO(Phase 4.a): capture() against a git worktree for `--ref`.
