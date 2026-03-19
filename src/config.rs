use crate::cli::ProjectType;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AliasEntry {
    pub repo: String,
    pub path: String,
    pub alias_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub project_name: String,
    pub template: ProjectType,
    pub company: String,
    pub email: String,
    pub year: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UinitConfig {
    pub project: ProjectMetadata,
    #[serde(default)]
    pub aliases: HashMap<String, AliasEntry>,
}

impl UinitConfig {
    pub fn load(dir: &Path) -> anyhow::Result<Self> {
        let path = dir.join("unitool.toml");
        let text = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file at {:?}", path))?;

        let config: Self = toml::from_str(&text)
            .context("Failed to parse unitool.toml. Please check your TOML syntax.")?;

        Ok(config)
    }

    // TODO: limitation here is if something goes wrong mid-write
    pub fn save(&self, dir: &Path) -> anyhow::Result<()> {
        let path = dir.join("unitool.toml");
        let text = toml::to_string_pretty(self)?;
        std::fs::write(&path, text)
            .with_context(|| format!("Failed to write config file to {:?}", path))?;

        Ok(())
    }
}
