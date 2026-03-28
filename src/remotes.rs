use anyhow::bail;
use comfy_table::Table;

use crate::{
    alias_registry::{AliasRegistry, RemoteResource},
    config::UinitConfig,
    enums::RemoteCategory,
    reporter::Reporter,
    unity_project::UnityProject,
};

pub fn list_aliases(unity_project: &UnityProject, reporter: &Reporter) -> anyhow::Result<()> {
    reporter.info("Loading uinit.toml file");
    let config: UinitConfig = UinitConfig::load(&unity_project.root)?;
    reporter.info("Loading default_aliases.toml file");
    let registry = AliasRegistry::load(&config);

    reporter.info("Creating table...");
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
    alias_type: &RemoteCategory,
    unity_project: &UnityProject,
    reporter: &Reporter,
) -> anyhow::Result<()> {
    reporter.info("Loading uinit.toml file");
    let mut config: UinitConfig = UinitConfig::load(&unity_project.root)?;

    println!("Adding custom alias '{}' to uinit.toml...", alias);

    reporter.info("Validate alias doesn't not already exist in custom aliases.");
    if config.custom_aliases.remotes.contains_key(alias) {
        let confirmation = reporter.prompt(&format!(
            "Alias {} already exists in local uinit.toml config.\n
            Do you want to override it?",
            alias
        ));
        if !confirmation {
            return Ok(());
        }
    }

    reporter.info("Adding alias to alias registry.");
    // We got past the confirmation
    config.custom_aliases.remotes.insert(
        alias.to_string(),
        RemoteResource {
            url: repo.to_string(),
            path: path.to_string(),
            category: alias_type.to_string(),
        },
    );

    reporter.info("Saving config and writing uinit.toml to disk.");
    config.save(&unity_project.root)?;

    reporter.success(&format!("Added custom alias '{}' to uinit.toml.", alias));
    Ok(())
}

pub fn remove_alias(
    alias: &String,
    unity_project: &UnityProject,
    reporter: &Reporter,
) -> anyhow::Result<()> {
    reporter.info("Loading uinit.toml file");
    let mut config: UinitConfig = UinitConfig::load(&unity_project.root)?;

    println!("Removing custom alias '{}' from uinit.toml...", alias);

    reporter.info("Validating custom alias does exist in uinit.toml");
    if !config.custom_aliases.remotes.contains_key(alias) {
        bail!(format!(
            "Alias {} does not exists in local uinit.toml config",
            alias
        ))
    } else {
        reporter.info("Removing alias from alias registry");
        config.custom_aliases.remotes.remove(&alias.to_string());
    }

    reporter.info("Saving config and writing uinit.toml to disk.");
    config.save(&unity_project.root)?;

    reporter.success(&format!(
        "Removed custom alias '{}' from uinit.toml.",
        alias
    ));
    Ok(())
}
