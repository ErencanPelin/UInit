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

    pub fn add_package(&self, package_name: &str, version: &str) -> anyhow::Result<()> {
        let manifest_path = self.packages_dir().join("manifest.json");
        let mut manifest: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(&manifest_path)
                .with_context(|| "Failed to read manifest.json")?,
        )
        .with_context(|| "Failed to parse manifest.json")?;

        // Add package to dependencies if not already present
        let dependencies = manifest
            .get_mut("dependencies")
            .and_then(|d| d.as_object_mut())
            .with_context(|| "manifest.json is missing 'dependencies' object")?;

        if !dependencies.contains_key(package_name) {
            dependencies.insert(
                package_name.to_string(),
                serde_json::Value::String(version.to_string()),
            );
            std::fs::write(
                manifest_path,
                serde_json::to_string_pretty(&manifest)
                    .with_context(|| "Failed to serialize manifest.json")?,
            )
            .with_context(|| "Failed to write manifest.json")?;
            println!("Added {} to manifest.json dependencies.", package_name);
        } else {
            println!(
                "{} is already present in manifest.json dependencies.",
                package_name
            );
        }

        Ok(())
    }
}
