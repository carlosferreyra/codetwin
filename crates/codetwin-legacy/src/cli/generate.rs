//! `codetwin gen` — render documentation for the current project.

use anyhow::Result;

use super::GenArgs;
use crate::config::Config;
use crate::pipeline::{self, GenOptions};

/// Entry point for `codetwin gen`.
pub fn run(args: GenArgs, json: bool) -> Result<()> {
    let mut config = Config::load_or_default()?;
    apply_overrides(&mut config, &args);

    if args.save {
        // TODO(Phase 1.e): persist merged config back to `codetwin.toml`.
        //                  Mirror `Config::save` semantics from v0.1.
        tracing::warn!("--save not yet implemented");
    }

    if args.watch {
        // TODO(Phase 1.e): wire up the watcher via `crate::watch::run_loop`.
        //                  Should re-run the pipeline on debounced fs events.
        return crate::watch::run_loop(&config, move |cfg| {
            pipeline::run(cfg, &args.clone().into(), json)
        });
    }

    pipeline::run(&config, &args.into(), json)
}

fn apply_overrides(config: &mut Config, args: &GenArgs) {
    if let Some(layout) = &args.layout {
        config.layout = layout.clone();
    }
    if let Some(output) = &args.output {
        config.output_file = output.clone().into();
    }
    if let Some(format) = &args.format {
        config.format = format.parse().unwrap_or(config.format);
    }
    if !args.source.is_empty() {
        config.source_dirs = args.source.iter().map(Into::into).collect();
    }
    if !args.exclude.is_empty() {
        config.exclude_patterns.extend(args.exclude.iter().cloned());
    }
    if !args.drivers.is_empty() {
        config.drivers = Some(args.drivers.clone());
    }
}

impl From<GenArgs> for GenOptions {
    fn from(value: GenArgs) -> Self {
        Self {
            dump_ir: value.dump_ir,
            multi_file: value.multi_file,
        }
    }
}
