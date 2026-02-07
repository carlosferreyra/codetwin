//! CodeTwin - Code to Diagram Generator
//! Using anyhow for error handling
pub use anyhow::{Context, Result};

/// Exposes modules so 'tests/' can see them
pub mod app;
pub mod cli;
pub mod core;
pub mod drivers;
pub mod io;
pub mod layouts;

// Preserve existing public paths
pub use app::engine;
pub use core::{config, discovery, ir};
