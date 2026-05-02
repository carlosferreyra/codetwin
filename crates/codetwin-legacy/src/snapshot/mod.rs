//! Snapshot capture & on-disk store (NEW_ROADMAP Phase 4.a).

mod store;

pub use store::{SnapshotStore, snapshot_dir};

use std::path::PathBuf;

use anyhow::Result;

use crate::config::Config;

/// Build a `CodeModel` for `git_ref` (or the working tree) and persist it.
///
/// Returns the path the snapshot was written to.
pub fn capture(config: &Config, git_ref: Option<&str>) -> Result<PathBuf> {
    // TODO(Phase 4.a): if `git_ref` is `Some`, `git worktree add` into a
    //                  temporary directory, run the pipeline against that
    //                  checkout, then clean up. For now we snapshot the
    //                  working tree regardless.
    let _ = git_ref;
    let _ = config;

    // TODO(Phase 4.a): call `crate::pipeline::discover` + driver.parse +
    //                  merge, then serialize the resulting CodeModel.
    anyhow::bail!("snapshot capture is not implemented yet (NEW_ROADMAP Phase 4.a)")
}
