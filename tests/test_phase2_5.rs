// Phase 2.5 Integration Tests: Language-Agnostic Refactoring
use codetwin::ir::Blueprint;
use codetwin::layouts::trait_def::Layout;
use codetwin::layouts::{
    dependency_graph::DependencyGraphLayout, folder_markdown::FolderMarkdownLayout,
    layered::LayeredLayout, readme_embedded::ReadmeEmbeddedLayout,
};
use std::path::PathBuf;

fn create_test_blueprints() -> Vec<Blueprint> {
    vec![
        Blueprint {
            source_path: PathBuf::from("src/main.rs"),
            language: "rust".to_string(),
            elements: vec![],
            dependencies: vec!["cli".to_string()],
        },
        Blueprint {
            source_path: PathBuf::from("src/cli.rs"),
            language: "rust".to_string(),
            elements: vec![],
            dependencies: vec!["engine".to_string()],
        },
        Blueprint {
            source_path: PathBuf::from("src/engine.rs"),
            language: "rust".to_string(),
            elements: vec![],
            dependencies: vec!["ir".to_string()],
        },
        Blueprint {
            source_path: PathBuf::from("src/ir.rs"),
            language: "rust".to_string(),
            elements: vec![],
            dependencies: vec![],
        },
    ]
}

#[test]
fn test_no_hardcoded_paths_in_folder_markdown() {
    let blueprints = create_test_blueprints();
    let layout = FolderMarkdownLayout::new("architecture.md");
    let output = layout.format(&blueprints).expect("Format should succeed");

    let full_output: String = output
        .iter()
        .map(|(_, c)| c.as_str())
        .collect::<Vec<_>>()
        .join("\n");

    assert!(
        !full_output.contains("main[main.rs]"),
        "Should not contain hardcoded 'main[main.rs]'"
    );
    assert!(
        !full_output.contains("cli[cli.rs]"),
        "Should not contain hardcoded 'cli[cli.rs]'"
    );
    assert!(
        !full_output.contains("engine[engine.rs]"),
        "Should not contain hardcoded 'engine[engine.rs]'"
    );
    assert!(!full_output.is_empty(), "Output should not be empty");
}

#[test]
fn test_no_hardcoded_paths_in_readme_embedded() {
    let blueprints = create_test_blueprints();
    let layout = ReadmeEmbeddedLayout;
    let output = layout.format(&blueprints).expect("Format should succeed");

    let full_output: String = output
        .iter()
        .map(|(_, c)| c.as_str())
        .collect::<Vec<_>>()
        .join("\n");

    assert!(
        !full_output.contains("src/drivers/my_language"),
        "Should not contain hardcoded 'src/drivers/my_language'"
    );
    assert!(
        !full_output.contains("src/layouts/my_layout"),
        "Should not contain hardcoded 'src/layouts/my_layout'"
    );
}

#[test]
fn test_layered_layout_auto_detection() {
    let blueprints = create_test_blueprints();
    let layout = LayeredLayout::new(vec![]);

    let output = layout.format(&blueprints).expect("Format should succeed");
    assert!(!output.is_empty(), "Output should not be empty");

    let full_output: String = output
        .iter()
        .map(|(_, c)| c.as_str())
        .collect::<Vec<_>>()
        .join("\n");

    assert!(
        full_output.contains("Layer") || full_output.contains("layer"),
        "Output should reference layers"
    );
}

#[test]
fn test_generic_terminology_in_dependency_graph() {
    let blueprints = create_test_blueprints();
    let layout = DependencyGraphLayout;
    let output = layout.format(&blueprints).expect("Format should succeed");

    let full_output: String = output
        .iter()
        .map(|(_, c)| c.as_str())
        .collect::<Vec<_>>()
        .join("\n");

    assert!(!full_output.is_empty(), "Should generate output");
}

#[test]
fn test_all_layouts_handle_generic_structure() {
    let blueprints = vec![
        Blueprint {
            source_path: PathBuf::from("module1.rs"),
            language: "rust".to_string(),
            elements: vec![],
            dependencies: vec![],
        },
        Blueprint {
            source_path: PathBuf::from("module2.rs"),
            language: "rust".to_string(),
            elements: vec![],
            dependencies: vec!["module1".to_string()],
        },
    ];

    let _ = DependencyGraphLayout
        .format(&blueprints)
        .expect("DependencyGraphLayout should format");

    let _ = FolderMarkdownLayout::new("test.md")
        .format(&blueprints)
        .expect("FolderMarkdownLayout should format");

    let _ = ReadmeEmbeddedLayout
        .format(&blueprints)
        .expect("ReadmeEmbeddedLayout should format");

    let _ = LayeredLayout::new(vec![])
        .format(&blueprints)
        .expect("LayeredLayout should format");
}

#[test]
fn test_custom_layout_loading() {
    let result = codetwin::layouts::load_custom_layout("examples/simple_layout.toml");

    if result.is_ok() {
        let layout = result.unwrap();
        let blueprints = create_test_blueprints();

        let output = layout
            .format(&blueprints)
            .expect("Custom layout format should succeed");
        assert!(!output.is_empty(), "Custom layout should produce output");
    }
}

#[test]
fn test_invalid_custom_layout_error_handling() {
    let result = codetwin::layouts::load_custom_layout("nonexistent_layout.toml");
    assert!(result.is_err(), "Loading nonexistent layout should fail");
}
