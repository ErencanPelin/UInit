use chrono::Datelike;
use clap::Parser;

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
use cli::{Cli, Commands, FeatureActions, SteamActions};

use crate::{
    cli::{AliasActions, ProjectActions},
    constants::{DEFAULT_COMPANY, DEFAULT_EMAIL},
    new_project::{ProjectContext, init_project},
    unity_project::UnityProject,
};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let unity_project = UnityProject::detect()?;
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
                    template_alias: template.as_str(),
                    project_name: name.as_str(),
                    company: company.as_deref().unwrap_or_else(|| DEFAULT_COMPANY),
                    email: email.as_deref().unwrap_or_else(|| DEFAULT_EMAIL),
                    year: chrono::Utc::now().year(),
                };
                init_project(&ctx, &unity_project)?;
            }
        },
        Commands::Steam { action } => match action {
            SteamActions::Init { app_id } => {
                let ctx = steam::SteamContext { app_id: *app_id };
                steam::init_steam(&ctx, &unity_project)?;
            }
        },
        Commands::Ci { action: _ } => {}
        Commands::Feature { action } => match action {
            FeatureActions::Create { name } => {
                feature::init_feature(name, &unity_project)?;
            }
        },
        Commands::Add { alias } => {
            add::handle_add(alias, &unity_project)?;
        }
        Commands::Alias { action } => match action {
            AliasActions::List {} => alias::list_aliases(&unity_project)?,
            AliasActions::Add {
                alias,
                repo,
                path,
                alias_type,
            } => alias::add_alias(&alias, &repo, &path, &alias_type, &unity_project)?,
            AliasActions::Rm { alias } => alias::remove_alias(&alias, &unity_project)?,
        },
    }

    Ok(())
}
