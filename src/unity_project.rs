use std::path::Path;

use crate::fs;
use anyhow::Context;

pub struct UnityProject {
    pub root: std::path::PathBuf,
}

impl UnityProject {
    pub fn detect() -> anyhow::Result<Self> {
        let cwd = std::env::current_dir()?;

        if cwd.join("Assets").exists() && cwd.join("ProjectSettings").exists() {
            Ok(Self { root: cwd })
        } else {
            anyhow::bail!("Please run this from the Project's root directory.");
        }
    }

    pub fn assets_dir(&self) -> std::path::PathBuf {
        self.root.join("Assets")
    }

    pub fn packages_dir(&self) -> std::path::PathBuf {
        self.root.join("Packages")
    }

    pub fn project_settings_dir(&self) -> std::path::PathBuf {
        self.root.join("ProjectSettings")
    }

    pub fn rel_path(&self, path: &Path) -> String {
        path.strip_prefix(&self.root)
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_else(|_| path.to_string_lossy().into_owned())
    }
}
