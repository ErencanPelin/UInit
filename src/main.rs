use clap::Parser;
use minijinja::Environment;
use std::error::Error;
use std::path::PathBuf;

mod args;
mod constants;
mod contexts;
mod fs;
use std::{env, path::Path};

use args::{Cli, ProjectType};
use constants::{GAME_PROJECT_TEMPLATE, PACKAGE_PROJECT_TEMPLATE, PACKAGE_TEMPLATE_JSON};
use contexts::ProjectContext;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let current_directory: PathBuf = env::current_dir()?;
    fs::validate_current_directory(&current_directory)?;

    let env = Environment::new();
    let ctx = contexts::get_project_context(&cli);

    // Create the project folder in /Assets with underscore prefix
    let project_path: PathBuf = current_directory.join("_").join(&cli.project_name);
    fs::create_directory(&project_path)?;

    match cli.project_type {
        ProjectType::Game => init_game(&project_path)?,
        ProjectType::Package => init_package(&project_path, &env, &ctx)?,
    }

    fs::write_common_files(&project_path, &env, &ctx)?;

    Ok(())
}

fn create_template(base_path: &Path, template: &'static [&str]) -> std::io::Result<()> {
    for path_str in template {
        let path = base_path.join(path_str.strip_prefix("/").unwrap_or_default());
        if path_str.ends_with("/") {
            fs::create_directory(&path)?;
        } else {
            fs::create_file(&path)?;
        }
    }
    Ok(())
}

fn init_package(
    project_path: &Path,
    env: &Environment,
    ctx: &ProjectContext,
) -> std::io::Result<()> {
    create_template(&project_path, &PACKAGE_PROJECT_TEMPLATE)?;

    let package_json_path = project_path.join("package.json");
    let rendered_package_json = fs::render_template(&env, PACKAGE_TEMPLATE_JSON, ctx)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    std::fs::write(package_json_path, rendered_package_json)?;

    Ok(())
}

fn init_game(project_path: &Path) -> std::io::Result<()> {
    create_template(&project_path, &GAME_PROJECT_TEMPLATE)?;

    Ok(())
}
