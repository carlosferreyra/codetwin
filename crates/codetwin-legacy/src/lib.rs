//! CodeTwin — zero-config, language-agnostic documentation generator.
//!
//! This crate exposes the building blocks of the CodeTwin pipeline so the
//! binary (`src/main.rs`), integration tests, and downstream tools can reuse
//! them.
//!
//! ## Crate layout
//!
//! | Module       | Responsibility                                              |
//! | ------------ | ----------------------------------------------------------- |
//! | [`cli`]      | `clap`-derived CLI surface and argument parsing             |
//! | [`config`]   | `codetwin.toml` schema and loading                          |
//! | [`ir`]       | Intermediate representation (`CodeModel`, symbols, edges)   |
//! | [`drivers`]  | Language parsers + auto-detection registry                  |
//! | [`layouts`]  | Rendering strategies + registry                             |
//! | [`pipeline`] | Orchestrates discover → parse → merge → render → write      |
//! | [`render`]   | Output helpers (Markdown, Mermaid, and future HTML)         |
//! | [`snapshot`] | `CodeModel` snapshot capture / on-disk cache                |
//! | [`diff`]     | Structural diff between two snapshots                       |
//! | [`watch`]    | Filesystem watcher (shared by `gen`, `snapshot`, `diff`)    |
//! | [`util`]     | Small cross-cutting helpers                                 |
//!
//! See `ROADMAP.md` for the phased implementation plan.

#![deny(rust_2018_idioms)]
#![warn(missing_docs)]

pub use anyhow::{Context, Result};

pub mod cli;
pub mod config;
pub mod diff;
pub mod drivers;
pub mod ir;
pub mod layouts;
pub mod pipeline;
pub mod render;
pub mod snapshot;
pub mod util;
pub mod watch;

/// Library-wide error type.
///
/// Most call sites should use [`anyhow::Result`] for flexibility; this enum
/// is surfaced at module boundaries where structured errors pay off (e.g. the
/// driver or layout registry).
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// No driver could be auto-detected for the given project.
    #[error("no driver matched the project at {0}")]
    NoDriverMatched(std::path::PathBuf),

    /// Requested layout is not registered.
    #[error("layout not found: {0}")]
    LayoutNotFound(String),

    /// Requested driver is not registered.
    #[error("driver not found: {0}")]
    DriverNotFound(String),

    /// Parsing failed for a specific file.
    #[error("parse error in {path}: {reason}")]
    Parse {
        /// File that failed to parse.
        path: std::path::PathBuf,
        /// Human-readable reason.
        reason: String,
    },

    /// Underlying I/O error.
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
