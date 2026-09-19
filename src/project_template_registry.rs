use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{alias_registry::Dependency, enums::ProjectType};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectTemplate {
    pub project_type: ProjectType,
    pub paths: Vec<String>,
    pub dependencies: Vec<Dependency>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectTemplateRegistry {
    pub templates: HashMap<String, ProjectTemplate>,
}

impl ProjectTemplateRegistry {
    pub fn load() -> Self {
        let defaults_str: &str = include_str!("./resources/project_templates.toml");
        let registry: ProjectTemplateRegistry = toml::from_str(defaults_str)
            .expect("Critical Error: Failed to parse embedded project_templates.toml");

        registry
    }

    pub fn get(&self, t: &ProjectType) -> Option<&ProjectTemplate> {
        self.templates.get(&t.to_string())
    }
}
