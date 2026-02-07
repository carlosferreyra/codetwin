use anyhow::{Result, anyhow};
use glob::Pattern;
use ignore::WalkBuilder;
/// File discovery - find source files based on directory patterns
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tracing::debug;

/// Find all supported source files in the given source directories
pub fn find_source_files(
    source_dirs: &[String],
    exclude_patterns: &[String],
) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let root = Arc::new(std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    let exclude_globs = Arc::new(compile_exclude_patterns(exclude_patterns));

    for dir_str in source_dirs {
        let dir = Path::new(dir_str);

        if !dir.exists() {
            return Err(anyhow!("Source directory does not exist: {}", dir_str));
        }

        if dir.is_file() {
            if !should_skip(dir, exclude_globs.as_slice(), root.as_ref())
                && is_supported_source_file(dir)
            {
                files.push(dir.to_path_buf());
                continue;
            }
            return Err(anyhow!("Unsupported source file: {}", dir_str));
        }

        if !dir.is_dir() {
            return Err(anyhow!("Not a directory: {}", dir_str));
        }

        debug!("Discovering files in: {}", dir_str);

        let root = Arc::clone(&root);
        let exclude_globs = Arc::clone(&exclude_globs);
        let walker = WalkBuilder::new(dir)
            .standard_filters(true)
            .require_git(false)
            .filter_entry(move |entry| {
                !should_skip(entry.path(), exclude_globs.as_slice(), root.as_ref())
            })
            .build();

        for entry in walker {
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    debug!("Skipping entry due to walk error: {}", err);
                    continue;
                }
            };

            let path = entry.path();
            if is_supported_source_file(path) {
                files.push(path.to_path_buf());
            }
        }
    }

    // Sort for consistent output
    files.sort();

    Ok(files)
}

/// Check if path should be skipped during discovery using glob + gitignore patterns
fn should_skip(path: &Path, exclude_patterns: &[Pattern], root: &Path) -> bool {
    if is_hidden(path) {
        return true;
    }

    let rel_path = path.strip_prefix(root).unwrap_or(path);
    let path_str = rel_path.to_string_lossy();

    // Check if path matches any exclude pattern
    for pattern in exclude_patterns {
        if pattern.matches(&path_str) {
            return true;
        }
    }

    // Also skip hidden files and directories
    false
}

fn is_supported_source_file(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|s| s.to_str()),
        Some("rs") | Some("py")
    )
}

fn compile_exclude_patterns(patterns: &[String]) -> Vec<Pattern> {
    patterns
        .iter()
        .filter_map(|pattern| Pattern::new(pattern).ok())
        .collect()
}

fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.starts_with('.') && name != ".")
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_find_source_files() {
        // Test on actual src directory
        let files = find_source_files(&["src".to_string()], &[]).expect("Failed to find files");

        // Should find at least main.rs and lib.rs
        assert!(!files.is_empty(), "Should find .rs files in src/");

        // All should be supported source files
        for f in files {
            let extension = f.extension().and_then(|s| s.to_str());
            assert!(
                matches!(extension, Some("rs") | Some("py")),
                "Should only find supported source files"
            );
        }
    }

    #[test]
    fn test_find_source_files_accepts_file_paths() {
        let tmp_dir = std::env::temp_dir();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let file_path = tmp_dir.join(format!("codetwin_discovery_{}.py", now));

        fs::write(&file_path, "def helper():\n    return 1\n").expect("write temp file");

        let files = find_source_files(&[file_path.to_string_lossy().to_string()], &[])
            .expect("Should accept file path");

        assert_eq!(files.len(), 1);
        assert_eq!(files[0], file_path);

        fs::remove_file(&file_path).expect("cleanup temp file");
    }

    #[test]
    fn test_nested_gitignore_excludes_children() {
        let tmp_dir = std::env::temp_dir();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let root = tmp_dir.join(format!("codetwin_nested_gitignore_{}", now));
        let src_dir = root.join("src");
        let nested_dir = src_dir.join("nested");

        fs::create_dir_all(&nested_dir).expect("create nested dir");
        fs::write(src_dir.join("keep.rs"), "fn main() {}\n").expect("write keep");
        fs::write(nested_dir.join("ignored.rs"), "fn ignored() {}\n").expect("write ignored");
        fs::write(nested_dir.join("ok.py"), "def ok():\n    return 1\n").expect("write ok");
        fs::write(nested_dir.join(".gitignore"), "ignored.rs\n").expect("write gitignore");

        let files = find_source_files(&[src_dir.to_string_lossy().to_string()], &[])
            .expect("discover files");

        assert!(files.iter().any(|p| p.ends_with("keep.rs")));
        assert!(files.iter().any(|p| p.ends_with("ok.py")));
        assert!(!files.iter().any(|p| p.ends_with("ignored.rs")));

        fs::remove_dir_all(&root).expect("cleanup temp dir");
    }
}
