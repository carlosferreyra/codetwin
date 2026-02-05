use anyhow::{anyhow, Result};
use glob::Pattern;
/// File discovery - find source files based on directory patterns
use std::path::{Path, PathBuf};
use tracing::debug;
use walkdir::WalkDir;

/// Find all Rust files in the given source directories
pub fn find_rust_files(source_dirs: &[String]) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for dir_str in source_dirs {
        let dir = Path::new(dir_str);

        if !dir.exists() {
            return Err(anyhow!("Source directory does not exist: {}", dir_str));
        }

        if !dir.is_dir() {
            return Err(anyhow!("Not a directory: {}", dir_str));
        }

        debug!("Discovering files in: {}", dir_str);

        // Use walkdir to recursively find all .rs files
        for entry in WalkDir::new(dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !should_skip(e.path()))
        {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                files.push(path.to_path_buf());
            }
        }
    }

    // Sort for consistent output
    files.sort();

    Ok(files)
}

/// Check if path should be skipped during discovery using glob patterns
fn should_skip(path: &Path) -> bool {
    // Default exclude patterns
    let exclude_patterns = vec![
        "**/target/**",
        "**/node_modules/**",
        "**/.git/**",
        "**/tests/**",
        "**/.hidden/*",
    ];

    let path_str = path.to_string_lossy();

    // Check if path matches any exclude pattern
    for pattern_str in exclude_patterns {
        if let Ok(pattern) = Pattern::new(pattern_str) {
            if pattern.matches(&path_str) {
                return true;
            }
        }
    }

    // Also skip hidden files and directories
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.starts_with('.') && name != ".")
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_rust_files() {
        // Test on actual src directory
        let files = find_rust_files(&["src".to_string()]).expect("Failed to find files");

        // Should find at least main.rs and lib.rs
        assert!(!files.is_empty(), "Should find .rs files in src/");

        // All should be .rs files
        for f in files {
            assert_eq!(
                f.extension().and_then(|s| s.to_str()),
                Some("rs"),
                "Should only find .rs files"
            );
        }
    }
}
