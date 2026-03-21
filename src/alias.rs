use std::collections::HashMap;

use anyhow::bail;
use comfy_table::Table;

use crate::{
    cli::AliasType,
    config::{AliasEntry, UinitConfig},
    constants::DEFAULT_ALIASES,
    unity_project::UnityProject,
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
    let config: UinitConfig = UinitConfig::load(&unity_project.root)?;

    let aliases = get_aliases(&config);

    let mut table = Table::new();

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

pub fn add_alias(
    alias: &String,
    repo: &String,
    path: &String,
    alias_type: &AliasType,
    unity_project: &UnityProject,
) -> anyhow::Result<()> {
    let mut config: UinitConfig = UinitConfig::load(&unity_project.root)?;
    let aliases = get_aliases(&config);

    println!("Adding alias {} to uinit.toml...", alias);

    if aliases.contains_key(alias) {
        bail!(format!(
            "Alias {} already exists in local uinit.toml config",
            alias
        ))
    } else {
        config.aliases.insert(
            alias.to_string(),
            AliasEntry {
                repo: repo.to_string(),
                path: path.to_string(),
                alias_type: alias_type.to_string(),
            },
        );
    }

    config.save(&unity_project.root)?;

    println!("Added alias to uinit.toml!");
    Ok(())
}

pub fn remove_alias(alias: &String, unity_project: &UnityProject) -> anyhow::Result<()> {
    let mut config: UinitConfig = UinitConfig::load(&unity_project.root)?;
    let aliases = get_aliases(&config);

    println!("Removing alias {} from uinit.toml...", alias);

    if !aliases.contains_key(alias) {
        bail!(format!(
            "Alias {} does not exists in local uinit.toml config",
            alias
        ))
    } else {
        config.aliases.remove(&alias.to_string());
    }

    config.save(&unity_project.root)?;

    println!("Removed alias from uinit.toml!");
    Ok(())
}
