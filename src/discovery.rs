/// File discovery - find source files based on directory patterns
use std::fs;
use std::path::{Path, PathBuf};

/// Find all Rust files in the given source directories
pub fn find_rust_files(source_dirs: &[String]) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();

    for dir_str in source_dirs {
        let dir = Path::new(dir_str);

        if !dir.exists() {
            return Err(format!("Source directory does not exist: {}", dir_str));
        }

        if !dir.is_dir() {
            return Err(format!("Not a directory: {}", dir_str));
        }

        // Recursively find all .rs files
        find_rs_files_recursive(dir, &mut files)?;
    }

    // Sort for consistent output
    files.sort();

    Ok(files)
}

/// Recursively walk directory tree and collect .rs files
fn find_rs_files_recursive(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries = fs::read_dir(dir)
        .map_err(|e| format!("Failed to read directory {}: {}", dir.display(), e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;

        let path = entry.path();

        // Skip target/ and other build directories
        if should_skip(&path) {
            continue;
        }

        if path.is_dir() {
            find_rs_files_recursive(&path, files)?;
        } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            files.push(path);
        }
    }

    Ok(())
}

/// Check if path should be skipped during discovery
fn should_skip(path: &Path) -> bool {
    let skip_dirs = ["target", "build", ".git", "node_modules", ".hidden"];

    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| skip_dirs.contains(&name) || name.starts_with('.'))
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
