pub mod markdown;
pub mod python;
pub mod rust;
pub mod terminology;
pub mod trait_def;
pub mod typescript;

use std::path::Path;
pub use terminology::LanguageTerminology;
use trait_def::Driver;

/// Factory: get_driver_for_file(path) -> Box<dyn Driver>
pub fn get_driver_for_file(path: &Path) -> Option<Box<dyn Driver>> {
    let extension = path.extension()?.to_str()?;

    match extension {
        "rs" => Some(Box::new(rust::RustDriver)),
        "py" => Some(Box::new(python::PythonDriver)),
        "ts" | "tsx" => Some(Box::new(typescript::TypeScriptDriver)),
        "md" => Some(Box::new(markdown::MarkdownDriver)),
        _ => None,
    }
}

/// Resolve language-specific terminology without needing file paths
pub fn terminology_for_language(language: &str) -> LanguageTerminology {
    match language {
        "rust" => LanguageTerminology::rust(),
        "python" => LanguageTerminology::python(),
        "typescript" | "javascript" => LanguageTerminology {
            element_type_singular: "class".to_string(),
            element_type_plural: "classes".to_string(),
            function_label: "method".to_string(),
            function_label_plural: "methods".to_string(),
            return_type_default: "void".to_string(),
            property_label: "property".to_string(),
            property_label_plural: "properties".to_string(),
        },
        _ => LanguageTerminology::generic(),
    }
}
