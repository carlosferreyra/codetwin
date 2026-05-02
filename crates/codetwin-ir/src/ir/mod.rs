//! Intermediate representation shared between drivers and layouts.
//!
//! Every driver produces a [`CodeModel`]; every layout consumes one. This is
//! the contract from NEW_ROADMAP Phase 1.a.
//!
//! The IR is `serde`-friendly so it can be cached on disk for snapshots and
//! diffs (Phase 4).

mod edge;
mod model;
mod module;
mod symbol;
mod visibility;

pub use edge::{Edge, EdgeKind};
pub use model::CodeModel;
pub use module::{Module, ModuleId};
pub use symbol::{Symbol, SymbolKind};
pub use visibility::Visibility;
