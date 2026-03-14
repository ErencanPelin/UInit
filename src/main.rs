use minijinja::{Environment, context};
use std::env;
use std::error::Error;
use std::io;
use std::path::PathBuf;

mod constants;
use crate::constants::{GAME_PROJECT_TEMPLATE, PACKAGE_PROJECT_TEMPLATE, PACKAGE_TEMPLATE_JSON};

fn validate_current_directory(current_directory: &PathBuf) -> Result<(), Box<dyn Error>> {
    let current_directory_name = current_directory.file_name().unwrap_or_default();
    if current_directory_name != "Assets" {
        panic!(
            "Current directory is not 'Assets'. Please navigate to the 'Assets' directory and try again."
        );
    }

    Ok(())
}

fn ask(question: &str) -> Result<String, Box<dyn Error>> {
    println!("{}", question);
    let mut answer = String::new();
    io::stdin().read_line(&mut answer)?;
    Ok(answer.trim().to_string())
}

fn main() -> Result<(), Box<dyn Error>> {
    let current_directory: PathBuf = env::current_dir()?;
    validate_current_directory(&current_directory)?;

    let project_type =
        ask("What type of project are you working on? (e.g., 'Game', 'Package')")?.to_lowercase();

    let project_name = ask(format!("What is the name of your {}?", project_type).as_str())?;

    // Create the project folder in /Assets with underscore prefix
    let mut project_path: PathBuf = PathBuf::new();
    project_path.push(current_directory);
    project_path.push(format!("_{}", project_name));
    create_directory(&project_path)?;

    match project_type.as_str() {
        "game" => {
            create_template_directories(&project_path, &GAME_PROJECT_TEMPLATE)?;
        }
        "package" => {
            create_template_directories(&project_path, &PACKAGE_PROJECT_TEMPLATE)?;
            let env = Environment::new();
            let rendered = env.render_str(
                PACKAGE_TEMPLATE_JSON,
                context!(
                   package_name => project_name,
                   owner => constants::OWNER,
                   owner_email => constants::OWNER_EMAIL,
                ),
            )?;
            let mut package_json_path = project_path.clone();
            package_json_path.push("package.json");
            std::fs::write(package_json_path, rendered)?;
        }
        _ => {
            panic!("Invalid project type. Please enter 'game' or 'package'.");
        }
    }

    Ok(())
}

fn create_template_directories(
    base_path: &PathBuf,
    template: &'static [&str],
) -> std::io::Result<()> {
    for path_str in template {
        let mut path = base_path.clone();
        path.push(path_str.strip_prefix("/").unwrap_or_default());
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
