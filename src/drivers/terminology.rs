/// Language-agnostic terminology for code elements
/// Drivers override these for language-specific accuracy
#[derive(Debug, Clone)]
pub struct LanguageTerminology {
    /// Singular: "type", "struct", "class", "data type"
    pub element_type_singular: String,

    /// Plural: "types", "structs", "classes"
    pub element_type_plural: String,

    /// "item", "function", "def", "fn", "func", "method"
    pub function_label: String,

    /// "items", "functions", "defs", "fns", "funcs", "methods"
    pub function_label_plural: String,

    /// Default return type: "—", "void", "None", "()", "error"
    pub return_type_default: String,

    /// "property", "field", "attribute", "member", "instance variable"
    pub property_label: String,

    /// "properties", "fields", "attributes", "members", "instance variables"
    pub property_label_plural: String,
}

impl LanguageTerminology {
    /// Generic defaults - works for all languages, changes as drivers are added
    pub fn generic() -> Self {
        LanguageTerminology {
            element_type_singular: "type".to_string(),
            element_type_plural: "types".to_string(),
            function_label: "item".to_string(),
            function_label_plural: "items".to_string(),
            return_type_default: "—".to_string(),
            property_label: "field".to_string(),
            property_label_plural: "fields".to_string(),
        }
    }

    /// Rust-specific terminology
    pub fn rust() -> Self {
        LanguageTerminology {
            element_type_singular: "struct".to_string(),
            element_type_plural: "structs".to_string(),
            function_label: "fn".to_string(),
            function_label_plural: "fns".to_string(),
            return_type_default: "()".to_string(),
            property_label: "field".to_string(),
            property_label_plural: "fields".to_string(),
        }
    }

    /// Example for Python driver (future Phase 3)
    #[allow(dead_code)]
    pub fn python() -> Self {
        LanguageTerminology {
            element_type_singular: "class".to_string(),
            element_type_plural: "classes".to_string(),
            function_label: "def".to_string(),
            function_label_plural: "defs".to_string(),
            return_type_default: "None".to_string(),
            property_label: "attribute".to_string(),
            property_label_plural: "attributes".to_string(),
        }
    }

    /// Example for Go driver (future Phase 3+)
    #[allow(dead_code)]
    pub fn go() -> Self {
        LanguageTerminology {
            element_type_singular: "type".to_string(),
            element_type_plural: "types".to_string(),
            function_label: "func".to_string(),
            function_label_plural: "funcs".to_string(),
            return_type_default: "error".to_string(),
            property_label: "field".to_string(),
            property_label_plural: "fields".to_string(),
        }
    }

    /// Example for non-OOP (Haskell - future Phase 3+)
    #[allow(dead_code)]
    pub fn haskell() -> Self {
        LanguageTerminology {
            element_type_singular: "data type".to_string(),
            element_type_plural: "data types".to_string(),
            function_label: "function".to_string(),
            function_label_plural: "functions".to_string(),
            return_type_default: "IO ()".to_string(),
            property_label: "constructor".to_string(),
            property_label_plural: "constructors".to_string(),
        }
    }
}

/// This design is future-proof and scales to any language paradigm:
///
/// **OOP Languages** (Java, C#, Go, Python):
/// - Customize element_type_singular/plural ("class" for Python, "type" for Go)
/// - Customize function_label/plural ("method" for Java, "def" for Python)
/// - Same layout code generates language-appropriate output
///
/// **Non-OOP Languages** (Haskell, Lisp, Clojure):
/// - Override all fields with paradigm-specific primitives
/// - function_label becomes "function" not "method"
/// - element_type becomes "data type" not "class"
/// - Scales to any language without layout changes
///
/// **Multi-Language Projects**:
/// - Each driver's terminology() method provides language-specific terms
/// - Layouts ask for terminology from active driver
/// - Same layout + same IR → different output per language
///
/// Example outputs (same layout, different drivers):
/// ```
/// Rust:    **Contents**: 5 structs, 8 fns
/// Python:  **Contents**: 5 classes, 8 defs
/// Go:      **Contents**: 5 types, 8 funcs
/// Haskell: **Contents**: 5 data types, 8 functions
/// ```
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_terminology() {
        let generic = LanguageTerminology::generic();
        assert_eq!(generic.element_type_singular, "type");
        assert_eq!(generic.function_label, "item");
    }

    #[test]
    fn test_rust_terminology_override() {
        let rust = LanguageTerminology::rust();
        assert_eq!(rust.element_type_singular, "struct");
        assert_eq!(rust.function_label, "fn");
        assert_eq!(rust.return_type_default, "()");
    }

    #[test]
    fn test_python_terminology() {
        let python = LanguageTerminology::python();
        assert_eq!(python.element_type_singular, "class");
        assert_eq!(python.function_label, "def");
        assert_eq!(python.return_type_default, "None");
    }
}
