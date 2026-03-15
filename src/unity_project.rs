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
}
