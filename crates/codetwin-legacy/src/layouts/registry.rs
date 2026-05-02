//! Layout registry.

use super::{ArchitectureMapLayout, C4Layout, Layout, MetricsLayout, ProjectOverviewLayout};

/// Ordered collection of layouts.
pub struct LayoutRegistry {
    layouts: Vec<Box<dyn Layout>>,
}

impl Default for LayoutRegistry {
    fn default() -> Self {
        let mut r = Self::empty();
        r.register(Box::new(ProjectOverviewLayout));
        r.register(Box::new(ArchitectureMapLayout));
        r.register(Box::new(C4Layout));
        r.register(Box::new(MetricsLayout));
        r
    }
}

impl LayoutRegistry {
    /// An empty registry.
    pub fn empty() -> Self {
        Self {
            layouts: Vec::new(),
        }
    }

    /// Register a layout. Later entries take precedence on name collisions.
    pub fn register(&mut self, layout: Box<dyn Layout>) {
        self.layouts.push(layout);
    }

    /// Names of all registered layouts.
    pub fn names(&self) -> Vec<&'static str> {
        self.layouts.iter().map(|l| l.name()).collect()
    }

    /// Look up a layout by its name.
    pub fn get(&self, name: &str) -> Option<&dyn Layout> {
        self.layouts
            .iter()
            .map(|l| l.as_ref())
            .find(|l| l.name() == name)
    }
}
