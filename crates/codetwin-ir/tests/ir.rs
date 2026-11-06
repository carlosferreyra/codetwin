//! IR serde round-trips and merge semantics (`NEW_ROADMAP` Phase 1.a).

use codetwin_ir::{
    CodeModel, Edge, EdgeKind, Module, ModuleId, Package, PackageDependency, PackageDependencyKind,
    PackageId, PackageTarget, Symbol, SymbolKind, Visibility,
};
use pretty_assertions::assert_eq;

fn sample_module() -> Module {
    Module {
        id: ModuleId::from("crate::cli"),
        name: "cli".to_string(),
        path: "src/cli/mod.rs".into(),
        symbols: vec![Symbol {
            name: "run".to_string(),
            kind: SymbolKind::Function,
            visibility: Visibility::Public,
            line: 42,
            doc: Some("Entry point.".to_string()),
            signature: Some("fn run() -> Result<()>".to_string()),
        }],
        doc: None,
    }
}

#[test]
fn code_model_json_round_trip() {
    let mut model = CodeModel::new("rust");
    model.modules.push(sample_module());
    model.edges.push(Edge {
        from: ModuleId::from("crate::cli"),
        to: ModuleId::from("crate::pipeline"),
        kind: EdgeKind::Import,
    });

    let json = serde_json::to_string(&model).unwrap();
    let parsed: CodeModel = serde_json::from_str(&json).unwrap();
    assert_eq!(model, parsed);
}

#[test]
fn merging_models_switches_language_to_polyglot() {
    let rust = CodeModel::new("rust");
    let python = CodeModel::new("python");

    let mut merged = rust;
    merged.merge(python);

    assert_eq!(merged.language, "polyglot");
}

#[test]
fn merging_empty_keeps_original_language() {
    let mut rust = CodeModel::new("rust");
    rust.merge(CodeModel::default());
    assert_eq!(rust.language, "rust");
}

#[test]
fn stored_models_without_workspace_fields_remain_readable() {
    let model: CodeModel =
        serde_json::from_str(r#"{"modules":[],"edges":[],"language":"rust"}"#).unwrap();
    assert_eq!(model.packages, []);
    assert_eq!(model.package_dependencies, []);
    assert_eq!(model.language, "rust");
}

#[test]
fn workspace_facts_round_trip_and_merge_with_source_facts() {
    let mut workspace = CodeModel::new("rust");
    workspace.packages.push(Package {
        id: PackageId("app/Cargo.toml".into()),
        name: "app".into(),
        manifest_path: "app/Cargo.toml".into(),
        targets: vec![PackageTarget {
            name: "app".into(),
            kinds: vec!["bin".into()],
            source_path: "app/src/main.rs".into(),
        }],
    });
    workspace.packages.push(Package {
        id: PackageId("model/Cargo.toml".into()),
        name: "model".into(),
        manifest_path: "model/Cargo.toml".into(),
        targets: vec![],
    });
    workspace.package_dependencies.push(PackageDependency {
        from: PackageId("app/Cargo.toml".into()),
        to: PackageId("model/Cargo.toml".into()),
        kind: PackageDependencyKind::Dev,
        alias: Some("schema".into()),
        optional: false,
        target: Some("cfg(unix)".into()),
    });
    let serialized = serde_json::to_string(&workspace).unwrap();
    let parsed: CodeModel = serde_json::from_str(&serialized).unwrap();
    assert_eq!(parsed, workspace);
    let mut source = CodeModel::new("rust");
    source.modules.push(sample_module());
    source.merge(workspace.clone());
    assert_eq!(source.packages, workspace.packages);
    assert_eq!(source.package_dependencies, workspace.package_dependencies);
    assert_eq!(source.modules, vec![sample_module()]);
    assert_eq!(source.language, "rust");
}

// TODO(Phase 1.a): add tests for the structured Signature once implemented.
// TODO(Phase 1.d): assert merge de-duplicates overlapping symbols.
