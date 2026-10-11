//! Layout registry + MVP layout smoke tests (`NEW_ROADMAP` Phase 1.c + Phase 2).

use codetwin_legacy::config::Config;
use codetwin_legacy::ir::{CodeModel, Edge, EdgeKind, ModuleId};
use codetwin_legacy::layouts::LayoutRegistry;
use codetwin_legacy::render::mermaid::graph_td;

#[test]
fn default_registry_lists_all_layouts() {
    let names = LayoutRegistry::default().names();
    assert!(names.contains(&"project-overview"));
    assert!(names.contains(&"architecture-map"));
    assert!(names.contains(&"c4"));
    assert!(names.contains(&"metrics"));
}

#[test]
fn project_overview_produces_a_single_markdown_file() {
    let registry = LayoutRegistry::default();
    let layout = registry.get("project-overview").expect("registered");

    let model = CodeModel::new("rust");
    let config = Config::default();

    let out = layout.render(&model, &config).unwrap();
    assert_eq!(out.len(), 1);
    assert!(out[0].content.contains("# Project Overview"));
}

#[test]
fn architecture_map_produces_a_single_markdown_file() {
    let registry = LayoutRegistry::default();
    let layout = registry.get("architecture-map").expect("registered");

    let model = CodeModel::new("rust");
    let config = Config::default();

    let out = layout.render(&model, &config).unwrap();
    assert_eq!(out.len(), 1);
    assert!(out[0].content.contains("# Architecture Map"));
}

#[test]
fn c4_layout_is_not_implemented_yet() {
    let registry = LayoutRegistry::default();
    let layout = registry.get("c4").expect("registered");

    let err = layout
        .render(&CodeModel::default(), &Config::default())
        .unwrap_err();
    assert!(format!("{err:#}").contains("not implemented"));
}

#[test]
fn mermaid_preserves_relationship_styles_and_stable_order() {
    let mut model = CodeModel::new("rust");
    for kind in [
        EdgeKind::Import,
        EdgeKind::Uses,
        EdgeKind::Implements,
        EdgeKind::Extends,
        EdgeKind::Calls,
    ] {
        model.edges.push(Edge {
            from: ModuleId::from("crate::cli"),
            to: ModuleId::from("crate::pipeline"),
            kind,
        });
    }
    let output = graph_td(&model);
    assert_eq!(
        output,
        "graph TD\n  crate__cli --> crate__pipeline\n  crate__cli -.-> crate__pipeline\n  crate__cli ==> crate__pipeline\n  crate__cli ==> crate__pipeline\n  crate__cli --> crate__pipeline\n"
    );
    assert_eq!(output, graph_td(&model));
}

// TODO(Phase 2.a): assert project-overview output contains the expected sections
//                  (module table, dependency diagram, data-flow narrative).
// TODO(Phase 2.b): assert architecture-map contains circular-dep warnings
//                  when the model has a cycle.
// TODO(Phase 2.d): deterministic-output regression test: same input → same bytes.
