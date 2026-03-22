use chrono::Datelike;
use clap::Parser;

mod add;
mod alias;
mod cli;
mod config;
mod constants;
mod doctor;
mod feature;
mod fs;
mod new_project;
mod project_context;
mod reporter;
mod steam;
mod unity_project;
mod version;
use cli::{Cli, Commands, FeatureActions, SteamActions};

use crate::project_context::ProjectContext;
use crate::reporter::Reporter;
use crate::{
    cli::{AliasActions, ProjectActions},
    constants::{DEFAULT_COMPANY, DEFAULT_EMAIL},
    doctor::handle_doctor,
    new_project::init_project,
    unity_project::UnityProject,
};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let reporter = Reporter::new(cli.verbose);

    let unity_project = UnityProject::detect()?;

    match &cli.command {
        Commands::Project { action } => match action {
            ProjectActions::Init {
                name,
                template,
                company,
                email,
            } => {
                let ctx = ProjectContext {
                    template_alias: template.to_string(),
                    project_name: name.to_string(),
                    // Clone the string if it exists, otherwise use the default
                    company: company
                        .clone()
                        .unwrap_or_else(|| DEFAULT_COMPANY.to_string()),
                    email: email.clone().unwrap_or_else(|| DEFAULT_EMAIL.to_string()),
                    year: chrono::Utc::now().year(),
                };
                init_project(&ctx, &unity_project, &reporter)?;
            }
        },
        Commands::Steam { action } => match action {
            SteamActions::Init { app_id } => {
                let ctx = steam::SteamContext { app_id: *app_id };
                steam::init_steam(&ctx, &unity_project, &reporter)?;
            }
        },
        Commands::Feature { action } => match action {
            FeatureActions::Create { name } => {
                feature::init_feature(name, &unity_project, &reporter)?;
            }
        },
        Commands::Add { alias } => {
            add::handle_add(alias, &unity_project, &reporter)?;
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
        Commands::Doctor { fix } => handle_doctor(&unity_project, &reporter, *fix)?,
    }

    version::check_for_updates(&reporter)?;

    Ok(())
}
