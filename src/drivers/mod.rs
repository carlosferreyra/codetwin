pub mod trait_def;
pub mod python;
pub mod typescript;
pub mod markdown;

use std::path::Path;
use trait_def::Driver;

/// Factory: get_driver_for_file(path) -> Box<dyn Driver>
pub fn get_driver_for_file(_path: &Path) -> Option<Box<dyn Driver>> {
    // TODO: Match file extension to appropriate driver
    None
}
