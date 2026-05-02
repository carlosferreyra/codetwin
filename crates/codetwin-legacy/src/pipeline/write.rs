//! Write [`OutputFile`]s to disk, creating parent directories as needed.

use anyhow::{Context, Result};

use crate::layouts::OutputFile;

/// Persist each output file; parents are created recursively.
pub fn write_outputs(outputs: &[OutputFile]) -> Result<()> {
    for out in outputs {
        if let Some(parent) = out.path.parent()
            && !parent.as_os_str().is_empty()
        {
            std::fs::create_dir_all(parent).with_context(|| {
                format!(
                    "failed to create parent directory for {}",
                    out.path.display()
                )
            })?;
        }
        std::fs::write(&out.path, &out.content)
            .with_context(|| format!("failed to write {}", out.path.display()))?;
        tracing::info!(path = %out.path.display(), bytes = out.content.len(), "wrote output");
    }
    Ok(())
}
