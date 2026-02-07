use codetwin::discovery;
use codetwin::drivers::python::PythonDriver;
use codetwin::drivers::trait_def::Driver;
use codetwin::ir::{Blueprint, Class, Documentation, Element, Function, Signature, Visibility};
use codetwin::layouts::dependency_graph::DependencyGraphLayout;
use codetwin::layouts::trait_def::Layout;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn parse_python(source: &str) -> Blueprint {
    let driver = PythonDriver;
    driver.parse(source).expect("Failed to parse Python")
}

fn sample_python() -> &'static str {
    r#"
import os, sys as system
from typing import Optional

class User:
    """User model.

    Extended description line.
    """

    count = 0

    def __init__(self, name: str, age: int = 0) -> None:
        """Create a user."""
        self.name = name
        self.age = age

    @property
    def is_adult(self) -> bool:
        """True if age is >= 18."""
        return self.age >= 18

    @staticmethod
    def normalize_name(name: str) -> str:
        return name.strip()

    @classmethod
    def from_name(cls, name: str) -> "User":
        return cls(name=name)


def helper(value: int) -> Optional[str]:
    """Convert values."""
    if value > 0:
        return str(value)
    return None


class Admin(User):
    pass
"#
}

#[test]
fn test_python_extracts_class() {
    let blueprint = parse_python(sample_python());
    let classes: Vec<&Class> = blueprint
        .elements
        .iter()
        .filter_map(|e| match e {
            Element::Class(c) => Some(c),
            _ => None,
        })
        .collect();

    assert!(!classes.is_empty(), "Should find at least one class");
    assert!(classes.iter().any(|c| c.name == "User"));
}

#[test]
fn test_python_extracts_function() {
    let blueprint = parse_python(sample_python());
    let functions: Vec<&Function> = blueprint
        .elements
        .iter()
        .filter_map(|e| match e {
            Element::Function(f) => Some(f),
            _ => None,
        })
        .collect();

    assert!(functions.iter().any(|f| f.name == "helper"));
}

#[test]
fn test_python_function_docstring() {
    let blueprint = parse_python(sample_python());
    let helper = blueprint
        .elements
        .iter()
        .find_map(|e| match e {
            Element::Function(f) if f.name == "helper" => Some(f),
            _ => None,
        })
        .expect("helper function");

    assert_eq!(
        helper.documentation.summary.as_deref(),
        Some("Convert values.")
    );
}

#[test]
fn test_python_class_docstring() {
    let blueprint = parse_python(sample_python());
    let user = blueprint
        .elements
        .iter()
        .find_map(|e| match e {
            Element::Class(c) if c.name == "User" => Some(c),
            _ => None,
        })
        .expect("User class");

    assert_eq!(user.documentation.summary.as_deref(), Some("User model."));
    assert!(
        user.documentation
            .description
            .as_deref()
            .unwrap_or("")
            .contains("Extended description")
    );
}

#[test]
fn test_python_staticmethod_detected() {
    let blueprint = parse_python(sample_python());
    let user = blueprint
        .elements
        .iter()
        .find_map(|e| match e {
            Element::Class(c) if c.name == "User" => Some(c),
            _ => None,
        })
        .expect("User class");

    let normalize = user
        .methods
        .iter()
        .find(|m| m.name == "normalize_name")
        .expect("normalize_name method");
    assert!(normalize.is_static, "staticmethod should be static");
}

#[test]
fn test_python_classmethod_detected() {
    let blueprint = parse_python(sample_python());
    let user = blueprint
        .elements
        .iter()
        .find_map(|e| match e {
            Element::Class(c) if c.name == "User" => Some(c),
            _ => None,
        })
        .expect("User class");

    let from_name = user
        .methods
        .iter()
        .find(|m| m.name == "from_name")
        .expect("from_name method");
    assert!(
        from_name.is_static,
        "classmethod should be treated as static"
    );
}

#[test]
fn test_python_property_decorator_creates_property() {
    let blueprint = parse_python(sample_python());
    let user = blueprint
        .elements
        .iter()
        .find_map(|e| match e {
            Element::Class(c) if c.name == "User" => Some(c),
            _ => None,
        })
        .expect("User class");

    assert!(user.properties.iter().any(|p| p.name == "is_adult"));
}

#[test]
fn test_python_type_annotations_in_parameters() {
    let blueprint = parse_python(sample_python());
    let user = blueprint
        .elements
        .iter()
        .find_map(|e| match e {
            Element::Class(c) if c.name == "User" => Some(c),
            _ => None,
        })
        .expect("User class");

    let init_method = user
        .methods
        .iter()
        .find(|m| m.name == "__init__")
        .expect("__init__ method");
    let name_param = init_method
        .signature
        .parameters
        .iter()
        .find(|p| p.name == "name")
        .expect("name parameter");

    assert_eq!(name_param.type_annotation.as_deref(), Some("str"));
}

#[test]
fn test_python_return_type_annotation() {
    let blueprint = parse_python(sample_python());
    let helper = blueprint
        .elements
        .iter()
        .find_map(|e| match e {
            Element::Function(f) if f.name == "helper" => Some(f),
            _ => None,
        })
        .expect("helper function");

    assert_eq!(
        helper.signature.return_type.as_deref(),
        Some("Optional[str]")
    );
}

#[test]
fn test_python_import_dependencies() {
    let blueprint = parse_python(sample_python());
    assert!(blueprint.dependencies.contains(&"os".to_string()));
    assert!(blueprint.dependencies.contains(&"sys".to_string()));
    assert!(blueprint.dependencies.contains(&"typing".to_string()));
}

#[test]
fn test_python_class_inheritance_dependency() {
    let blueprint = parse_python(sample_python());
    assert!(
        blueprint.dependencies.contains(&"User".to_string()),
        "Base class should appear as dependency"
    );
}

#[test]
fn test_python_assignment_property() {
    let blueprint = parse_python(sample_python());
    let user = blueprint
        .elements
        .iter()
        .find_map(|e| match e {
            Element::Class(c) if c.name == "User" => Some(c),
            _ => None,
        })
        .expect("User class");

    assert!(user.properties.iter().any(|p| p.name == "count"));
}

#[test]
fn test_python_visibility_private() {
    let code = r#"
class _Hidden:
    pass

def _helper():
    pass
"#;
    let blueprint = parse_python(code);
    let hidden = blueprint
        .elements
        .iter()
        .find_map(|e| match e {
            Element::Class(c) if c.name == "_Hidden" => Some(c),
            _ => None,
        })
        .expect("Hidden class");
    let helper = blueprint
        .elements
        .iter()
        .find_map(|e| match e {
            Element::Function(f) if f.name == "_helper" => Some(f),
            _ => None,
        })
        .expect("helper function");

    assert_eq!(hidden.visibility, Visibility::Private);
    assert_eq!(helper.visibility, Visibility::Private);
}

#[test]
fn test_multi_language_discovery() {
    let tmp_base = std::env::temp_dir();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let root = tmp_base.join(format!("codetwin_phase3_{}", now));
    let src_dir = root.join("src");
    fs::create_dir_all(&src_dir).expect("create temp src");

    fs::write(src_dir.join("main.rs"), "fn main() {}\n").expect("write rust");
    fs::write(src_dir.join("utils.py"), "def helper():\n    return 1\n").expect("write python");

    let files = discovery::find_source_files(&[src_dir.to_string_lossy().to_string()], &[])
        .expect("discover files");

    assert!(
        files
            .iter()
            .any(|p| p.extension().and_then(|s| s.to_str()) == Some("rs"))
    );
    assert!(
        files
            .iter()
            .any(|p| p.extension().and_then(|s| s.to_str()) == Some("py"))
    );

    fs::remove_dir_all(&root).expect("cleanup temp dir");
}

#[test]
fn test_dependency_graph_uses_python_terms() {
    let blueprint = Blueprint {
        source_path: PathBuf::from("sample.py"),
        language: "python".to_string(),
        elements: vec![
            Element::Class(Class {
                name: "Widget".to_string(),
                visibility: Visibility::Public,
                methods: vec![],
                properties: vec![],
                documentation: Documentation {
                    summary: None,
                    description: None,
                    examples: vec![],
                },
            }),
            Element::Function(Function {
                name: "build".to_string(),
                visibility: Visibility::Public,
                signature: Signature {
                    parameters: vec![],
                    return_type: None,
                },
                documentation: Documentation {
                    summary: None,
                    description: None,
                    examples: vec![],
                },
            }),
        ],
        dependencies: vec![],
    };

    let layout = DependencyGraphLayout;
    let outputs = layout.format(&[blueprint]).expect("layout output");
    let content = outputs[0].1.as_str();

    assert!(content.contains("classes"));
    assert!(content.contains("defs"));
}
