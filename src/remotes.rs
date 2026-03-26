use std::fmt;

use anyhow::bail;
use clap::ValueEnum;
use comfy_table::Table;
use dialoguer::{Confirm, theme::ColorfulTheme};
use serde::{Deserialize, Serialize};

use crate::{
    alias_registry::{AliasRegistry, RemoteResource},
    config::UinitConfig,
    unity_project::UnityProject,
};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, ValueEnum)] // Add ValueEnum here
#[serde(rename_all = "lowercase")]
pub enum AliasCategory {
    Util,
    Module,
    Tool,
}

// Keep your Display implementation for the 'list' table
impl fmt::Display for AliasCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AliasCategory::Util => write!(f, "util"),
            AliasCategory::Module => write!(f, "module"),
            AliasCategory::Tool => write!(f, "tool"),
        }
    }
}

pub fn list_aliases(unity_project: &UnityProject) -> anyhow::Result<()> {
    let config: UinitConfig = UinitConfig::load(&unity_project.root)?;
    let registry = AliasRegistry::load(&config);

    let mut table = Table::new();

    table.set_header(vec!["Alias", "Category", "Repo Path", "Repo URL"]);

    let mut keys: Vec<_> = registry.remotes.keys().collect();
    keys.sort();

    for name in keys {
        let entry = &registry.remotes[name];
        table.add_row(vec![
            name,
            &entry.category.to_string(),
            &entry.path,
            &entry.url,
        ]);
    }

    println!("{table}");
    Ok(())
}

pub fn add_alias(
    alias: &String,
    repo: &String,
    path: &String,
    alias_type: &AliasCategory,
    unity_project: &UnityProject,
) -> anyhow::Result<()> {
    let mut config: UinitConfig = UinitConfig::load(&unity_project.root)?;

    println!("Adding alias {} to uinit.toml...", alias);

    if config.custom_aliases.remotes.contains_key(alias) {
        let confirmation = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!(
                "Alias {} already exists in local uinit.toml config.\n
                    Do you want to override it?",
                alias
            ))
            .default(false)
            .wait_for_newline(true)
            .interact()
            .unwrap_or(false);

        if !confirmation {
            return Ok(());
        }
    }

    // We got past the confirmation
    config.custom_aliases.remotes.insert(
        alias.to_string(),
        RemoteResource {
            url: repo.to_string(),
            path: path.to_string(),
            category: alias_type.to_string(),
        },
    );

    config.save(&unity_project.root)?;

    // TODO: use reporter success
    println!("Added alias to uinit.toml!");
    Ok(())
}

pub fn remove_alias(alias: &String, unity_project: &UnityProject) -> anyhow::Result<()> {
    let mut config: UinitConfig = UinitConfig::load(&unity_project.root)?;

    println!("Removing alias {} from uinit.toml...", alias);

    if !config.custom_aliases.remotes.contains_key(alias) {
        bail!(format!(
            "Alias {} does not exists in local uinit.toml config",
            alias
        ))
    } else {
        config.custom_aliases.remotes.remove(&alias.to_string());
    }

    config.save(&unity_project.root)?;

    // TODO: use reporter success
    println!("Removed alias from uinit.toml!");
    Ok(())
}
