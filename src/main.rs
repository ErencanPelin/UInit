use chrono::Datelike;
use clap::Parser;
use std::error::Error;

mod cli;
mod constants;
mod fs;
mod new_project;
mod steam;
mod unity_project;

use cli::{CiActions, Cli, Commands, SteamActions};

use crate::{
    constants::{AUTHOR, COMPANY, EMAIL},
    new_project::{ProjectContext, new_project},
    unity_project::UnityProject,
};

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let unity_project = UnityProject::detect()?;
    println!("Running inside: {}", unity_project.root.display());

    match &cli.command {
        Commands::New {
            name,
            template,
            company,
            email,
            author,
        } => {
            let ctx = ProjectContext {
                template: template.clone(),
                project_name: name.as_str(),
                company: company.as_deref().unwrap_or_else(|| COMPANY),
                email: email.as_deref().unwrap_or_else(|| EMAIL),
                author: author.as_deref().unwrap_or_else(|| AUTHOR),
                year: chrono::Utc::now().year(),
            };
            new_project(&ctx, &unity_project)?;
        }
        Commands::Steam { action } => match action {
            SteamActions::Init { app_id } => {
                let ctx = steam::SteamContext { app_id: *app_id };
                steam::init_steam(&ctx, &unity_project);
            }
        },
        Commands::Ci { action } => {}
    }

    Ok(())
}
