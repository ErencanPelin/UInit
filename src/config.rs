use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::{alias_registry::AliasRegistry, fs, project_context::ProjectContext};

#[derive(Debug, Serialize, Deserialize)]
pub struct UinitConfig {
    pub project: ProjectContext,
    pub custom_aliases: AliasRegistry,
}

impl UinitConfig {
    pub fn load(dir: &Path) -> anyhow::Result<Self> {
        let path = dir.join("uinit.toml");
        let text = std::fs::read_to_string(&path).or_else(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                anyhow::bail!("uinit.toml not found. Run 'uinit init' to create one.");
            }
            Err(e).with_context(|| format!("Failed to read {:?}", path))
        })?;

        let config: Self = toml::from_str(&text)
            .context("Failed to parse uinit.toml. Please check your TOML syntax.")?;

        Ok(config)
    }

    // TODO: limitation here is if something goes wrong mid-write
    pub fn save(&self, dir: &Path) -> anyhow::Result<()> {
        let path = dir.join("uinit.toml");
        let text =
            toml::to_string_pretty(self).with_context(|| "Failed to save uinit as .toml.")?;
        fs::write_to_file(&text, &path)?;

        Ok(())
    }
}
