use std::collections::HashMap;

use anyhow::Context;
use minijinja::{Environment, context};

use crate::config::{ProjectMetadata, UinitConfig};
use crate::constants::{
    CHANGELOG_TEMPLATE, GITIGNORE_TEMPLATE, LICENSE_JINJA, PACKAGE_JINJA, PROJECT_TEMPLATES,
    README_JINJA,
};
use crate::fs;
use crate::unity_project::UnityProject;

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

pub fn init_project(ctx: &ProjectContext, unity_project: &UnityProject) -> anyhow::Result<()> {
    // Create template files and folders
    create_from_template(&ctx, &unity_project).with_context(|| {
        format!(
            "Failed to create from project template {:?}",
            ctx.template_alias
        )
    })?;

    modify_project_settings(&ctx, &unity_project)?;

    // Persist metadata so subsequent runs (e.g. `uinit feature`) can reconstruct the context
    let config = UinitConfig {
        project: ProjectMetadata {
            project_name: ctx.project_name.to_string(),
            template_alias: ctx.template_alias.to_string(),
            company: ctx.company.to_string(),
            email: ctx.email.to_string(),
            year: ctx.year,
        },
        aliases: HashMap::new(),
    };
    config.save(&unity_project.root)?;

    Ok(())
}

fn create_from_template(ctx: &ProjectContext, unity_project: &UnityProject) -> anyhow::Result<()> {
    // new environemtn for each project creation to avoid caching issues with different contexts
    let env = Environment::new();

    let template = PROJECT_TEMPLATES
        .iter()
        .find(|(name, _, _)| *name == ctx.template_alias);

    if let Some((name, folders, deps)) = template {
        println!("Selected template: {}", name);

        for &dir in *folders {
            // 1. Replace placeholder
            let path_str = &dir.replace("{}", &ctx.project_name.to_string());
            let path = unity_project.root.join(&path_str);

            // 2. Create file or directory
            if dir.ends_with("/") {
                fs::create_directory(&path)?;
                println!("Created directory: {}", path.display());
            } else {
                fs::create_file(&path)?;
                println!("Created file: {}", path.display());

                // if we reach this point it means the file was created successfully, so we can write content if needed
                // this stops us from overwriting existing files
                match std::path::Path::new(&path_str)
                    .file_name()
                    .and_then(|n| n.to_str())
                {
                    Some("CHANGELOG.md") => std::fs::write(&path, CHANGELOG_TEMPLATE)
                        .with_context(|| format!("Failed to write changelog.md at {:?}", path))?,

                    Some(".gitignore") => std::fs::write(&path, GITIGNORE_TEMPLATE)
                        .with_context(|| format!("Failed to write .gitignore at {:?}", path))?,

                    Some("LICENSE") => {
                        let rendered_license = render_jinja_template(LICENSE_JINJA, &ctx, &env)
                            .with_context(|| "Failed to render license from template")?;
                        std::fs::write(&path, rendered_license)
                            .with_context(|| format!("Failed to write license at {:?}", path))?;
                    }
                    Some("package.json") => {
                        let rendered_package_json =
                            render_jinja_template(PACKAGE_JINJA, &ctx, &env)
                                .with_context(|| "Failed to render package.json from template")?;
                        std::fs::write(&path, rendered_package_json).with_context(|| {
                            format!("Failed to write package.json at {:?}", path)
                        })?;
                    }
                    Some("README.md") => {
                        let rendered_readme = render_jinja_template(README_JINJA, &ctx, &env)
                            .with_context(|| "Failed to render README.md from template")?;
                        std::fs::write(&path, rendered_readme)
                            .with_context(|| format!("Failed to write README.md at {:?}", path))?;
                    }
                    _ => {}
                }
            }
        }

        // add template dependencies
        for &dependency in *deps {
            unity_project
                .add_package(dependency.0, dependency.1)
                .context("Failed to add Moq package to manifest.json")?;
        }
    } else {
        anyhow::bail!("Template '{}' not found", ctx.template_alias);
    }

    Ok(())
}

fn render_jinja_template(
    template_source: &str,
    ctx: &ProjectContext,
    env: &Environment,
) -> anyhow::Result<String> {
    let rendered = env.render_str(
        template_source,
        context!(
            project_name => ctx.project_name,
            company => ctx.company,
            email => ctx.email,
            year => ctx.year,
        ),
    )?;

    Ok(rendered)
}

/// Update the ProjectSettings.asset with the new company and product name
fn modify_project_settings(
    ctx: &ProjectContext,
    unity_project: &UnityProject,
) -> anyhow::Result<()> {
    let settings_path = unity_project
        .project_settings_dir()
        .join("ProjectSettings.asset");

    // Unity ProjectSettings.asset is YAML, not JSON.
    // Use serde_yaml so we can safely update the structure.
    let mut settings: serde_yaml::Value = serde_yaml::from_str(
        &std::fs::read_to_string(&settings_path)
            .with_context(|| "Failed to read ProjectSettings.asset")?,
    )
    .with_context(|| "Failed to parse ProjectSettings.asset. Ensure the file is in yaml format.")?;

    if let Some(player_settings) = settings.get_mut("PlayerSettings") {
        if let Some(company_name) = player_settings.get_mut("companyName") {
            *company_name = serde_yaml::Value::String(ctx.company.to_string());
        }
        if let Some(product_name) = player_settings.get_mut("productName") {
            *product_name = serde_yaml::Value::String(ctx.project_name.to_string());
        }
    }

    std::fs::write(
        settings_path,
        serde_yaml::to_string(&settings)
            .with_context(|| "Failed to serialize ProjectSettings to yaml.")?,
    )
    .with_context(|| "Failed to write to ProjectSettings.asset. Make sure the file exists and you have permission to modify it.")?;

    println!(
        "Updated ProjectSettings.asset with company name: '{}' and product name: '{}'.",
        ctx.company, ctx.project_name
    );

    Ok(())
}
