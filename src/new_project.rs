use minijinja::{Environment, context};

use crate::constants::{
    CHANGELOG_TEMPLATE, GAME_PROJECT_TEMPLATE, GITIGNORE_TEMPLATE, LICENSE_JINJA,
    NUGET_MOQ_PACKAGE, PACKAGE_JINJA, PACKAGE_PROJECT_TEMPLATE,
};
use crate::fs;
use crate::metadata::ProjectMetadata;
use crate::{cli::ProjectType, unity_project::UnityProject};

pub struct ProjectContext<'a> {
    pub template: ProjectType,
    pub project_name: &'a str,
    pub company: &'a str,
    pub email: &'a str,
    pub year: i32,
}

pub fn new_project(
    ctx: &ProjectContext,
    unity_project: &UnityProject,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create template files and folders
    match ctx.template {
        ProjectType::Game => create_from_template(&ctx, &unity_project, &GAME_PROJECT_TEMPLATE)?,
        ProjectType::Package => {
            create_from_template(&ctx, &unity_project, &PACKAGE_PROJECT_TEMPLATE)?
        }
    }

    modify_project_settings(&ctx, &unity_project);

    // Add common packages that are recommended to use for most projects
    unity_project
        .add_package(NUGET_MOQ_PACKAGE.0, NUGET_MOQ_PACKAGE.1)
        .expect("Failed to add Moq package to manifest.json");

    // Persist metadata so subsequent runs (e.g. `uinit feature`) can reconstruct the context
    let metadata = ProjectMetadata {
        project_name: ctx.project_name.to_string(),
        template: ctx.template,
        company: ctx.company.to_string(),
        email: ctx.email.to_string(),
        year: ctx.year,
    };
    metadata.save(&unity_project.root)?;

    Ok(())
}

fn create_from_template(
    ctx: &ProjectContext,
    unity_project: &UnityProject,
    template: &[&str],
) -> std::io::Result<()> {
    // new environemtn for each project creation to avoid caching issues with different contexts
    let env = Environment::new();

    for entry in template {
        // 1. Replace placeholder
        let path_str = entry.replace("{}", ctx.project_name);
        let path = unity_project.root.join(&path_str);

        // 2. Create file or directory
        if entry.ends_with("/") {
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
                Some("CHANGELOG.md") => std::fs::write(path, CHANGELOG_TEMPLATE)?,
                Some(".gitignore") => std::fs::write(path, GITIGNORE_TEMPLATE)?,
                Some("LICENSE") => {
                    let rendered_license = render_jinja_template(LICENSE_JINJA, &ctx, &env)
                        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
                    std::fs::write(path, rendered_license)?;
                }
                Some("package.json") => {
                    let rendered_package_json = render_jinja_template(PACKAGE_JINJA, &ctx, &env)
                        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
                    std::fs::write(path, rendered_package_json)?;
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn render_jinja_template(
    template_source: &str,
    ctx: &ProjectContext,
    env: &Environment,
) -> Result<String, minijinja::Error> {
    env.render_str(
        template_source,
        context!(
            project_name => ctx.project_name,
            company => ctx.company,
            email => ctx.email,
            year => ctx.year,
        ),
    )
}

/// Update the ProjectSettings.asset with the new company and product name
fn modify_project_settings(ctx: &ProjectContext, unity_project: &UnityProject) {
    let settings_path = unity_project
        .project_settings_dir()
        .join("ProjectSettings.asset");

    // Unity ProjectSettings.asset is YAML, not JSON.
    // Use serde_yaml so we can safely update the structure.
    let mut settings: serde_yaml::Value = serde_yaml::from_str(
        &std::fs::read_to_string(&settings_path).expect("Failed to read ProjectSettings.asset"),
    )
    .expect("Failed to parse ProjectSettings.asset");

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
        serde_yaml::to_string(&settings).expect("Failed to serialize ProjectSettings.asset"),
    )
    .expect("Failed to write ProjectSettings.asset");

    println!(
        "Updated ProjectSettings.asset with company name: '{}' and product name: '{}'.",
        ctx.company, ctx.project_name
    );
}
