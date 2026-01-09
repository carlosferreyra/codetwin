use super::trait_def::Driver;
/// Tree-sitter logic for Rust
use crate::ir::{
    Blueprint, Class, Documentation, Element, Function, Method, Parameter, Property, Signature,
    Visibility,
};
use std::path::PathBuf;
use tree_sitter::{Node, Parser};

pub struct RustDriver;

impl Driver for RustDriver {
    fn parse(&self, content: &str) -> Result<Blueprint, String> {
        parse_rust_code(content)
    }

    fn generate(&self, _blueprint: &Blueprint) -> Result<String, String> {
        Err(
            "RustDriver::generate: Not implemented yet (Rust is a source, not a target)"
                .to_string(),
        )
    }
}

/// Parse Rust source code and extract classes/functions
fn parse_rust_code(source: &str) -> Result<Blueprint, String> {
    let mut parser = Parser::new();
    // Get the Rust language from tree_sitter_rust
    let language = tree_sitter_rust::LANGUAGE.into();

    parser
        .set_language(&language)
        .map_err(|_| "Failed to set Rust language".to_string())?;

    let tree = parser
        .parse(source, None)
        .ok_or("Failed to parse Rust code")?;

    let mut elements = Vec::new();
    let mut cursor = tree.walk();

    // Walk top-level declarations
    for child in tree.root_node().children(&mut cursor) {
        match child.kind() {
            "struct_item" => {
                if let Ok(class) = extract_struct(&child, source) {
                    elements.push(Element::Class(class));
                }
            }
            "function_item" => {
                if let Ok(function) = extract_function(&child, source) {
                    elements.push(Element::Function(function));
                }
            }
            "impl_item" => {
                // Will be handled separately and attached to classes
                if let Ok(methods) = extract_impl_methods(&child, source) {
                    // Find corresponding class and add methods
                    if let Some(class_name) = get_impl_struct_name(&child, source) {
                        if let Some(Element::Class(class)) = elements.iter_mut().find(|e| {
                            if let Element::Class(c) = e {
                                c.name == class_name
                            } else {
                                false
                            }
                        }) {
                            class.methods.extend(methods);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    Ok(Blueprint {
        source_path: PathBuf::from("unknown.rs"),
        language: "rust".to_string(),
        elements,
    })
}

/// Extract struct definition as Class
fn extract_struct(node: &Node, source: &str) -> Result<Class, String> {
    // The struct name is typically the first identifier child
    let mut name = String::new();
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        if child.kind() == "type_identifier" {
            name = child
                .utf8_text(source.as_bytes())
                .map(|s| s.to_string())
                .unwrap_or_default();
            break;
        }
    }

    if name.is_empty() {
        return Err("Could not find struct name".to_string());
    }

    let visibility = extract_visibility(node, source);

    Ok(Class {
        name,
        visibility,
        methods: Vec::new(),
        properties: extract_struct_fields(node, source),
        documentation: extract_doc_comment(node, source),
    })
}

/// Extract function definition
fn extract_function(node: &Node, source: &str) -> Result<Function, String> {
    let name = get_node_text(node, source, "name")?;
    let visibility = extract_visibility(node, source);
    let signature = extract_function_signature(node, source)?;

    Ok(Function {
        name,
        visibility,
        signature,
        documentation: extract_doc_comment(node, source),
    })
}

/// Extract methods from impl block
fn extract_impl_methods(node: &Node, source: &str) -> Result<Vec<Method>, String> {
    let mut methods = Vec::new();
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        if child.kind() == "function_item" {
            if let Ok(sig) = extract_function_signature(&child, source) {
                let name = get_node_text(&child, source, "name").unwrap_or_default();
                let visibility = extract_visibility(&child, source);

                methods.push(Method {
                    name,
                    visibility,
                    is_static: false, // Can be refined to detect &self vs no self
                    signature: sig,
                    documentation: extract_doc_comment(&child, source),
                });
            }
        }
    }

    Ok(methods)
}

/// Get the struct name from impl block
fn get_impl_struct_name(node: &Node, source: &str) -> Option<String> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "type_identifier" {
            return Some(get_node_text(&child, source, "").unwrap_or_default());
        }
    }
    None
}

/// Extract struct fields as properties
fn extract_struct_fields(node: &Node, source: &str) -> Vec<Property> {
    let mut properties = Vec::new();
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        if child.kind() == "field_declaration" {
            if let Ok(prop) = extract_field(&child, source) {
                properties.push(prop);
            }
        }
    }

    properties
}

/// Extract single field as property
fn extract_field(node: &Node, source: &str) -> Result<Property, String> {
    let name = get_node_text(node, source, "name")?;
    let visibility = extract_visibility(node, source);
    let type_annotation = get_node_text(node, source, "type").ok();

    Ok(Property {
        name,
        visibility,
        type_annotation,
        documentation: extract_doc_comment(node, source),
    })
}

/// Extract function signature (parameters and return type)
fn extract_function_signature(node: &Node, source: &str) -> Result<Signature, String> {
    let mut parameters = Vec::new();
    let mut cursor = node.walk();

    // Find parameters node
    for child in node.children(&mut cursor) {
        if child.kind() == "parameters" {
            parameters = extract_parameters(&child, source);
            break;
        }
    }

    let return_type = extract_return_type(node, source);

    Ok(Signature {
        parameters,
        return_type,
    })
}

/// Extract function parameters
fn extract_parameters(node: &Node, source: &str) -> Vec<Parameter> {
    let mut parameters = Vec::new();
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        if child.kind() == "parameter" {
            if let Ok(param) = extract_parameter(&child, source) {
                parameters.push(param);
            }
        }
    }

    parameters
}

/// Extract single parameter
fn extract_parameter(node: &Node, source: &str) -> Result<Parameter, String> {
    let name = get_node_text(node, source, "pattern")?;
    let type_annotation = get_node_text(node, source, "type").ok();

    Ok(Parameter {
        name,
        type_annotation,
        default_value: None,
    })
}

/// Extract return type from function
fn extract_return_type(node: &Node, source: &str) -> Option<String> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "->" {
            // Next sibling should be the return type
            if let Some(next) = child.next_sibling() {
                return Some(
                    next.utf8_text(source.as_bytes())
                        .unwrap_or("")
                        .trim()
                        .to_string(),
                );
            }
        }
    }
    None
}

/// Extract visibility modifier
fn extract_visibility(node: &Node, source: &str) -> Visibility {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        match child.kind() {
            "pub" => {
                // Check for pub(crate) or pub(super)
                if let Ok(text) = child.utf8_text(source.as_bytes()) {
                    if text.contains("(crate)") {
                        return Visibility::Internal;
                    } else if text.contains("(super)") {
                        return Visibility::Protected;
                    }
                }
                return Visibility::Public;
            }
            _ => {}
        }
    }
    Visibility::Private
}

/// Extract documentation comment above node
fn extract_doc_comment(_node: &Node, _source: &str) -> Documentation {
    // For now, return empty documentation - can be enhanced with comment extraction
    Documentation {
        summary: None,
        description: None,
        examples: Vec::new(),
    }
}

/// Get text content of a specific child node by kind
fn get_node_text(node: &Node, source: &str, target_kind: &str) -> Result<String, String> {
    if target_kind.is_empty() {
        // Return the node itself
        return Ok(node
            .utf8_text(source.as_bytes())
            .map(|s| s.trim().to_string())
            .unwrap_or_default());
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == target_kind {
            return Ok(child
                .utf8_text(source.as_bytes())
                .map(|s| s.trim().to_string())
                .unwrap_or_default());
        }
    }

    Err(format!("Could not find {} in node", target_kind))
}
