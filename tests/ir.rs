//! IR serde round-trips and merge semantics (NEW_ROADMAP Phase 1.a).

use codetwin::ir::{CodeModel, Edge, EdgeKind, Module, ModuleId, Symbol, SymbolKind, Visibility};
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

// TODO(Phase 1.a): add tests for the structured Signature once implemented.
// TODO(Phase 1.d): assert merge de-duplicates overlapping symbols.
