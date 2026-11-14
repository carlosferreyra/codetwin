//! CodeTwin configuration schema and loader.
//!
//! Provides [`Config`], [`OutputFormat`], and [`LayerSpec`] — the full
//! `codetwin.toml` surface — plus the zero-config fallback
//! [`Config::load_or_default`].

mod config;

pub use config::{Config, DEFAULT_CONFIG_PATH, LayerSpec, OutputFormat};
