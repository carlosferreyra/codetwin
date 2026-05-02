//! On-disk snapshot store — `.codetwin/snapshots/<ref>.json`.

use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::ir::CodeModel;

/// Default directory that holds cached snapshots.
pub fn snapshot_dir() -> PathBuf {
    PathBuf::from(".codetwin").join("snapshots")
}

/// Thin wrapper around the snapshot directory.
pub struct SnapshotStore {
    root: PathBuf,
}

impl Default for SnapshotStore {
    fn default() -> Self {
        Self {
            root: snapshot_dir(),
        }
    }
}

impl SnapshotStore {
    /// Open a store rooted at `root`.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Absolute path for a given snapshot name (e.g. a git short SHA).
    pub fn path_for(&self, name: &str) -> PathBuf {
        self.root.join(format!("{name}.json"))
    }

    /// Load a snapshot by name.
    pub fn load(&self, name: &str) -> Result<CodeModel> {
        let path = self.path_for(name);
        let text = std::fs::read_to_string(&path)?;
        Ok(serde_json::from_str(&text)?)
    }

    /// Persist a snapshot under `name`.
    pub fn save(&self, name: &str, model: &CodeModel) -> Result<PathBuf> {
        std::fs::create_dir_all(&self.root)?;
        let path = self.path_for(name);
        std::fs::write(&path, serde_json::to_string_pretty(model)?)?;
        Ok(path)
    }

    /// Root directory of the store (useful for tests).
    pub fn root(&self) -> &Path {
        &self.root
    }
}
