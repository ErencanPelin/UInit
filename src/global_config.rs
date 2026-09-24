use std::path::{Path, PathBuf};

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::{fs::FileSystem, reporter::Reporter};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GlobalConfig {
    pub company: Option<String>,
    pub email: Option<String>,
}

impl GlobalConfig {
    pub fn default_config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|config_dir| config_dir.join("uinit").join("config.toml"))
    }

    pub fn load_from(path: &Path) -> anyhow::Result<Self> {
        match std::fs::read_to_string(path) {
            Ok(contents) => toml::from_str(&contents)
                .context("Failed to parse global uinit config. Please check your TONL syntax."),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Self::default()), // path provided didn't exist, fall back to default config
            Err(e) => Err(e).with_context(|| format!("Failed to read {:?}", path)),
        }
    }

    pub fn load(reporter: &Reporter) -> anyhow::Result<Self> {
        reporter.info("Loading global Uinit config");

        match Self::default_config_path() {
            Some(path) => Self::load_from(&path),
            None => Ok(Self::default()), // no config dir available, fall back to default config
        }
    }

    pub fn save_to(&self, path: &Path, fs: &FileSystem) -> anyhow::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let toml_string = toml::to_string_pretty(self)?;
        fs.write_to_file(&toml_string, path)
            .with_context(|| format!("Failed to write global config to {:?}", path))?;
        Ok(())
    }
}

pub fn handle_config(
    company: Option<String>,
    email: Option<String>,
    reporter: &Reporter,
    fs: &FileSystem,
) -> anyhow::Result<()> {
    let path = GlobalConfig::default_config_path()
        .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;

    reporter.info(&format!("Reading global config from {:?}", path));
    let mut config = GlobalConfig::load_from(&path)?;

    if company.is_none() && email.is_none() {
        println!("Global Uinit config:( {:?})", path);
        println!(
            "  Company: {}",
            config.company.as_deref().unwrap_or("Not set")
        );
        println!("  Email: {}", config.email.as_deref().unwrap_or("Not set"));
        return Ok(());
    }

    if let Some(company) = company {
        config.company = Some(company);
    }
    if let Some(email) = email {
        config.email = Some(email);
    }

    config.save_to(&path, fs)?;
    reporter.success(&format!("Updated global config at {:?}", path));
    Ok(())
}
