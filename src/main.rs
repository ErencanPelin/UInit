use chrono::Datelike;
use clap::Parser;
use minijinja::{Environment, context};
use std::error::Error;
use std::path::PathBuf;
use std::{env, path::Path};

mod args;
mod constants;

use crate::args::{Cli, ProjectType};
use crate::constants::{
    GAME_PROJECT_TEMPLATE, LICENSE_CONTENT, PACKAGE_PROJECT_TEMPLATE, PACKAGE_TEMPLATE_JSON,
};

struct ProjectContext<'a> {
    project_name: &'a str,
    company: &'a str,
    email: &'a str,
    author: &'a str,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let _env = Environment::new();

    let current_directory: PathBuf = env::current_dir()?;
    validate_current_directory(&current_directory)?;

    let env = Environment::new();
    let ctx = ProjectContext {
        project_name: cli.project_name.as_str(),
        company: cli.company.as_deref().unwrap_or_else(|| constants::COMPANY),
        email: cli.email.as_deref().unwrap_or_else(|| constants::EMAIL),
        author: cli.author.as_deref().unwrap_or_else(|| constants::AUTHOR),
    };

    // Create the project folder in /Assets with underscore prefix
    let project_path: PathBuf = current_directory.join(format!("_{}", cli.project_name));
    create_directory(&project_path)?;

    match cli.project_type {
        ProjectType::Game => init_game(&project_path)?,
        ProjectType::Package => init_package(&project_path, &env, &ctx)?,
    }

    write_common_files(&project_path, &env, &ctx)?;

    Ok(())
}

fn validate_current_directory(current_directory: &Path) -> Result<(), String> {
    match current_directory.file_name().and_then(|n| n.to_str()) {
        Some("Assets") => Ok(()),
        _ => Err("Please run this from inside the `Assets` directory.".into()),
    }
}

fn create_template(base_path: &Path, template: &'static [&str]) -> std::io::Result<()> {
    for path_str in template {
        let path = base_path.join(path_str.strip_prefix("/").unwrap_or_default());
        if path_str.ends_with("/") {
            create_directory(&path)?;
        } else {
            create_file(&path)?;
        }
    }
    Ok(())
}

fn create_directory(path: &PathBuf) -> std::io::Result<()> {
    std::fs::create_dir_all(path)?;
    Ok(())
}

fn create_file(path: &PathBuf) -> std::io::Result<()> {
    std::fs::File::create(path)?;
    Ok(())
}

fn init_package(
    project_path: &Path,
    env: &Environment,
    ctx: &ProjectContext,
) -> std::io::Result<()> {
    create_template(&project_path, &PACKAGE_PROJECT_TEMPLATE)?;

    let package_json_path = project_path.join("package.json");
    let rendered_package_json = render_template(&env, PACKAGE_TEMPLATE_JSON, ctx)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    std::fs::write(package_json_path, rendered_package_json)?;

    Ok(())
}

fn init_game(project_path: &Path) -> std::io::Result<()> {
    create_template(&project_path, &GAME_PROJECT_TEMPLATE)?;

    Ok(())
}

fn write_common_files(
    project_path: &Path,
    env: &Environment,
    ctx: &ProjectContext,
) -> std::io::Result<()> {
    let license_path = project_path.join("LICENSE");
    let rendered_license = render_template(&env, LICENSE_CONTENT, ctx)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    std::fs::write(&license_path, rendered_license)?;

    let gitignore_path = project_path.join(".gitignore");
    std::fs::write(&gitignore_path, constants::GITIGNORE_CONTENT)?;

    Ok(())
}

fn render_template(
    env: &Environment,
    template_source: &str,
    ctx: &ProjectContext,
) -> Result<String, minijinja::Error> {
    env.render_str(
        template_source,
        context!(
            project_name => ctx.project_name,
            company => ctx.company,
            email => ctx.email,
            author => ctx.author,
            year => chrono::Utc::now().year(),
        ),
    )
}
