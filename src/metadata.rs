use crate::cli::ProjectType;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub project_name: String,
    pub template: ProjectType,
    pub company: String,
    pub email: String,
    pub year: i32,
}

impl ProjectMetadata {
    pub fn load(dir: &Path) -> anyhow::Result<Self> {
        let path = dir.join(".uinit.json");
        let text = std::fs::read_to_string(&path)?;
        let meta: Self = serde_json::from_str(&text)?;
        Ok(meta)
    }

    pub fn save(&self, dir: &Path) -> anyhow::Result<()> {
        let path = dir.join(".uinit.json");
        let text = serde_json::to_string_pretty(self)?;
        std::fs::write(path, text)?;
        Ok(())
    }
}
