//! Exercise repository scaffolds through the installed command interface.

use std::fs;
use std::path::Path;
use std::process::{Command, Output};

use tempfile::TempDir;

fn fixture() -> TempDir {
    let dir = TempDir::new().unwrap();
    fs::create_dir_all(dir.path().join("crates/fixture/src")).unwrap();
    fs::write(
        dir.path().join("Cargo.toml"),
        r#"[workspace]
members = ["crates/*"]
resolver = "3"
[workspace.package]
version = "0.2.0"
edition = "2024"
rust-version = "1.93"
authors = []
license = "MIT"
repository = "https://example.com/fixture"
homepage = "https://example.com/fixture"
[workspace.dependencies]
fixture = { path = "crates/fixture", version = "0.2.0" }
clap = { version = "4.5", features = ["derive"] }
[workspace.metadata.xtask]
crate-prefix = "fixture"
primary-crate = "fixture"
[workspace.lints.rust]
unsafe_code = "forbid"
"#,
    )
    .unwrap();
    fs::write(
        dir.path().join("crates/fixture/Cargo.toml"),
        "[package]\nname = \"fixture\"\nversion.workspace = true\nedition.workspace = true\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("crates/fixture/src/lib.rs"),
        "//! Existing public library.\npub fn existing() {}\n",
    )
    .unwrap();
    dir
}

fn run(root: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_xtask"))
        .current_dir(root)
        .args(args)
        .output()
        .unwrap()
}

fn success(root: &Path, args: &[&str]) {
    let output = run(root, args);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn dry_run_from_member_directory_preserves_workspace() {
    let dir = fixture();
    let root = dir.path();
    let before = fs::read(root.join("Cargo.toml")).unwrap();
    success(
        &root.join("crates/fixture/src"),
        &["scaffold", "crate", "resolver", "--dry-run"],
    );
    assert_eq!(before, fs::read(root.join("Cargo.toml")).unwrap());
    assert!(!root.join("crates/fixture-resolver").exists());
}

#[test]
fn crate_scaffold_is_idempotent_and_preserves_customized_source() {
    let dir = fixture();
    let root = dir.path();
    let args = ["scaffold", "crate", "resolver"];
    success(root, &args);
    let before = fs::read(root.join("Cargo.toml")).unwrap();
    success(root, &args);
    assert_eq!(before, fs::read(root.join("Cargo.toml")).unwrap());

    let source = root.join("crates/fixture-resolver/src/lib.rs");
    fs::write(&source, "//! User-owned implementation.\n").unwrap();
    assert!(!run(root, &args).status.success());
    assert_eq!(before, fs::read(root.join("Cargo.toml")).unwrap());
    assert_eq!(
        fs::read_to_string(source).unwrap(),
        "//! User-owned implementation.\n"
    );
}

#[test]
fn ci_conflicting_parent_rejects_all_writes() {
    let dir = fixture();
    let root = dir.path();
    fs::write(root.join(".github"), "user-owned file\n").unwrap();
    for extra in [vec![], vec!["--dry-run"]] {
        let mut args = vec!["scaffold", "ci", "--preset", "full"];
        args.extend(extra);
        assert!(!run(root, &args).status.success());
        assert!(!root.join("deny.toml").exists());
        assert!(!root.join("typos.toml").exists());
        assert_eq!(
            fs::read_to_string(root.join(".github")).unwrap(),
            "user-owned file\n"
        );
    }
}

#[test]
fn agent_scaffold_preserves_project_rules_without_partial_writes() {
    let dir = fixture();
    let root = dir.path();
    fs::write(root.join("AGENTS.md"), "# Project-specific rules\n").unwrap();
    assert!(
        !run(root, &["scaffold", "agents", "--claude"])
            .status
            .success()
    );
    assert_eq!(
        fs::read_to_string(root.join("AGENTS.md")).unwrap(),
        "# Project-specific rules\n"
    );
    assert!(!root.join("CLAUDE.md").exists());
}

#[test]
fn full_ci_resolves_actions_and_preserves_github_expressions() {
    let dir = fixture();
    let root = dir.path();
    let args = ["scaffold", "ci", "--preset", "full"];
    success(root, &args);
    let workflow = fs::read_to_string(root.join(".github/workflows/ci.yml")).unwrap();
    assert!(workflow.contains("${{ github.ref }}"));
    assert!(workflow.contains("cargo xtask ci --full"));
    assert!(!workflow.contains("@checkout@"));
    assert!(!workflow.contains("{{checkout_version}}"));
    assert!(root.join(".github/workflows/audit.yml").is_file());
    assert!(root.join(".github/dependabot.yml").is_file());
    success(root, &args);
    assert_eq!(
        workflow,
        fs::read_to_string(root.join(".github/workflows/ci.yml")).unwrap()
    );
}

#[test]
fn primary_cli_preserves_library_and_versions_publishable_dependency() {
    let dir = fixture();
    let root = dir.path();
    let source = root.join("crates/fixture/src/lib.rs");
    let before = fs::read(&source).unwrap();
    success(root, &["scaffold", "cli", "--entrypoint", "primary"]);
    assert_eq!(before, fs::read(source).unwrap());
    let manifest = fs::read_to_string(root.join("Cargo.toml")).unwrap();
    assert!(
        manifest.contains("fixture-cli = { path = \"crates/fixture-cli\", version = \"0.2.0\" }")
    );
    let cli = fs::read_to_string(root.join("crates/fixture-cli/Cargo.toml")).unwrap();
    assert!(!cli.contains("publish = false"));
    assert!(root.join("crates/fixture/src/bin/fixture.rs").is_file());
}
