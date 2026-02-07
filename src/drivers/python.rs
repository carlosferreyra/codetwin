use super::trait_def::Driver;
use crate::core::ir::{
    Blueprint, Class, Documentation, Element, Function, Method, Parameter, Property, Signature,
    Visibility,
};
use crate::drivers::LanguageTerminology;
use anyhow::{Context, Result, anyhow};
use std::path::PathBuf;
use tracing::debug;
use tree_sitter::{Node, Parser};

pub struct PythonDriver;

impl Default for PythonDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl PythonDriver {
    #[allow(dead_code)]
    pub fn new() -> Self {
        PythonDriver
    }
}

impl Driver for PythonDriver {
    fn parse(&self, content: &str) -> Result<Blueprint> {
        parse_python_code(content)
    }

    fn generate(&self, _blueprint: &Blueprint) -> Result<String> {
        Err(anyhow!(
            "PythonDriver::generate: Not implemented yet (Python is a source, not a target)"
        ))
    }

    fn terminology(&self) -> LanguageTerminology {
        LanguageTerminology::python()
    }
}

fn parse_python_code(source: &str) -> Result<Blueprint> {
    let mut parser = Parser::new();
    let language = tree_sitter_python::language();

    parser
        .set_language(language)
        .context("Failed to set Python language")?;

    let tree = parser
        .parse(source, None)
        .ok_or_else(|| anyhow!("Failed to parse Python code"))?;

    let root = tree.root_node();
    let mut elements = Vec::new();
    let mut dependencies = Vec::new();
    let mut cursor = root.walk();

    debug!("Parsing Python source (root kind: {})", root.kind());

    for child in root.named_children(&mut cursor) {
        match child.kind() {
            "import_statement" => {
                dependencies.extend(extract_imports(&child, source));
            }
            "import_from_statement" => {
                dependencies.extend(extract_import_from(&child, source));
            }
            "class_definition" => {
                let (class, deps) = extract_class(&child, source, &[])?;
                elements.push(Element::Class(class));
                dependencies.extend(deps);
            }
            "function_definition" => {
                let function = extract_function(&child, source, &[])?;
                elements.push(Element::Function(function));
            }
            "decorated_definition" => {
                handle_decorated_definition(&child, source, &mut elements, &mut dependencies)?;
            }
            _ => {}
        }
    }

    dependencies.sort();
    dependencies.dedup();

    Ok(Blueprint {
        source_path: PathBuf::from("unknown.py"),
        language: "python".to_string(),
        elements,
        dependencies,
    })
}

fn handle_decorated_definition(
    node: &Node,
    source: &str,
    elements: &mut Vec<Element>,
    dependencies: &mut Vec<String>,
) -> Result<()> {
    let decorators = extract_decorators(node, source);
    if let Some(definition) = find_decorated_definition(node) {
        match definition.kind() {
            "class_definition" => {
                let (class, deps) = extract_class(&definition, source, &decorators)?;
                elements.push(Element::Class(class));
                dependencies.extend(deps);
            }
            "function_definition" => {
                let function = extract_function(&definition, source, &decorators)?;
                elements.push(Element::Function(function));
            }
            _ => {}
        }
    }

    Ok(())
}

fn find_decorated_definition<'a>(node: &'a Node<'a>) -> Option<Node<'a>> {
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        if matches!(child.kind(), "class_definition" | "function_definition") {
            return Some(child);
        }
    }
    None
}

fn extract_class(node: &Node, source: &str, decorators: &[String]) -> Result<(Class, Vec<String>)> {
    let name =
        get_child_text(node, source, "name").or_else(|_| get_first_identifier(node, source))?;
    let visibility = visibility_from_name(&name);
    let mut dependencies = extract_superclasses(node, source);

    let body = node
        .child_by_field_name("body")
        .ok_or_else(|| anyhow!("Missing class body"))?;

    let documentation = extract_docstring_from_block(&body, source);
    let (methods, properties, class_deps) = extract_class_body(&body, source)?;

    dependencies.extend(class_deps);

    debug!("Parsed class {} (decorators: {:?})", name, decorators);

    Ok((
        Class {
            name,
            visibility,
            methods,
            properties,
            documentation,
        },
        dependencies,
    ))
}

fn extract_function(node: &Node, source: &str, decorators: &[String]) -> Result<Function> {
    let name =
        get_child_text(node, source, "name").or_else(|_| get_first_identifier(node, source))?;
    let visibility = visibility_from_name(&name);
    let signature = extract_function_signature(node, source);
    let documentation = node
        .child_by_field_name("body")
        .map(|body| extract_docstring_from_block(&body, source))
        .unwrap_or_else(empty_documentation);

    debug!("Parsed function {} (decorators: {:?})", name, decorators);

    Ok(Function {
        name,
        visibility,
        signature,
        documentation,
    })
}

fn extract_class_body(
    body: &Node,
    source: &str,
) -> Result<(Vec<Method>, Vec<Property>, Vec<String>)> {
    let mut methods = Vec::new();
    let mut properties = Vec::new();
    let mut dependencies = Vec::new();

    let mut cursor = body.walk();
    for child in body.named_children(&mut cursor) {
        match child.kind() {
            "function_definition" => {
                let method = extract_method(&child, source, &[])?;
                methods.push(method);
            }
            "decorated_definition" => {
                let decorators = extract_decorators(&child, source);
                if let Some(definition) = find_decorated_definition(&child)
                    && definition.kind() == "function_definition"
                {
                    if decorators.iter().any(|d| d == "property") {
                        if let Some(prop) = property_from_accessor(&definition, source) {
                            properties.push(prop);
                        }
                    } else {
                        let method = extract_method(&definition, source, &decorators)?;
                        methods.push(method);
                    }
                }
            }
            "assignment" => {
                if let Some(prop) = extract_assignment_property(&child, source) {
                    properties.push(prop);
                }
            }
            "expression_statement" => {
                if let Some(assign) = find_child_kind(&child, "assignment")
                    && let Some(prop) = extract_assignment_property(&assign, source)
                {
                    properties.push(prop);
                }
            }
            _ => {}
        }
    }

    dependencies.sort();
    dependencies.dedup();

    Ok((methods, properties, dependencies))
}

fn extract_method(node: &Node, source: &str, decorators: &[String]) -> Result<Method> {
    let name =
        get_child_text(node, source, "name").or_else(|_| get_first_identifier(node, source))?;
    let visibility = visibility_from_name(&name);
    let signature = extract_function_signature(node, source);
    let documentation = node
        .child_by_field_name("body")
        .map(|body| extract_docstring_from_block(&body, source))
        .unwrap_or_else(empty_documentation);
    let is_static = decorators
        .iter()
        .any(|d| d == "staticmethod" || d == "classmethod");

    Ok(Method {
        name,
        visibility,
        is_static,
        signature,
        documentation,
    })
}

fn property_from_accessor(node: &Node, source: &str) -> Option<Property> {
    let name = get_child_text(node, source, "name")
        .or_else(|_| get_first_identifier(node, source))
        .ok()?;
    let visibility = visibility_from_name(&name);
    let return_type = extract_return_type(node, source);
    let documentation = node
        .child_by_field_name("body")
        .map(|body| extract_docstring_from_block(&body, source))
        .unwrap_or_else(empty_documentation);

    Some(Property {
        name,
        visibility,
        type_annotation: return_type,
        documentation,
    })
}

fn extract_assignment_property(node: &Node, source: &str) -> Option<Property> {
    let name = node
        .child_by_field_name("left")
        .and_then(|left| get_first_identifier(&left, source).ok())
        .or_else(|| get_first_identifier(node, source).ok())?;
    let visibility = visibility_from_name(&name);
    let type_annotation = node
        .child_by_field_name("type")
        .and_then(|t| t.utf8_text(source.as_bytes()).ok())
        .map(|s| s.trim().to_string());

    Some(Property {
        name,
        visibility,
        type_annotation,
        documentation: empty_documentation(),
    })
}

fn extract_function_signature(node: &Node, source: &str) -> Signature {
    let mut parameters = Vec::new();
    let mut cursor = node.walk();

    for child in node.named_children(&mut cursor) {
        if child.kind() == "parameters" {
            parameters = extract_parameters(&child, source);
            break;
        }
    }

    Signature {
        parameters,
        return_type: extract_return_type(node, source),
    }
}

fn extract_parameters(node: &Node, source: &str) -> Vec<Parameter> {
    let mut parameters = Vec::new();
    let mut cursor = node.walk();

    for child in node.named_children(&mut cursor) {
        if let Some(param) = extract_parameter(&child, source) {
            parameters.push(param);
        }
    }

    parameters
}

fn extract_parameter(node: &Node, source: &str) -> Option<Parameter> {
    match node.kind() {
        "identifier" => Some(Parameter {
            name: node.utf8_text(source.as_bytes()).ok()?.to_string(),
            type_annotation: None,
            default_value: None,
        }),
        "typed_parameter" | "typed_default_parameter" => {
            let name = node
                .child_by_field_name("name")
                .and_then(|n| n.utf8_text(source.as_bytes()).ok())
                .map(|s| s.to_string())
                .or_else(|| get_first_identifier(node, source).ok())?;
            let type_annotation = node
                .child_by_field_name("type")
                .and_then(|t| t.utf8_text(source.as_bytes()).ok())
                .map(|s| s.trim().to_string());
            let default_value = node
                .child_by_field_name("value")
                .and_then(|v| v.utf8_text(source.as_bytes()).ok())
                .map(|s| s.trim().to_string());

            Some(Parameter {
                name,
                type_annotation,
                default_value,
            })
        }
        "default_parameter" => {
            let name = node
                .child_by_field_name("name")
                .and_then(|n| n.utf8_text(source.as_bytes()).ok())
                .map(|s| s.to_string())
                .or_else(|| get_first_identifier(node, source).ok())?;
            let default_value = node
                .child_by_field_name("value")
                .and_then(|v| v.utf8_text(source.as_bytes()).ok())
                .map(|s| s.trim().to_string());

            Some(Parameter {
                name,
                type_annotation: None,
                default_value,
            })
        }
        "list_splat_pattern" => {
            let name = get_first_identifier(node, source).ok()?;
            Some(Parameter {
                name: format!("*{}", name),
                type_annotation: None,
                default_value: None,
            })
        }
        "dictionary_splat_pattern" => {
            let name = get_first_identifier(node, source).ok()?;
            Some(Parameter {
                name: format!("**{}", name),
                type_annotation: None,
                default_value: None,
            })
        }
        _ => None,
    }
}

fn extract_return_type(node: &Node, source: &str) -> Option<String> {
    node.child_by_field_name("return_type")
        .or_else(|| node.child_by_field_name("type"))
        .and_then(|n| n.utf8_text(source.as_bytes()).ok())
        .map(|s| s.trim().to_string())
        .or_else(|| {
            let mut cursor = node.walk();
            let mut seen_arrow = false;
            for child in node.children(&mut cursor) {
                if seen_arrow {
                    return child
                        .utf8_text(source.as_bytes())
                        .ok()
                        .map(|s| s.trim().to_string());
                }
                if child.kind() == "->" {
                    seen_arrow = true;
                }
            }
            None
        })
}

fn extract_imports(node: &Node, source: &str) -> Vec<String> {
    let mut deps = Vec::new();
    let text = node.utf8_text(source.as_bytes()).unwrap_or("").trim();

    if let Some(list) = text.strip_prefix("import ") {
        for item in list.split(',') {
            let name = item.split_whitespace().next().unwrap_or("");
            if let Some(dep) = normalize_dependency(name) {
                deps.push(dep);
            }
        }
    }

    deps
}

fn extract_import_from(node: &Node, source: &str) -> Vec<String> {
    let mut deps = Vec::new();
    let text = node.utf8_text(source.as_bytes()).unwrap_or("").trim();

    if let Some(from_part) = text.strip_prefix("from ")
        && let Some((module, _)) = from_part.split_once(" import ")
    {
        let module = module.trim().trim_start_matches('.');
        if let Some(dep) = normalize_dependency(module) {
            deps.push(dep);
        }
    }

    deps
}

fn extract_superclasses(node: &Node, source: &str) -> Vec<String> {
    let mut deps = Vec::new();

    if let Some(superclasses) = node.child_by_field_name("superclasses") {
        let mut cursor = superclasses.walk();
        for child in superclasses.named_children(&mut cursor) {
            if let Ok(text) = child.utf8_text(source.as_bytes())
                && let Some(dep) = normalize_dependency(text.trim())
            {
                deps.push(dep);
            }
        }
    } else if let Some(arg_list) = find_child_kind(node, "argument_list") {
        let text = arg_list.utf8_text(source.as_bytes()).unwrap_or("").trim();
        let text = text.trim_start_matches('(').trim_end_matches(')');
        for item in text.split(',') {
            if let Some(dep) = normalize_dependency(item.trim()) {
                deps.push(dep);
            }
        }
    }

    deps
}

fn extract_decorators(node: &Node, source: &str) -> Vec<String> {
    let mut decorators = Vec::new();
    let mut cursor = node.walk();

    for child in node.named_children(&mut cursor) {
        if child.kind() == "decorator" && let Ok(text) = child.utf8_text(source.as_bytes()) {
            let trimmed = text.trim().trim_start_matches('@');
            let name = trimmed
                .split('(')
                .next()
                .unwrap_or("")
                .split('.')
                .next_back()
                .unwrap_or("")
                .trim()
                .to_string();
            if !name.is_empty() {
                decorators.push(name);
            }
        }
    }

    decorators
}

fn extract_docstring_from_block(block: &Node, source: &str) -> Documentation {
    let mut cursor = block.walk();
    if let Some(child) = block.named_children(&mut cursor).next()
        && child.kind() == "expression_statement"
        && let Some(text) = find_string_literal(&child, source)
    {
        return documentation_from_docstring(&text);
    }

    empty_documentation()
}

fn documentation_from_docstring(text: &str) -> Documentation {
    let cleaned = strip_string_delimiters(text);
    let mut lines = cleaned.lines().map(|l| l.trim()).filter(|l| !l.is_empty());

    let summary = lines.next().map(|s| s.to_string());
    let description_lines: Vec<String> = lines.map(|s| s.to_string()).collect();
    let description = if description_lines.is_empty() {
        None
    } else {
        Some(description_lines.join("\n"))
    };

    Documentation {
        summary,
        description,
        examples: Vec::new(),
    }
}

fn strip_string_delimiters(text: &str) -> String {
    let trimmed = text.trim();
    let first_quote = trimmed
        .find('"')
        .or_else(|| trimmed.find('\''))
        .unwrap_or(0);
    let without_prefix = &trimmed[first_quote..];

    for delimiter in ["\"\"\"", "'''", "\"", "'"] {
        if without_prefix.starts_with(delimiter) && without_prefix.ends_with(delimiter) {
            let inner = &without_prefix[delimiter.len()..without_prefix.len() - delimiter.len()];
            return inner.to_string();
        }
    }

    without_prefix.to_string()
}

fn find_string_literal(node: &Node, source: &str) -> Option<String> {
    if matches!(
        node.kind(),
        "string" | "string_literal" | "concatenated_string"
    ) {
        return node
            .utf8_text(source.as_bytes())
            .ok()
            .map(|s| s.to_string());
    }

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        if let Some(value) = find_string_literal(&child, source) {
            return Some(value);
        }
    }

    None
}

fn get_child_text(node: &Node, source: &str, field: &str) -> Result<String> {
    node.child_by_field_name(field)
        .and_then(|child| child.utf8_text(source.as_bytes()).ok())
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow!("Missing field '{}'", field))
}

fn get_first_identifier(node: &Node, source: &str) -> Result<String> {
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        if child.kind() == "identifier" {
            return Ok(child.utf8_text(source.as_bytes()).unwrap_or("").to_string());
        }
    }

    Err(anyhow!("Missing identifier"))
}

fn find_child_kind<'a>(node: &'a Node<'a>, kind: &str) -> Option<Node<'a>> {
    let mut cursor = node.walk();
    node.named_children(&mut cursor)
        .find(|child| child.kind() == kind)
}

fn normalize_dependency(name: &str) -> Option<String> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return None;
    }
    let root = trimmed.split('.').next().unwrap_or("");
    if root.is_empty() || root == "self" {
        None
    } else {
        Some(root.to_string())
    }
}

fn visibility_from_name(name: &str) -> Visibility {
    if name.starts_with("__") {
        if name.ends_with("__") {
            Visibility::Public
        } else {
            Visibility::Private
        }
    } else if name.starts_with('_') {
        Visibility::Private
    } else {
        Visibility::Public
    }
}

fn empty_documentation() -> Documentation {
    Documentation {
        summary: None,
        description: None,
        examples: Vec::new(),
    }
}
