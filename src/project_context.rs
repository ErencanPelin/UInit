use serde::Serialize;

use crate::config::{ProjectMetadata, UinitConfig};

#[derive(Serialize, Clone)]
pub struct ProjectContext {
    pub template_alias: String,
    pub project_name: String,
    pub company: String,
    pub email: String,
    pub year: i32,
}

impl ProjectContext {
    pub(crate) fn from_config(config: &UinitConfig) -> Self {
        Self {
            template_alias: config.project.template_alias.clone(),
            project_name: config.project.project_name.clone(),
            company: config.project.company.clone(),
            email: config.project.email.clone(),
            year: config.project.year,
        }
    }
}

impl From<&ProjectContext> for UinitConfig {
    fn from(ctx: &ProjectContext) -> Self {
        Self {
            project: ProjectMetadata {
                project_name: ctx.project_name.clone(),
                template_alias: ctx.template_alias.clone(),
                company: ctx.company.clone(),
                email: ctx.email.clone(),
                year: ctx.year,
            },
            aliases: std::collections::HashMap::new(),
        }
    }
}
