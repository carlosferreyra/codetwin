use std::collections::HashMap;
use std::path::PathBuf;

use codetwin::config::Config;
use codetwin::formatters::get_formatter;
use codetwin::ir::{
    Blueprint, Class, Documentation, Element, Method, Parameter, Signature, Visibility,
};

fn sample_blueprint(path: &str) -> Blueprint {
    Blueprint {
        source_path: PathBuf::from(path),
        language: "rust".to_string(),
        elements: vec![Element::Class(Class {
            name: "Widget".to_string(),
            visibility: Visibility::Public,
            methods: vec![Method {
                name: "new".to_string(),
                visibility: Visibility::Public,
                is_static: true,
                signature: Signature {
                    parameters: vec![Parameter {
                        name: "name".to_string(),
                        type_annotation: Some("String".to_string()),
                        default_value: None,
                    }],
                    return_type: Some("Self".to_string()),
                },
                documentation: Documentation {
                    summary: Some("Creates a widget".to_string()),
                    description: None,
                    examples: vec![],
                },
            }],
            properties: vec![],
            documentation: Documentation {
                summary: Some("A test widget".to_string()),
                description: None,
                examples: vec![],
            },
        })],
    }
}

#[test]
fn folder_markdown_formatter_outputs_folder_and_index() {
    let mut config = Config::default();
    config.formatter = "folder_markdown".to_string();

    let formatter = get_formatter(&config).expect("formatter lookup");
    let blueprints = vec![sample_blueprint("src/example.rs")];

    let outputs = formatter.format(&blueprints).expect("format ok");
    let map: HashMap<String, String> = outputs.into_iter().collect();

    assert!(map.contains_key("src.md"), "should emit folder markdown");
    assert!(
        map.contains_key(&config.main_diagram),
        "should emit index diagram"
    );

    let folder_content = map.get("src.md").unwrap();
    assert!(folder_content.contains("# src"));
    assert!(folder_content.contains("classDiagram"));

    let index_content = map.get(&config.main_diagram).unwrap();
    assert!(index_content.contains("Project Architecture"));
}

#[test]
fn one_per_file_formatter_emits_one_file_per_blueprint() {
    let mut config = Config::default();
    config.formatter = "one_per_file".to_string();

    let formatter = get_formatter(&config).expect("formatter lookup");
    let blueprints = vec![
        sample_blueprint("src/foo.rs"),
        sample_blueprint("src/bar.rs"),
    ];

    let outputs = formatter.format(&blueprints).expect("format ok");
    let map: HashMap<String, String> = outputs.into_iter().collect();

    assert!(map.contains_key("foo.md"));
    assert!(map.contains_key("bar.md"));

    for content in map.values() {
        assert!(!content.trim().is_empty(), "content should not be empty");
    }
}
