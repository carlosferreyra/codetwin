//! `codetwin init` — write a starter `codetwin.toml`.

use anyhow::Result;

use super::InitArgs;
use crate::config::Config;

/// Entry point for `codetwin init`.
pub fn run(args: InitArgs) -> Result<()> {
    // TODO(Phase 1.e): honour `--force`; bail with a friendly message when
    //                  `codetwin.toml` already exists and `--force` is unset.
    let _ = args.force;
    Config::default().save_to_default_path()?;
    tracing::info!("wrote codetwin.toml");
    Ok(())
}
