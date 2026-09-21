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
mod global_config;
mod import;
mod new_project;
mod project_context;
mod project_template_registry;
mod remotes;
mod reporter;
mod steam;
mod unity_project;
mod version;

use crate::{
    cli::{CiActions, Cli, Commands, RemotesActions},
    constants::{DEFAULT_COMPANY, DEFAULT_EMAIL},
    doctor::handle_doctor,
    fs::FileSystem,
    new_project::init_project,
    project_context::ProjectContext,
    project_template_registry::ProjectTemplateRegistry,
    reporter::Reporter,
    unity_project::UnityProject,
};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let reporter = Reporter::new(cli.verbose, cli.no_prompts);
    let fs = FileSystem::new(cli.dry_run);
    let project_template_registry = ProjectTemplateRegistry::load();

    if let Commands::Config { company, email } = &cli.command {
        global_config::handle_config(company.clone(), email.clone(), &reporter, &fs)?;
    } else {
        let unity_project = UnityProject::detect()?;
        let global_config = global_config::GlobalConfig::load()?;
        match &cli.command {
            Commands::Init {
                name,
                template,
                company,
                email,
            } => {
                let ctx = ProjectContext {
                    project_type: template.clone(),
                    project_name: name.to_string(),
                    // Clone the string if it exists, otherwise use the default
                    company: company
                        .clone()
                        .or_else(|| global_config.company.clone())
                        .unwrap_or_else(|| DEFAULT_COMPANY.to_string()),
                    email: email
                        .clone()
                        .or_else(|| global_config.email.clone())
                        .unwrap_or_else(|| DEFAULT_EMAIL.to_string()),
                    year: chrono::Utc::now().year(),
                };
                init_project(
                    &ctx,
                    &unity_project,
                    &reporter,
                    &fs,
                    &project_template_registry,
                )?;
            }
            Commands::Steam { app_id } => {
                let ctx = steam::SteamContext { app_id: *app_id };
                steam::init_steam(&ctx, &unity_project, &reporter, &fs)?;
            }
            Commands::Ci { action } => match action {
                CiActions::List {} => ci::list_workflows(&reporter)?,
                CiActions::Add { host, workflow } => {
                    ci::handle_add_ci_workflow(&host, &workflow, &unity_project, &reporter, &fs)?
                }
            },
            Commands::Feature {
                name,
                no_editor,
                no_tests,
            } => {
                feature::init_feature(name, *no_editor, *no_tests, &unity_project, &reporter, &fs)?;
            }
            Commands::Import { alias, path } => {
                import::handle_import(alias, &path, &unity_project, &reporter, &fs)?;
            }
            Commands::Remote { action } => match action {
                RemotesActions::List {} => remotes::list_aliases(&unity_project, &reporter)?,
                RemotesActions::Add {
                    alias,
                    repo,
                    path,
                    category,
                } => remotes::add_alias(
                    &alias,
                    &repo,
                    &path,
                    &category,
                    &unity_project,
                    &reporter,
                    &fs,
                )?,
                RemotesActions::Remove { alias } => {
                    remotes::remove_alias(&alias, &unity_project, &reporter, &fs)?
                }
            },
            Commands::Doctor { fix } => handle_doctor(
                &unity_project,
                &reporter,
                &fs,
                *fix,
                &project_template_registry,
            )?,
            Commands::Config { .. } => unreachable!("handled above"),
        }
    }

    version::check_for_updates(&reporter)?;

    Ok(())
}
