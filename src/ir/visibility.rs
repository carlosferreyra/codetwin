//! Visibility modifier.

use serde::{Deserialize, Serialize};

/// Access modifier for a [`Symbol`](super::Symbol).
///
/// Languages map onto these as follows:
///
/// | Language   | Public       | Private      | Protected | Internal       |
/// | ---------- | ------------ | ------------ | --------- | -------------- |
/// | Rust       | `pub`        | (none)       | —         | `pub(crate)`   |
/// | Python     | no `_`       | `_prefix`    | —         | —              |
/// | TypeScript | `export`     | (none)       | —         | —              |
/// | Go         | Upper-case   | lower-case   | —         | —              |
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Visibility {
    /// Visible outside the module/crate.
    Public,
    /// Not visible outside the module.
    #[default]
    Private,
    /// Visible to subclasses (OO languages).
    Protected,
    /// Visible only within the current crate/package.
    Internal,
}
