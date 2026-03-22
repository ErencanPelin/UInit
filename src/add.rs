use anyhow::Context;
use minijinja::Environment;
use std::{collections::HashMap, path::Path, process::Command};

use crate::project_context::ProjectContext;
use crate::reporter::Reporter;
use crate::{
    alias,
    config::{AliasEntry, UinitConfig},
    constants::{self},
    feature::create_assembly_definition,
    fs,
    unity_project::UnityProject,
};

pub fn handle_add(
    alias: &str,
    unity_project: &UnityProject,
    reporter: &Reporter,
) -> anyhow::Result<()> {
    let config = UinitConfig::load(&unity_project.root)?;
    let ctx = ProjectContext::from_config(&config);
    let aliases: HashMap<String, AliasEntry> = alias::get_aliases(&config);

    if let Some(alias_entry) = aliases.get(alias) {
        println!(
            "Adding '{}' from repo '{}' at path '{}'",
            alias, alias_entry.repo, alias_entry.path
        );

        match alias_entry.alias_type.to_lowercase().as_str() {
            "util" => import_util(&ctx, &unity_project, &reporter, &alias_entry)?,
            "module" => import_module(&ctx, &unity_project, &reporter, &alias_entry)?,
            "tool" => import_tool(&unity_project, &reporter, &alias_entry)?,
            _ => {}
        }

        reporter.success(&format!(
            "Successfully added {} '{}'",
            alias_entry.alias_type, alias
        ));
    } else {
        anyhow::bail!(
            "Alias '{}' not found in configuration. Check your 'uinit.toml' or use 'uinit alias list' to see available aliases.",
            alias
        )
    }

    Ok(())
}

fn import_tool(
    unity_project: &UnityProject,
    reporter: &Reporter,
    alias_entry: &AliasEntry,
) -> anyhow::Result<()> {
    let local_path = &unity_project.root.join("Tools");

    fetch_file(&reporter, &alias_entry.repo, &alias_entry.path, &local_path)?;

    Ok(())
}

fn import_module(
    ctx: &ProjectContext,
    unity_project: &UnityProject,
    reporter: &Reporter,
    alias_entry: &AliasEntry,
) -> anyhow::Result<()> {
    // fetch and move into the project
    let local_path = &unity_project
        .assets_dir()
        .join(&ctx.project_name)
        .join("Scripts");

    fetch_directory(&reporter, &alias_entry.repo, &alias_entry.path, &local_path)?;

    Ok(())
}

fn import_util(
    ctx: &ProjectContext,
    unity_project: &UnityProject,
    reporter: &Reporter,
    alias_entry: &AliasEntry,
) -> anyhow::Result<()> {
    // fetch and move into the project
    let local_path = &unity_project
        .assets_dir()
        .join(&ctx.project_name)
        .join("Scripts/Utils");

    fetch_directory(&reporter, &alias_entry.repo, &alias_entry.path, &local_path)?;

    // create assembly definition for the utils folder if it doesn't exist
    let assembly_name_file_name =
        format!("com.{}.{}.utils.asmdef", ctx.company, ctx.project_name).to_lowercase();

    let assembly_path = &local_path.join(&assembly_name_file_name);
    if !assembly_path.exists() {
        create_assembly_definition(
            &local_path,
            constants::ASSEMBLY_DEF_RUNTIME_JINJA,
            &ctx,
            reporter,
            "runtime",
            "utils",
            None,
            &Environment::new(),
        )?;
    }

    Ok(())
}

fn fetch_directory(
    reporter: &Reporter,
    repo: &str,
    remote_folder_path: &str,
    local_dest_path: &Path,
) -> anyhow::Result<()> {
    let temp_dir = ".uinit_temp";

    // FIXME: we should always clean up the temp dir if a failure occurs anywhere in this function
    // cleanup old temp dir if it still exists (e.g. a mid failed process)
    reporter.info("Checking if temporary directory already exists.");
    if Path::new(temp_dir).exists() {
        std::fs::remove_dir_all(temp_dir)?;
    }

    // Initialize and add remote to a temp directory
    // We pull the repo into the temp directory then move files to the correct destination in the project
    reporter.info("Initialising new temp git repo.");
    Command::new("git").args(["init", temp_dir]).output()?;
    let cmd_dir = Path::new(temp_dir);

    reporter.info("Writing git config.");
    Command::new("git")
        .current_dir(cmd_dir)
        .args(["remote", "add", "origin", repo])
        .output()?;

    // Enable sparse-checkout
    reporter.info("Enabling sparse checkout.");
    Command::new("git")
        .current_dir(cmd_dir)
        .args(["sparse-checkout", "init", "--cone"])
        .output()?;

    // Set the specific path to fetch so that we don't pull the whole repo. We need sparse-checkout for this
    // Note: remote_folder_path must match the repo root structure exactly
    reporter.info("Setting sparse-checkout remote path.");
    Command::new("git")
        .current_dir(cmd_dir)
        .args(["sparse-checkout", "set", remote_folder_path])
        .output()?;

    // Pull using HEAD to auto-detect main/master
    reporter.info("Downloading files from from git remote...");
    let pull_status = Command::new("git")
        .current_dir(cmd_dir)
        .args(["pull", "--depth", "1", "origin", "HEAD"])
        .status()?;

    if !pull_status.success() {
        anyhow::bail!("Git pull failed. Check your internet connection or repository URL.");
    }

    // Copy to the correct path in the project files
    let downloaded_path = cmd_dir.join(remote_folder_path);
    if downloaded_path.exists() && downloaded_path.is_dir() {
        let folder_name = Path::new(remote_folder_path)
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("Invalid remote path"))?;

        let final_local_path = local_dest_path.join(folder_name);

        reporter.info("Copying pulled files intto the project.");
        fs::copy_dir_recursive(&downloaded_path, &final_local_path)?;
    } else {
        // Debug: List files to see what Git actually pulled
        reporter.info("Oops, looks like git didn't pull everything correctly.");
        let entries = std::fs::read_dir(cmd_dir)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()?;

        entries.iter().for_each(|f| {
            reporter.info(&format!("Pulled {:?}.", f));
        });

        anyhow::bail!(
            "Directory '{}' not found in downloaded content. Found: {:?}",
            remote_folder_path,
            entries
        );
    }

    // cleanup the temp directory
    reporter.info("Cleanup: Deleting temporary git repo.");
    std::fs::remove_dir_all(temp_dir)?;
    Ok(())
}

fn fetch_file(
    reporter: &Reporter,
    repo: &str,
    remote_file_path: &str,
    local_dest_dir: &Path,
) -> anyhow::Result<()> {
    let temp_dir = ".uinit_temp";

    reporter.info("Checking if temporary directory already exists.");
    if Path::new(temp_dir).exists() {
        std::fs::remove_dir_all(temp_dir).context("Failed to clean up old temp directory")?;
    }

    reporter.info("Initialising new temp git repo.");
    Command::new("git")
        .args(["init", temp_dir])
        .output()
        .context("Failed to init git")?;

    let cmd_dir = Path::new(temp_dir);
    let run_git = |args: &[&str]| {
        Command::new("git")
            .current_dir(cmd_dir)
            .args(args)
            .output()
            .with_context(|| format!("Git command failed: git {:?}", args))
    };

    run_git(&["remote", "add", "origin", repo])?;
    run_git(&["config", "core.sparseCheckout", "true"])?;

    reporter.info("Initialising sparse-checkout new temp git repo.");
    let sparse_info = cmd_dir.join(".git/info/sparse-checkout");
    std::fs::write(sparse_info, format!("{}\n", remote_file_path))?;

    reporter.info("Downloading files from git remote...");
    run_git(&["pull", "--depth", "1", "origin", "HEAD"])?;

    reporter.info("Making sure file exists locally.");
    let file_name = Path::new(remote_file_path)
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("Invalid remote file path: {}", remote_file_path))?;

    let downloaded_file = cmd_dir.join(remote_file_path);
    let target_path = local_dest_dir.join(file_name);

    reporter.info("Making sure file exists locally.");
    if downloaded_file.is_file() {
        reporter.info("Creating target directory inside project.");
        std::fs::create_dir_all(local_dest_dir)
            .with_context(|| format!("Failed to create directory: {}", local_dest_dir.display()))?;

        reporter.info("Cpying downloading file into target directory inside project.");
        std::fs::copy(&downloaded_file, &target_path)
            .with_context(|| format!("Failed to copy file to {}", target_path.display()))?;

        reporter.success(&format!("Successfully imported: {}", target_path.display()));
    } else {
        anyhow::bail!("File not found in repository at path: {}", remote_file_path);
    }

    reporter.info("Cleanup: Deleting temporary git repo.");
    let _ = std::fs::remove_dir_all(temp_dir);
    Ok(())
}
