//! Filesystem watcher shared by `gen --watch`, `snapshot --watch`, and
//! `diff --watch` (NEW_ROADMAP Phase 1.e).

use std::sync::mpsc::channel;
use std::time::Duration;

use anyhow::Result;
use notify_debouncer_mini::{DebounceEventResult, new_debouncer, notify::RecursiveMode};

use crate::config::Config;

/// Default debounce window (ms).
pub const DEFAULT_DEBOUNCE_MS: u64 = 300;

/// Run `task` once, then whenever any file under `config.source_dirs`
/// changes (debounced).
pub fn run_loop<F>(config: &Config, mut task: F) -> Result<()>
where
    F: FnMut(&Config) -> Result<()>,
{
    // Initial run.
    task(config)?;

    let (tx, rx) = channel();
    let mut debouncer = new_debouncer(
        Duration::from_millis(DEFAULT_DEBOUNCE_MS),
        move |res: DebounceEventResult| {
            let _ = tx.send(res);
        },
    )?;

    for dir in &config.source_dirs {
        if dir.exists() {
            debouncer.watcher().watch(dir, RecursiveMode::Recursive)?;
        }
    }

    tracing::info!("watching for changes; press Ctrl-C to stop");

    while let Ok(event) = rx.recv() {
        match event {
            Ok(events) if !events.is_empty() => {
                if let Err(err) = task(config) {
                    tracing::error!(error = ?err, "watch task failed");
                }
            }
            Ok(_) => {}
            Err(err) => {
                tracing::warn!(error = ?err, "watcher error");
            }
        }
    }

    Ok(())
}
