//! Driver registry detection + lookup (NEW_ROADMAP Phase 1.b).

use codetwin::drivers::DriverRegistry;
use pretty_assertions::assert_eq;
use tempfile::TempDir;

#[test]
fn default_registry_lists_all_builtins() {
    let names = DriverRegistry::default().names();
    assert_eq!(names, vec!["rust", "python", "typescript", "go"]);
}

#[test]
fn detects_rust_by_cargo_toml() {
    let dir = TempDir::new().unwrap();
    std::fs::write(dir.path().join("Cargo.toml"), "[package]\n").unwrap();

    let active: Vec<_> = DriverRegistry::default()
        .detect_all(dir.path())
        .into_iter()
        .map(|d| d.name())
        .collect();
    assert_eq!(active, vec!["rust"]);
}

#[test]
fn detects_python_by_pyproject_toml() {
    let dir = TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("pyproject.toml"),
        "[project]\nname = \"x\"\n",
    )
    .unwrap();

    let active: Vec<_> = DriverRegistry::default()
        .detect_all(dir.path())
        .into_iter()
        .map(|d| d.name())
        .collect();
    assert_eq!(active, vec!["python"]);
}

#[test]
fn polyglot_project_activates_multiple_drivers() {
    let dir = TempDir::new().unwrap();
    std::fs::write(dir.path().join("Cargo.toml"), "[package]\n").unwrap();
    std::fs::write(dir.path().join("pyproject.toml"), "[project]\n").unwrap();

    let active: Vec<_> = DriverRegistry::default()
        .detect_all(dir.path())
        .into_iter()
        .map(|d| d.name())
        .collect();
    assert!(active.contains(&"rust"));
    assert!(active.contains(&"python"));
}

#[test]
fn get_returns_none_for_unknown_driver() {
    assert!(DriverRegistry::default().get("pascal").is_none());
}

// TODO(Phase 1.b): integration test parsing a real Rust fixture end-to-end.
// TODO(Phase 5.a): add a `tsconfig.json` detection test for TypeScript.
// TODO(Phase 5.b): add a `go.mod` detection test for Go.
