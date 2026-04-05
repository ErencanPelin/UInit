use chrono::Datelike;
use clap::Parser;

mod alias_registry;
mod ci;
mod cli;
mod config;
mod constants;
mod doctor;
mod enums;
mod feature;
mod fs;
mod import;
mod new_project;
mod project_context;
mod remotes;
mod reporter;
mod steam;
mod unity_project;
mod version;
use crate::{
    ci::handle_ci,
    cli::{Cli, Commands, Integration, RemotesActions},
    constants::{DEFAULT_COMPANY, DEFAULT_EMAIL},
    doctor::handle_doctor,
    new_project::init_project,
    project_context::ProjectContext,
    reporter::Reporter,
    unity_project::UnityProject,
};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let reporter = Reporter::new(cli.verbose, cli.no_prompts);

    let unity_project = UnityProject::detect()?;

    match &cli.command {
        Commands::Init {
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
        Commands::Setup(args) => match &args.integration {
            Integration::Steam { app_id } => {
                let ctx = steam::SteamContext { app_id: *app_id };
                steam::init_steam(&ctx, &unity_project, &reporter)?;
            }
            Integration::Ci { host, name } => handle_ci(&host, &name, &unity_project, &reporter)?,
        },
        Commands::Gen {
            name,
            no_editor,
            no_tests,
        } => {
            feature::init_feature(name, *no_editor, *no_tests, &unity_project, &reporter)?;
        }
        Commands::Import { alias, path } => {
            import::handle_import(alias, &path, &unity_project, &reporter)?;
        }
        Commands::Remote { action } => match action {
            RemotesActions::List {} => remotes::list_aliases(&unity_project, &reporter)?,
            RemotesActions::Add {
                alias,
                repo,
                path,
                category: alias_type,
            } => remotes::add_alias(&alias, &repo, &path, &alias_type, &unity_project, &reporter)?,
            RemotesActions::Remove { alias } => {
                remotes::remove_alias(&alias, &unity_project, &reporter)?
            }
        },
        Commands::Doctor { fix } => handle_doctor(&unity_project, &reporter, *fix)?,
    }

    version::check_for_updates(&reporter)?;

    Ok(())
}
