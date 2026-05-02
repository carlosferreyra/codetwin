//! Filesystem helpers.

use std::path::Path;

use anyhow::{Context, Result};

/// Ensure `path`'s parent directory exists (no-op if the path is rooted at `/`).
pub fn ensure_parent_dir(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    Ok(())
}
