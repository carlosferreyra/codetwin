/// The Driver trait definition
use crate::ir::Blueprint;

pub trait Driver {
    fn parse(&self, content: &str) -> Result<Blueprint, String>;
    fn generate(&self, blueprint: &Blueprint) -> Result<String, String>;
}
