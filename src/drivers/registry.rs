//! Driver registry + auto-detection.

use std::path::Path;

use super::{Driver, GoDriver, PythonDriver, RustDriver, TypeScriptDriver};

/// Ordered collection of drivers.
///
/// The default registry contains every built-in driver. Users who embed
/// `codetwin` as a library can construct a custom registry via
/// [`DriverRegistry::empty`] + [`DriverRegistry::register`].
pub struct DriverRegistry {
    drivers: Vec<Box<dyn Driver>>,
}

impl Default for DriverRegistry {
    fn default() -> Self {
        let mut r = Self::empty();
        r.register(Box::new(RustDriver));
        r.register(Box::new(PythonDriver));
        r.register(Box::new(TypeScriptDriver));
        r.register(Box::new(GoDriver));
        r
    }
}

impl DriverRegistry {
    /// An empty registry.
    pub fn empty() -> Self {
        Self {
            drivers: Vec::new(),
        }
    }

    /// Register a driver. Later drivers have higher priority during
    /// detection ties.
    pub fn register(&mut self, driver: Box<dyn Driver>) {
        self.drivers.push(driver);
    }

    /// Names of every registered driver.
    pub fn names(&self) -> Vec<&'static str> {
        self.drivers.iter().map(|d| d.name()).collect()
    }

    /// Return the drivers whose [`Driver::detect`] returns `true` for
    /// `project_root`.
    pub fn detect_all<'a>(&'a self, project_root: &Path) -> Vec<&'a dyn Driver> {
        self.drivers
            .iter()
            .map(|d| d.as_ref())
            .filter(|d| d.detect(project_root))
            .collect()
    }

    /// Find a driver by name.
    pub fn get(&self, name: &str) -> Option<&dyn Driver> {
        self.drivers
            .iter()
            .map(|d| d.as_ref())
            .find(|d| d.name() == name)
    }
}
