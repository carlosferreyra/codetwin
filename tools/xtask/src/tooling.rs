//! Shared action references for workflow rendering.

use toml_edit::DocumentMut;

use crate::Result;

pub(crate) fn workflow(template: &str) -> Result<String> {
    let registry = include_str!("../assets/tooling.toml")
        .parse::<DocumentMut>()
        .map_err(|error| format!("invalid tooling registry: {error}"))?;
    let actions = registry["actions"]
        .as_table()
        .ok_or("missing actions table")?;
    let mut rendered = template.to_owned();
    for (name, action) in actions {
        rendered = rendered.replace(
            &format!("@{name}@"),
            action
                .as_str()
                .ok_or_else(|| format!("invalid action reference: {name}"))?,
        );
    }
    Ok(rendered)
}
