/// Intermediate Representation - The "Universal Structs"
/// Inspired by UML/OO paradigm for cross-language documentation sync
use std::path::PathBuf;

/// Blueprint = Complete structural representation of a source file
#[derive(Debug, Clone, PartialEq)]
pub struct Blueprint {
    pub source_path: PathBuf,
    pub language: String, // "python", "typescript", "rust", etc.
    pub elements: Vec<Element>,
    pub dependencies: Vec<String>, // Module names this file depends on
}

/// Element = Any documentable code construct (module-level only, no nesting)
#[derive(Debug, Clone, PartialEq)]
pub enum Element {
    Module(Module),
    Class(Class),
    Function(Function),
}

/// Module = A file or namespace container
#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub name: String,
    pub documentation: Documentation,
}

/// Class = OO class/interface/struct
#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    pub name: String,
    pub visibility: Visibility,
    pub methods: Vec<Method>,
    pub properties: Vec<Property>,
    pub documentation: Documentation,
}

/// Function = Top-level function (not nested, not a method)
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub visibility: Visibility,
    pub signature: Signature,
    pub documentation: Documentation,
}

/// Method = Function inside a class
#[derive(Debug, Clone, PartialEq)]
pub struct Method {
    pub name: String,
    pub visibility: Visibility,
    pub is_static: bool,
    pub signature: Signature,
    pub documentation: Documentation,
}

/// Property = Class field/attribute
#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    pub name: String,
    pub visibility: Visibility,
    pub type_annotation: Option<String>, // Simple string: "str", "List[int]", "Promise<User>"
    pub documentation: Documentation,
}

/// Signature = Parameters + return type (body is ignored)
#[derive(Debug, Clone, PartialEq)]
pub struct Signature {
    pub parameters: Vec<Parameter>,
    pub return_type: Option<String>, // Simple string: "void", "int", "Result<T, E>"
}

/// Parameter = Function/method argument
#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub type_annotation: Option<String>, // Simple string
    pub default_value: Option<String>,   // "None", "0", "true"
}

/// Visibility = Access modifier
#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Public,
    Private,
    Protected,
    Internal, // For languages like C#, Kotlin
}

/// Documentation = Prose extracted from comments/docstrings
#[derive(Debug, Clone, PartialEq)]
pub struct Documentation {
    pub summary: Option<String>,     // One-line description
    pub description: Option<String>, // Multi-line detailed explanation
    pub examples: Vec<String>,       // Code snippets showing usage
}
