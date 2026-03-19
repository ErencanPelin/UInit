use chrono::Datelike;
use clap::Parser;
use std::error::Error;

mod add;
mod alias;
mod cli;
mod config;
mod constants;
mod feature;
mod fs;
mod new_project;
mod steam;
mod unity_project;
use cli::{CiActions, Cli, Commands, FeatureActions, SteamActions};

use crate::{
    cli::ProjectActions,
    constants::{COMPANY, EMAIL},
    new_project::{ProjectContext, new_project},
    unity_project::UnityProject,
};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let mut unity_project = UnityProject::detect()?;
    println!("Running inside: {}", unity_project.root.display());

    match &cli.command {
        Commands::Project { action } => match action {
            ProjectActions::Init {
                name,
                template,
                company,
                email,
            } => {
                let ctx = ProjectContext {
                    template: template.clone(),
                    project_name: name.as_str(),
                    company: company.as_deref().unwrap_or_else(|| COMPANY),
                    email: email.as_deref().unwrap_or_else(|| EMAIL),
                    year: chrono::Utc::now().year(),
                };
                new_project(&ctx, &unity_project)?;
            }
        },
        Commands::Steam { action } => match action {
            SteamActions::Init { app_id } => {
                let ctx = steam::SteamContext { app_id: *app_id };
                steam::init_steam(&ctx, &unity_project);
            }
        },
        Commands::Ci { action } => {}
        Commands::Feature { action } => match action {
            FeatureActions::Create { name } => {
                feature::init_feature(name, &unity_project)?;
            }
        },
        Commands::Add { alias } => {
            add::handle_add(alias, &unity_project)?;
        }
        Commands::Alias { action } => {
            alias::list_aliases(&unity_project)?;
        }
    }

    Ok(())
}
