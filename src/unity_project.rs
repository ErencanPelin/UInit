use std::path::Path;

use anyhow::Context;

use crate::fs;

pub struct UnityProject {
    pub root: std::path::PathBuf,
}

impl UnityProject {
    pub fn detect() -> anyhow::Result<Self> {
        let cwd = std::env::current_dir()?;

        if cwd.join("Assets").exists() && cwd.join("ProjectSettings").exists() {
            Ok(Self { root: cwd })
        } else {
            anyhow::bail!("Please run this from inside the `Assets` directory.");
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

    pub fn add_package(&self, package_name: &str, version: &str) -> anyhow::Result<()> {
        let path = self.packages_dir().join("manifest.json");

        let mut manifest: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&path)?)
                .with_context(|| "Failed to parse manifest.json")?;

        let deps = manifest
            .get_mut("dependencies")
            .and_then(|d| d.as_object_mut())
            .ok_or_else(|| anyhow::anyhow!("manifest.json missing 'dependencies' object"))?;

        // Check if we actually need to change anything
        let existing_version = deps.get(package_name).and_then(|v| v.as_str());

        if existing_version == Some(version) {
            println!(
                "  ✅ Package {} is already at version {}.",
                package_name, version
            );
            return Ok(());
        }

        // Update or Insert
        let existing = deps.insert(package_name.to_string(), version.into());

        // Use your fs module for the write
        let output = serde_json::to_string_pretty(&manifest)?;
        fs::write_to_file(&output, &path)?;

        if existing.is_none() {
            println!(
                "  ✅ Added package {} {} in manifest.json",
                package_name, version
            );
        } else {
            println!(
                "  ✅ Updated package {} to {} in manifest.json",
                package_name, version
            );
        }
        Ok(())
    }
}
