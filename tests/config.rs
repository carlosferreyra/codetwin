//! `codetwin.toml` parsing, defaults, and missing-file fallback.

use codetwin::config::{Config, OutputFormat};
use pretty_assertions::assert_eq;
use tempfile::TempDir;

#[test]
fn defaults_are_sane_for_a_typical_rust_project() {
    let cfg = Config::default();
    assert_eq!(cfg.source_dirs, vec![std::path::PathBuf::from("src")]);
    assert_eq!(cfg.layout, "project-overview");
    assert_eq!(cfg.format, OutputFormat::Markdown);
    assert!(cfg.drivers.is_none());
}

#[test]
fn missing_file_returns_defaults() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("codetwin.toml");
    let cfg = Config::load_or_default_from(&path).unwrap();
    assert_eq!(cfg.layout, Config::default().layout);
}

#[test]
fn valid_file_parses() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("codetwin.toml");
    std::fs::write(
        &path,
        r#"
source_dirs = ["src", "crates/core/src"]
output_file = "docs/ARCH.md"
layout = "architecture-map"
format = "markdown"
exclude_patterns = ["**/target/**"]
"#,
    )
    .unwrap();

    let cfg = Config::load_or_default_from(&path).unwrap();
    assert_eq!(cfg.layout, "architecture-map");
    assert_eq!(cfg.source_dirs.len(), 2);
}

#[test]
fn unknown_field_is_rejected() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("codetwin.toml");
    std::fs::write(&path, "not_a_real_field = 42\n").unwrap();

    let err = Config::load_or_default_from(&path).unwrap_err();
    assert!(format!("{err:#}").contains("not_a_real_field"));
}

#[test]
fn roundtrip_through_toml() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("codetwin.toml");

    let cfg = Config::default();
    cfg.save_to(&path).unwrap();

    let parsed = Config::load_or_default_from(&path).unwrap();
    assert_eq!(parsed.layout, cfg.layout);
    assert_eq!(parsed.source_dirs, cfg.source_dirs);
}

// TODO(Phase 1.e): test that `exclude_patterns` are applied during discovery.
