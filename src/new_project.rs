use minijinja::{Environment, context};

use crate::constants::{
    CHANGELOG_TEMPLATE, GAME_PROJECT_TEMPLATE, GITIGNORE_TEMPLATE, LICENSE_JINJA, PACKAGE_JINJA,
    PACKAGE_PROJECT_TEMPLATE,
};
use crate::fs;
use crate::{cli::ProjectType, unity_project::UnityProject};

pub struct ProjectContext<'a> {
    pub template: ProjectType,
    pub project_name: &'a str,
    pub company: &'a str,
    pub email: &'a str,
    pub author: &'a str,
    pub year: i32,
}

pub fn new_project(
    ctx: &ProjectContext,
    unity_project: &UnityProject,
) -> Result<(), Box<dyn std::error::Error>> {
    match ctx.template {
        ProjectType::Game => create_from_template(&ctx, &unity_project, &GAME_PROJECT_TEMPLATE)?,
        ProjectType::Package => {
            create_from_template(&ctx, &unity_project, &PACKAGE_PROJECT_TEMPLATE)?
        }
    }
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
            author => ctx.author,
            year => ctx.year,
        ),
    )
}
