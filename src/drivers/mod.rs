pub mod trait_def;
pub mod rust;
pub mod python;
pub mod typescript;
pub mod markdown;

use std::path::Path;
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
