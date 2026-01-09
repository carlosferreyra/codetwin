use codetwin::drivers::{markdown::MarkdownDriver, trait_def::Driver};
use codetwin::ir::*;
use std::path::PathBuf;

#[test]
fn test_generate_struct_md() {
    // Create a sample Blueprint
    let blueprint = Blueprint {
        source_path: PathBuf::from("src/example.rs"),
        language: "rust".to_string(),
        elements: vec![
            Element::Function(Function {
                name: "greet".to_string(),
                visibility: Visibility::Public,
                signature: Signature {
                    parameters: vec![Parameter {
                        name: "name".to_string(),
                        type_annotation: Some("&str".to_string()),
                        default_value: None,
                    }],
                    return_type: Some("String".to_string()),
                },
                documentation: Documentation {
                    summary: Some("Greets a user by name".to_string()),
                    description: None,
                    examples: vec![],
                },
            }),
            Element::Class(Class {
                name: "User".to_string(),
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
                        summary: Some("Creates a new User instance".to_string()),
                        description: None,
                        examples: vec![],
                    },
                }],
                properties: vec![Property {
                    name: "name".to_string(),
                    visibility: Visibility::Public,
                    type_annotation: Some("String".to_string()),
                    documentation: Documentation {
                        summary: None,
                        description: None,
                        examples: vec![],
                    },
                }],
                documentation: Documentation {
                    summary: Some("Represents a user in the system".to_string()),
                    description: None,
                    examples: vec![],
                },
            }),
        ],
    };

    // Generate markdown
    let driver = MarkdownDriver;
    let result = driver.generate(&blueprint);

    assert!(result.is_ok());
    let markdown = result.unwrap();

    // Verify content
    assert!(markdown.contains("# src"));
    assert!(markdown.contains("Language: **rust**"));
    assert!(markdown.contains("src/example.rs"));
    assert!(markdown.contains("class User"));

    println!("{}", markdown);
}
