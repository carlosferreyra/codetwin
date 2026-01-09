/// Markdown generator - Blueprint → STRUCT.md

use crate::ir::{Blueprint, Element, Visibility};
use super::trait_def::Driver;

pub struct MarkdownDriver;

impl Driver for MarkdownDriver {
    fn parse(&self, _content: &str) -> Result<Blueprint, String> {
        Err("MarkdownDriver::parse: Not implemented yet (Markdown is a target, not a source for now)".to_string())
    }

    fn generate(&self, blueprint: &Blueprint) -> Result<String, String> {
        let mut output = String::new();

        // Header
        output.push_str(&format!("# {}\n\n",
            blueprint.source_path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Unknown")
        ));

        output.push_str(&format!("> Language: **{}**\n\n", blueprint.language));
        output.push_str(&format!("> Source: `{}`\n\n", blueprint.source_path.display()));

        output.push_str("---\n\n");

        // Mermaid Class Diagram
        if let Some(mermaid) = generate_mermaid_diagram(blueprint) {
            output.push_str("## Class Diagram\n\n");
            output.push_str("```mermaid\n");
            output.push_str(&mermaid);
            output.push_str("```\n\n");
            output.push_str("---\n\n");
        }

        // Elements
        for element in &blueprint.elements {
            match element {
                Element::Module(module) => {
                    output.push_str(&format!("## Module: {}\n\n", module.name));
                    if let Some(summary) = &module.documentation.summary {
                        output.push_str(&format!("{}\n\n", summary));
                    }
                    if let Some(desc) = &module.documentation.description {
                        output.push_str(&format!("{}\n\n", desc));
                    }
                }
                Element::Class(class) => {
                    output.push_str(&format!("## Class: `{}`\n\n", class.name));
                    output.push_str(&format!("**Visibility:** {}\n\n", format_visibility(&class.visibility)));

                    if let Some(summary) = &class.documentation.summary {
                        output.push_str(&format!("{}\n\n", summary));
                    }

                    // Properties
                    if !class.properties.is_empty() {
                        output.push_str("### Properties\n\n");
                        for prop in &class.properties {
                            let type_str = prop.type_annotation.as_deref().unwrap_or("unknown");
                            output.push_str(&format!("- `{}`: {} ({})\n",
                                prop.name, type_str, format_visibility(&prop.visibility)));
                        }
                        output.push_str("\n");
                    }

                    // Methods
                    if !class.methods.is_empty() {
                        output.push_str("### Methods\n\n");
                        for method in &class.methods {
                            let params: Vec<String> = method.signature.parameters.iter()
                                .map(|p| {
                                    let type_str = p.type_annotation.as_deref().unwrap_or("_");
                                    format!("{}: {}", p.name, type_str)
                                })
                                .collect();
                            let return_str = method.signature.return_type.as_deref().unwrap_or("void");

                            output.push_str(&format!("#### `{}({})`\n\n", method.name, params.join(", ")));
                            output.push_str(&format!("**Returns:** `{}`\n\n", return_str));
                            output.push_str(&format!("**Visibility:** {}\n\n", format_visibility(&method.visibility)));

                            if let Some(summary) = &method.documentation.summary {
                                output.push_str(&format!("{}\n\n", summary));
                            }
                        }
                    }
                }
                Element::Function(func) => {
                    let params: Vec<String> = func.signature.parameters.iter()
                        .map(|p| {
                            let type_str = p.type_annotation.as_deref().unwrap_or("_");
                            format!("{}: {}", p.name, type_str)
                        })
                        .collect();
                    let return_str = func.signature.return_type.as_deref().unwrap_or("void");

                    output.push_str(&format!("## Function: `{}({})`\n\n", func.name, params.join(", ")));
                    output.push_str(&format!("**Returns:** `{}`\n\n", return_str));
                    output.push_str(&format!("**Visibility:** {}\n\n", format_visibility(&func.visibility)));

                    if let Some(summary) = &func.documentation.summary {
                        output.push_str(&format!("{}\n\n", summary));
                    }
                }
            }

            output.push_str("---\n\n");
        }

        Ok(output)
    }
}

fn format_visibility(vis: &Visibility) -> &'static str {
    match vis {
        Visibility::Public => "public",
        Visibility::Private => "private",
        Visibility::Protected => "protected",
        Visibility::Internal => "internal",
    }
}

fn generate_mermaid_diagram(blueprint: &Blueprint) -> Option<String> {
    // Only generate if there are classes or functions
    if blueprint.elements.is_empty() {
        return None;
    }

    let mut diagram = String::from("classDiagram\n");

    for element in &blueprint.elements {
        match element {
            Element::Class(class) => {
                diagram.push_str(&format!("    class {} {{\n", class.name));

                // Properties
                for prop in &class.properties {
                    let visibility_symbol = mermaid_visibility(&prop.visibility);
                    let type_str = prop.type_annotation.as_deref().unwrap_or("_");
                    diagram.push_str(&format!("        {}{} {}\n", visibility_symbol, prop.name, type_str));
                }

                // Methods
                for method in &class.methods {
                    let visibility_symbol = mermaid_visibility(&method.visibility);
                    let params: Vec<String> = method.signature.parameters.iter()
                        .filter(|p| p.name != "self")
                        .map(|p| {
                            let type_str = p.type_annotation.as_deref().unwrap_or("_");
                            format!("{}: {}", p.name, type_str)
                        })
                        .collect();
                    let return_str = method.signature.return_type.as_deref().unwrap_or("void");
                    
                    diagram.push_str(&format!("        {}{}({}) {}\n", 
                        visibility_symbol, 
                        method.name, 
                        params.join(", "),
                        return_str
                    ));
                }

                diagram.push_str("    }\n");
            }
            Element::Function(_) => {
                // Functions could be represented as utility classes or module-level
                // For now, skip standalone functions in class diagram
            }
            Element::Module(_) => {
                // Modules typically aren't shown in class diagrams
            }
        }
    }

    Some(diagram)
}

fn mermaid_visibility(vis: &Visibility) -> &'static str {
    match vis {
        Visibility::Public => "+",
        Visibility::Private => "-",
        Visibility::Protected => "#",
        Visibility::Internal => "~",
    }
}
