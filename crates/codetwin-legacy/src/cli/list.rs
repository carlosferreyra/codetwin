//! `codetwin list` — print the registered drivers and layouts.

use anyhow::Result;

use super::ListArgs;
use crate::drivers::DriverRegistry;
use crate::layouts::LayoutRegistry;

/// Entry point for `codetwin list`.
pub fn run(args: ListArgs, json: bool) -> Result<()> {
    // Default: show both when neither flag is given.
    let show_drivers = args.drivers || !args.layouts;
    let show_layouts = args.layouts || !args.drivers;

    if json {
        let payload = serde_json::json!({
            "drivers": if show_drivers { DriverRegistry::default().names() } else { vec![] },
            "layouts": if show_layouts { LayoutRegistry::default().names() } else { vec![] },
        });
        println!("{}", serde_json::to_string_pretty(&payload)?);
        return Ok(());
    }

    if show_drivers {
        println!("drivers:");
        for name in DriverRegistry::default().names() {
            println!("  - {name}");
        }
    }
    if show_layouts {
        println!("layouts:");
        for name in LayoutRegistry::default().names() {
            println!("  - {name}");
        }
    }
    Ok(())
}
