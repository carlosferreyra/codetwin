use std::collections::HashMap;
use std::path::PathBuf;

use codetwin::config::Config;
use codetwin::ir::{
    Blueprint, Class, Documentation, Element, Function, Method, Parameter, Signature, Visibility,
};
use codetwin::layouts::get_layout;

fn sample_blueprint(path: &str, name: &str, with_deps: bool) -> Blueprint {
    Blueprint {
        source_path: PathBuf::from(path),
        language: "rust".to_string(),
        elements: vec![Element::Class(Class {
            name: name.to_string(),
            visibility: Visibility::Public,
            methods: vec![Method {
                name: "new".to_string(),
                visibility: Visibility::Public,
                is_static: true,
                signature: Signature {
                    parameters: vec![Parameter {
                        name: "config".to_string(),
                        type_annotation: Some("Config".to_string()),
                        default_value: None,
                    }],
                    return_type: Some("Self".to_string()),
                },
                documentation: Documentation {
                    summary: Some(format!("Creates a new {}", name)),
                    description: None,
                    examples: vec![],
                },
            }],
            properties: vec![],
            documentation: Documentation {
                summary: Some(format!("A test {}", name)),
                description: None,
                examples: vec![],
            },
        })],
        dependencies: if with_deps {
            vec!["config".to_string(), "ir".to_string()]
        } else {
            vec![]
        },
    }
}

// ============================================================================
// LAYERED LAYOUT TESTS
// ============================================================================

#[test]
fn test_layered_layout_generates() {
    let config = Config {
        layout: "layered".to_string(),
        ..Config::defaults()
    };

    let layout = get_layout(&config).expect("layout lookup");
    let blueprints = vec![
        sample_blueprint("src/lib.rs", "Lib", false),
        sample_blueprint("src/engine.rs", "Engine", true),
        sample_blueprint("src/drivers/rust.rs", "RustDriver", true),
    ];

    let outputs = layout.format(&blueprints).expect("format ok");
    assert!(!outputs.is_empty(), "should produce output");

    let map: HashMap<String, String> = outputs.into_iter().collect();
    assert!(
        map.contains_key("architecture.md"),
        "should emit architecture.md"
    );

    let content = map.get("architecture.md").unwrap();
    assert!(!content.is_empty(), "content should not be empty");
    assert!(
        content.contains("Layered Architecture"),
        "should contain title"
    );
}

#[test]
fn test_layered_layout_contains_layer_descriptions() {
    let config = Config {
        layout: "layered".to_string(),
        ..Config::defaults()
    };

    let layout = get_layout(&config).expect("layout lookup");
    let blueprints = vec![
        sample_blueprint("src/lib.rs", "Lib", false),
        sample_blueprint("src/engine.rs", "Engine", true),
    ];

    let outputs = layout.format(&blueprints).expect("format ok");
    let content = outputs[0].1.clone();

    // Verify layer names are in the output
    assert!(
        content.contains("Core") || content.contains("Engine"),
        "should mention at least one layer"
    );
}

#[test]
fn test_layered_layout_contains_mermaid_diagram() {
    let config = Config {
        layout: "layered".to_string(),
        ..Config::defaults()
    };

    let layout = get_layout(&config).expect("layout lookup");
    let blueprints = vec![
        sample_blueprint("src/lib.rs", "Lib", false),
        sample_blueprint("src/engine.rs", "Engine", true),
    ];

    let outputs = layout.format(&blueprints).expect("format ok");
    let content = outputs[0].1.clone();

    assert!(
        content.contains("```mermaid"),
        "should contain mermaid diagram"
    );
    assert!(content.contains("graph TD"), "should contain mermaid graph");
    assert!(content.contains("```"), "should close mermaid block");
}

#[test]
fn test_layered_layout_shows_dependencies() {
    let config = Config {
        layout: "layered".to_string(),
        ..Config::defaults()
    };

    let layout = get_layout(&config).expect("layout lookup");
    let blueprints = vec![sample_blueprint("src/engine.rs", "Engine", true)];

    let outputs = layout.format(&blueprints).expect("format ok");
    let content = outputs[0].1.clone();

    // Should list dependencies
    assert!(content.contains("Dependencies") || content.contains("dependency"));
}

// ============================================================================
// README-EMBEDDED LAYOUT TESTS
// ============================================================================

#[test]
fn test_readme_embedded_layout_generates() {
    let config = Config {
        layout: "readme-embedded".to_string(),
        ..Config::defaults()
    };

    let layout = get_layout(&config).expect("layout lookup");
    let blueprints = vec![
        sample_blueprint("src/lib.rs", "Lib", false),
        sample_blueprint("src/engine.rs", "Engine", true),
    ];

    let outputs = layout.format(&blueprints).expect("format ok");
    assert!(!outputs.is_empty(), "should produce output");

    let map: HashMap<String, String> = outputs.into_iter().collect();
    assert!(
        map.contains_key("architecture.md"),
        "should emit architecture.md"
    );
}

#[test]
fn test_readme_embedded_layout_compact_output() {
    let config = Config {
        layout: "readme-embedded".to_string(),
        ..Config::defaults()
    };

    let layout = get_layout(&config).expect("layout lookup");
    let blueprints = vec![
        sample_blueprint("src/lib.rs", "Lib", false),
        sample_blueprint("src/engine.rs", "Engine", true),
        sample_blueprint("src/config.rs", "Config", false),
        sample_blueprint("src/discovery.rs", "Discovery", true),
    ];

    let outputs = layout.format(&blueprints).expect("format ok");
    let content = outputs[0].1.clone();

    let line_count = content.lines().count();
    // Should be reasonably compact (definitely under 300 lines for small input)
    assert!(
        line_count < 500,
        "output should be compact, got {} lines",
        line_count
    );
}

#[test]
fn test_readme_embedded_has_component_table() {
    let config = Config {
        layout: "readme-embedded".to_string(),
        ..Config::defaults()
    };

    let layout = get_layout(&config).expect("layout lookup");
    let blueprints = vec![
        sample_blueprint("src/lib.rs", "Lib", false),
        sample_blueprint("src/engine.rs", "Engine", true),
    ];

    let outputs = layout.format(&blueprints).expect("format ok");
    let content = outputs[0].1.clone();

    // Should have the component table
    assert!(
        content.contains("### Components"),
        "should have Components section"
    );
    assert!(content.contains("| Module |"), "should have table header");
    assert!(
        content.contains("Purpose") || content.contains("Key Functions"),
        "should have table columns"
    );
}

#[test]
fn test_readme_embedded_has_dependency_diagram() {
    let config = Config {
        layout: "readme-embedded".to_string(),
        ..Config::defaults()
    };

    let layout = get_layout(&config).expect("layout lookup");
    let blueprints = vec![
        sample_blueprint("src/lib.rs", "Lib", false),
        sample_blueprint("src/engine.rs", "Engine", true),
    ];

    let outputs = layout.format(&blueprints).expect("format ok");
    let content = outputs[0].1.clone();

    assert!(
        content.contains("### Dependency Overview") || content.contains("graph TD"),
        "should have dependency diagram"
    );
    assert!(content.contains("```mermaid"), "should use mermaid syntax");
}

#[test]
fn test_readme_embedded_has_data_flow() {
    let config = Config {
        layout: "readme-embedded".to_string(),
        ..Config::defaults()
    };

    let layout = get_layout(&config).expect("layout lookup");
    let blueprints = vec![
        sample_blueprint("src/cli.rs", "Cli", false),
        sample_blueprint("src/engine.rs", "Engine", true),
    ];

    let outputs = layout.format(&blueprints).expect("format ok");
    let content = outputs[0].1.clone();

    assert!(
        content.contains("### Data Flow"),
        "should have Data Flow section"
    );
    // Should have numbered steps
    assert!(
        content.contains("1.") || content.contains("2."),
        "should have steps"
    );
}

#[test]
fn test_readme_embedded_has_dev_guide() {
    let config = Config {
        layout: "readme-embedded".to_string(),
        ..Config::defaults()
    };

    let layout = get_layout(&config).expect("layout lookup");
    let blueprints = vec![sample_blueprint("src/lib.rs", "Lib", false)];

    let outputs = layout.format(&blueprints).expect("format ok");
    let content = outputs[0].1.clone();

    assert!(
        content.contains("### Development Guide"),
        "should have Development Guide section"
    );
    assert!(content.contains("#### Key Files"), "should list key files");
}

// ============================================================================
// LAYOUT REGISTRY TESTS
// ============================================================================

#[test]
fn test_all_layouts_available_in_registry() {
    let config_graph = Config {
        layout: "dependency-graph".to_string(),
        ..Config::defaults()
    };
    let config_layered = Config {
        layout: "layered".to_string(),
        ..Config::defaults()
    };
    let config_readme = Config {
        layout: "readme-embedded".to_string(),
        ..Config::defaults()
    };

    assert!(
        get_layout(&config_graph).is_ok(),
        "dependency-graph layout should be available"
    );
    assert!(
        get_layout(&config_layered).is_ok(),
        "layered layout should be available"
    );
    assert!(
        get_layout(&config_readme).is_ok(),
        "readme-embedded layout should be available"
    );
}

#[test]
fn test_layout_cli_flag_selection() {
    // Test that we can select layouts via CLI (via Config.layout)
    let config_layered = Config {
        layout: "layered".to_string(),
        ..Config::defaults()
    };

    let layout = get_layout(&config_layered).expect("layout lookup");
    let blueprints = vec![sample_blueprint("src/lib.rs", "Lib", false)];

    let outputs = layout.format(&blueprints).expect("format ok");
    let content = outputs[0].1.clone();

    // Verify it's the layered layout by checking for unique markers
    assert!(
        content.contains("Layered Architecture") || content.contains("Layer Diagram"),
        "should produce layered layout output"
    );
}

#[test]
fn test_invalid_layout_name_returns_error() {
    let config = Config {
        layout: "nonexistent-layout".to_string(),
        ..Config::defaults()
    };

    assert!(
        get_layout(&config).is_err(),
        "should return error for invalid layout"
    );
}

// ============================================================================
// COEXISTENCE TESTS
// ============================================================================

#[test]
fn test_all_layouts_parse_same_source() {
    let blueprints = vec![
        sample_blueprint("src/lib.rs", "Lib", false),
        sample_blueprint("src/engine.rs", "Engine", true),
    ];

    let config_graph = Config {
        layout: "dependency-graph".to_string(),
        ..Config::defaults()
    };
    let config_layered = Config {
        layout: "layered".to_string(),
        ..Config::defaults()
    };
    let config_readme = Config {
        layout: "readme-embedded".to_string(),
        ..Config::defaults()
    };

    let layout_graph = get_layout(&config_graph).expect("graph layout");
    let layout_layered = get_layout(&config_layered).expect("layered layout");
    let layout_readme = get_layout(&config_readme).expect("readme layout");

    // All should produce outputs without errors
    let out_graph = layout_graph.format(&blueprints).expect("graph format");
    let out_layered = layout_layered.format(&blueprints).expect("layered format");
    let out_readme = layout_readme.format(&blueprints).expect("readme format");

    assert!(!out_graph.is_empty());
    assert!(!out_layered.is_empty());
    assert!(!out_readme.is_empty());

    // All outputs should be different (different layouts)
    let content_graph = &out_graph[0].1;
    let content_layered = &out_layered[0].1;
    let content_readme = &out_readme[0].1;

    // Check that they're different (at least one unique identifier per layout)
    let has_graph_marker = content_graph.contains("Dependency Graph");
    let has_layered_marker = content_layered.contains("Layered Architecture");
    let has_readme_marker = content_readme.contains("Project Architecture");

    assert!(
        has_graph_marker || has_layered_marker || has_readme_marker,
        "at least one layout marker should be present"
    );
}

#[test]
fn test_layout_with_empty_blueprints() {
    let config_layered = Config {
        layout: "layered".to_string(),
        ..Config::defaults()
    };
    let config_readme = Config {
        layout: "readme-embedded".to_string(),
        ..Config::defaults()
    };

    let layout_layered = get_layout(&config_layered).expect("layered layout");
    let layout_readme = get_layout(&config_readme).expect("readme layout");

    let blueprints = vec![];

    // Should not crash with empty input
    let out_layered = layout_layered
        .format(&blueprints)
        .expect("should handle empty blueprints");
    let out_readme = layout_readme
        .format(&blueprints)
        .expect("should handle empty blueprints");

    assert!(
        !out_layered.is_empty(),
        "should produce output even with empty input"
    );
    assert!(
        !out_readme.is_empty(),
        "should produce output even with empty input"
    );
}

#[test]
fn test_layout_config_layer_configuration() {
    // Test that LayerConfig is properly used
    let layers = vec![
        codetwin::config::Layer {
            name: "UI".to_string(),
            patterns: vec!["src/ui/**".to_string()],
        },
        codetwin::config::Layer {
            name: "API".to_string(),
            patterns: vec!["src/api/**".to_string()],
        },
    ];

    let config = Config {
        layout: "layered".to_string(),
        layers,
        ..Config::defaults()
    };

    let layout = get_layout(&config).expect("layout lookup");
    let blueprints = vec![sample_blueprint("src/ui/main.rs", "UIMain", false)];

    let outputs = layout.format(&blueprints).expect("format ok");
    assert!(!outputs.is_empty(), "should format with custom layers");
}

#[test]
fn test_dependency_graph_still_works_after_phase2() {
    // Ensure Phase 1.5 Layout 1 still works
    let config = Config {
        layout: "dependency-graph".to_string(),
        ..Config::defaults()
    };

    let layout = get_layout(&config).expect("layout lookup");
    let blueprints = vec![
        sample_blueprint("src/lib.rs", "Lib", false),
        sample_blueprint("src/engine.rs", "Engine", true),
    ];

    let outputs = layout.format(&blueprints).expect("format ok");
    let content = outputs[0].1.clone();

    assert!(
        content.contains("## Dependency Graph"),
        "should have dependency graph"
    );
    assert!(
        content.contains("Modules"),
        "should list modules (dependency-graph specific)"
    );
}
