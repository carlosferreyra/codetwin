//! `codetwin diff` — compare two snapshots.

use anyhow::Result;

use super::DiffArgs;
use crate::config::Config;
use crate::diff as diff_engine;

/// Entry point for `codetwin diff`.
pub fn run(args: DiffArgs, json: bool) -> Result<()> {
    let config = Config::load_or_default()?;

    if args.watch {
        // TODO(Phase 4.c): watch-mode re-diff on filesystem changes.
        return crate::watch::run_loop(&config, move |cfg| {
            diff_engine::run(cfg, args.ref_a.as_deref(), args.ref_b.as_deref(), json)
        });
    }

    diff_engine::run(&config, args.ref_a.as_deref(), args.ref_b.as_deref(), json)
}
