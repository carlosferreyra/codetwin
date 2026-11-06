//! CodeTwin intermediate representation.

pub mod ir;
pub use ir::{
    CodeModel, Edge, EdgeKind, Module, ModuleId, Package, PackageDependency, PackageDependencyKind,
    PackageId, PackageTarget, Symbol, SymbolKind, Visibility,
};
