//! `codetwin snapshot` — capture and cache a `CodeModel` snapshot.

use anyhow::Result;

use super::SnapshotArgs;
use crate::config::Config;
use crate::snapshot;

/// Entry point for `codetwin snapshot`.
pub fn run(args: SnapshotArgs, _json: bool) -> Result<()> {
    let config = Config::load_or_default()?;

    if args.watch {
        // TODO(Phase 4.a): implement watch loop that re-snapshots on change.
        return crate::watch::run_loop(&config, move |cfg| {
            snapshot::capture(cfg, args.r#ref.as_deref()).map(|_| ())
        });
    }

    let path = snapshot::capture(&config, args.r#ref.as_deref())?;
    tracing::info!(path = %path.display(), "snapshot captured");
    Ok(())
}
