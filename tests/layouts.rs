//! Layout registry + MVP layout smoke tests (NEW_ROADMAP Phase 1.c + Phase 2).

use codetwin::config::Config;
use codetwin::ir::CodeModel;
use codetwin::layouts::LayoutRegistry;

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

// TODO(Phase 2.a): assert project-overview output contains the expected sections
//                  (module table, dependency diagram, data-flow narrative).
// TODO(Phase 2.b): assert architecture-map contains circular-dep warnings
//                  when the model has a cycle.
// TODO(Phase 2.d): deterministic-output regression test: same input → same bytes.
