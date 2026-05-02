//! File discovery (NEW_ROADMAP Phase 1.e).

use std::path::PathBuf;

use anyhow::Result;
use ignore::WalkBuilder;

use crate::config::Config;

/// Walk `config.source_dirs` and return the list of files to parse.
///
/// Respects `.gitignore` + nested ignore files via the `ignore` crate, and
/// applies the `config.exclude_patterns` glob list on top.
pub fn discover(config: &Config) -> Result<Vec<PathBuf>> {
    // TODO(Phase 1.e): apply `config.exclude_patterns` as glob filters,
    //                  handle `source_dirs` that don't exist with a
    //                  friendly error (NEW_ROADMAP "zero-config" goal).
    let mut results = Vec::new();

    for dir in &config.source_dirs {
        if !dir.exists() {
            tracing::warn!(path = %dir.display(), "source directory does not exist");
            continue;
        }
        let walker = WalkBuilder::new(dir)
            .hidden(false)
            .git_ignore(true)
            .git_exclude(true)
            .build();
        for entry in walker {
            let entry = entry?;
            if entry.file_type().is_some_and(|ft| ft.is_file()) {
                results.push(entry.into_path());
            }
        }
    }

    results.sort();
    Ok(results)
}
