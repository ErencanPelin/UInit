use std::collections::HashMap;

use comfy_table::Table;

use crate::{
    config::{AliasEntry, UinitConfig},
    constants::DEFAULT_ALIASES,
    unity_project::{self, UnityProject},
};

pub fn get_aliases(config: &UinitConfig) -> HashMap<String, AliasEntry> {
    // merge alias overrides with alias from constants so that user specified aliases can override default ones
    let mut default_aliases: HashMap<String, AliasEntry> = DEFAULT_ALIASES
        .iter()
        .map(|(k, r, p, t)| {
            (
                k.to_string(),
                AliasEntry {
                    repo: r.to_string(),
                    path: p.to_string(),
                    alias_type: t.to_string(),
                },
            )
        })
        .collect();

    default_aliases.extend(config.aliases.clone());

    default_aliases
}

pub fn list_aliases(unity_project: &UnityProject) -> anyhow::Result<()> {
    let config = UinitConfig::load(&unity_project.root)?;

    let aliases = get_aliases(&config);

    let mut table = Table::new();
    // Use a professional "Modern" or "No Border" look
    table.set_header(vec!["Alias", "Type", "Repo Path", "Repo"]);

    let mut keys: Vec<_> = aliases.keys().collect();
    keys.sort();

    for name in keys {
        let entry = &aliases[name];
        table.add_row(vec![
            name,
            &entry.alias_type.to_string(),
            &entry.path,
            &entry.repo,
        ]);
    }

    println!("{table}");
    Ok(())
}
