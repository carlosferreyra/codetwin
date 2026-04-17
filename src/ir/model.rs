//! `CodeModel` — the root IR type.

use serde::{Deserialize, Serialize};

use super::{Edge, Module};

/// A language-agnostic snapshot of a project's structure.
///
/// Produced by one or more [`crate::drivers::Driver`]s and consumed by
/// layouts. Multiple `CodeModel`s can be merged — see
/// [`crate::pipeline::merge`].
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeModel {
    /// Modules (files or logical namespaces) in declaration order.
    pub modules: Vec<Module>,
    /// First-class dependency edges between modules/symbols.
    pub edges: Vec<Edge>,
    /// Language label for the producing driver (e.g. `"rust"`, `"python"`).
    ///
    /// When multiple drivers contribute, the merged model sets this to
    /// `"polyglot"`.
    pub language: String,
}

impl CodeModel {
    /// Create an empty model tagged with the given language.
    pub fn new(language: impl Into<String>) -> Self {
        Self {
            modules: Vec::new(),
            edges: Vec::new(),
            language: language.into(),
        }
    }

    /// Merge `other` into `self`, preserving stable order.
    ///
    /// TODO(Phase 1.d): de-duplicate symbols/edges that originate from the
    ///                  same file when two drivers contribute overlapping
    ///                  information (e.g. Rust `proc-macro` expansion).
    pub fn merge(&mut self, other: CodeModel) {
        self.modules.extend(other.modules);
        self.edges.extend(other.edges);
        if other.language.is_empty() {
            // Nothing to merge on the language axis.
        } else if self.language.is_empty() {
            self.language = other.language;
        } else if self.language != other.language {
            self.language = "polyglot".to_string();
        }
    }
}
