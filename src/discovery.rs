/// The "Inference" - Logic to detect src/ docs/ and Language

use std::path::PathBuf;

pub struct ProjectDiscovery;

impl ProjectDiscovery {
    pub fn new() -> Self {
        ProjectDiscovery
    }

    pub fn detect_source_dir(&self) -> Option<PathBuf> {
        // TODO: Auto-detect src/ lib/ app/
        None
    }

    pub fn detect_docs_dir(&self) -> Option<PathBuf> {
        // TODO: Auto-detect docs/ or README.md
        None
    }

    pub fn detect_language(&self, _path: &PathBuf) -> Option<String> {
        // TODO: Detect language from file extension
        None
    }
}
