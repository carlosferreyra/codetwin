//! Format-agnostic output file.

use std::path::PathBuf;

use crate::config::OutputFormat;

/// A single rendered file ready to be written to disk.
#[derive(Debug, Clone, PartialEq)]
pub struct OutputFile {
    /// Destination path.
    pub path: PathBuf,
    /// Rendered content.
    pub content: String,
    /// Format tag — consumers may use this to add a file extension, apply
    /// HTML post-processing, etc.
    pub format: OutputFormat,
}
