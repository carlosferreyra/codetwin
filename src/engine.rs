use crate::drivers::markdown::{generate_file_md, generate_index_md};
/// The "Brain" - SyncEngine and the Loop logic
use crate::ir::*;
use std::fs;
use std::path::PathBuf;

pub struct SyncEngine;

impl SyncEngine {
    pub fn new() -> Self {
        SyncEngine
    }

    pub fn watch(&self) -> Result<(), String> {
        Err("watch: Not implemented yet".to_string())
    }

    pub fn sync(&self) -> Result<(), String> {
        // Generate one diagram per folder + index
        println!("📦 Creating mock Blueprints for demonstration...");

        let blueprints = vec![
            self.create_mock_engine_blueprint(),
            self.create_mock_driver_blueprint(),
            self.create_mock_ir_blueprint(),
        ];

        let output_dir = PathBuf::from("docs");
        println!("📝 Creating docs directory...");
        fs::create_dir_all(&output_dir).map_err(|e| format!("Failed to create docs dir: {}", e))?;

        // Group blueprints by folder
        use std::collections::BTreeMap;
        let mut folder_blueprints: BTreeMap<String, Vec<Blueprint>> = BTreeMap::new();

        for blueprint in blueprints.iter().cloned() {
            let folder = blueprint
                .source_path
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|f| f.to_str())
                .unwrap_or("root")
                .to_string();

            folder_blueprints
                .entry(folder)
                .or_insert_with(Vec::new)
                .push(blueprint);
        }

        // Generate one .md file per folder
        for (folder, folder_bps) in folder_blueprints.iter() {
            let file_name = format!("{}.md", folder);
            let file_path = output_dir.join(&file_name);

            println!("  → Generating {}", file_name);
            let markdown = generate_file_md(&folder_bps)
                .map_err(|e| format!("Failed to generate markdown: {}", e))?;

            fs::write(&file_path, markdown)
                .map_err(|e| format!("Failed to write {}: {}", file_path.display(), e))?;
        }

        // Generate index with folder dependency graph
        let folders: Vec<&String> = folder_blueprints.keys().collect();
        println!("📝 Generating index (STRUCT.md)...");
        let index = generate_index_md(&folders.iter().map(|s| s.as_str()).collect::<Vec<_>>())
            .map_err(|e| format!("Failed to generate index: {}", e))?;

        let index_path = output_dir.join("STRUCT.md");
        fs::write(&index_path, index).map_err(|e| format!("Failed to write index: {}", e))?;

        println!(
            "✅ Successfully generated documentation in {}",
            output_dir.display()
        );
        println!("   Files created:");
        println!("   - {}", index_path.display());
        for folder in folder_blueprints.keys() {
            println!(
                "   - {}",
                output_dir.join(format!("{}.md", folder)).display()
            );
        }

        Ok(())
    }

    pub fn check(&self) -> Result<(), String> {
        Err("check: Not implemented yet".to_string())
    }

    pub fn init(&self) -> Result<(), String> {
        Err("init: Not implemented yet".to_string())
    }

    pub fn list(&self) -> Result<(), String> {
        Err("list: Not implemented yet".to_string())
    }

    // Mock Blueprints for demonstration
    fn create_mock_engine_blueprint(&self) -> Blueprint {
        Blueprint {
            source_path: PathBuf::from("src/engine.rs"),
            language: "rust".to_string(),
            elements: vec![Element::Class(Class {
                name: "SyncEngine".to_string(),
                visibility: Visibility::Public,
                methods: vec![
                    Method {
                        name: "new".to_string(),
                        visibility: Visibility::Public,
                        is_static: true,
                        signature: Signature {
                            parameters: vec![],
                            return_type: Some("Self".to_string()),
                        },
                        documentation: Documentation {
                            summary: Some("Creates a new SyncEngine instance".to_string()),
                            description: None,
                            examples: vec![],
                        },
                    },
                    Method {
                        name: "sync".to_string(),
                        visibility: Visibility::Public,
                        is_static: false,
                        signature: Signature {
                            parameters: vec![],
                            return_type: Some("Result<(), String>".to_string()),
                        },
                        documentation: Documentation {
                            summary: Some("Generates documentation from source".to_string()),
                            description: None,
                            examples: vec![],
                        },
                    },
                ],
                properties: vec![],
                documentation: Documentation {
                    summary: Some("Core synchronization engine".to_string()),
                    description: None,
                    examples: vec![],
                },
            })],
        }
    }

    fn create_mock_driver_blueprint(&self) -> Blueprint {
        Blueprint {
            source_path: PathBuf::from("src/drivers/mod.rs"),
            language: "rust".to_string(),
            elements: vec![Element::Class(Class {
                name: "Driver".to_string(),
                visibility: Visibility::Public,
                methods: vec![Method {
                    name: "parse".to_string(),
                    visibility: Visibility::Public,
                    is_static: false,
                    signature: Signature {
                        parameters: vec![Parameter {
                            name: "content".to_string(),
                            type_annotation: Some("&str".to_string()),
                            default_value: None,
                        }],
                        return_type: Some("Result<Blueprint, String>".to_string()),
                    },
                    documentation: Documentation {
                        summary: Some("Parses source code into Blueprint".to_string()),
                        description: None,
                        examples: vec![],
                    },
                }],
                properties: vec![],
                documentation: Documentation {
                    summary: Some("Adapter trait for language-specific parsing".to_string()),
                    description: None,
                    examples: vec![],
                },
            })],
        }
    }

    fn create_mock_ir_blueprint(&self) -> Blueprint {
        Blueprint {
            source_path: PathBuf::from("src/ir.rs"),
            language: "rust".to_string(),
            elements: vec![Element::Class(Class {
                name: "Blueprint".to_string(),
                visibility: Visibility::Public,
                methods: vec![],
                properties: vec![
                    Property {
                        name: "source_path".to_string(),
                        visibility: Visibility::Public,
                        type_annotation: Some("PathBuf".to_string()),
                        documentation: Documentation {
                            summary: None,
                            description: None,
                            examples: vec![],
                        },
                    },
                    Property {
                        name: "language".to_string(),
                        visibility: Visibility::Public,
                        type_annotation: Some("String".to_string()),
                        documentation: Documentation {
                            summary: None,
                            description: None,
                            examples: vec![],
                        },
                    },
                    Property {
                        name: "elements".to_string(),
                        visibility: Visibility::Public,
                        type_annotation: Some("Vec<Element>".to_string()),
                        documentation: Documentation {
                            summary: None,
                            description: None,
                            examples: vec![],
                        },
                    },
                ],
                documentation: Documentation {
                    summary: Some("Universal intermediate representation".to_string()),
                    description: None,
                    examples: vec![],
                },
            })],
        }
    }
}
