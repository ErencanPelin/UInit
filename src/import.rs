use anyhow::Context;
use minijinja::Environment;
use std::path::PathBuf;
use std::{path::Path, process::Command};

use crate::cli::AddTarget;
use crate::embedded_resources::{CoreModuleAssets, ToolsModuleAssets};
use crate::{
    alias::{AliasRegistry, EmbeddedModule},
    constants::{self},
    feature::create_assembly_definition,
    fs::FileSystem,
    new_project::add_package,
    project_context::ProjectContext,
    reporter::Reporter,
    unity_project::UnityProject,
};

pub fn handle_add(
    target: &AddTarget,
    path: &Option<String>,
    unity_project: &UnityProject,
    project_context: &ProjectContext,
    alias_registry: &AliasRegistry,
    reporter: &Reporter,
    fs: &FileSystem,
) -> anyhow::Result<()> {
    if let Some(name) = &target.bundle {
        reporter.info("Adding bundle...");
        let bundle = alias_registry.resolve_bundle(name).ok_or_else(|| {
            anyhow::anyhow!("Bundle '{}' not found. Run 'uinit alias list'.", name)
        })?;
        for dep in &bundle.dependencies {
            add_package(unity_project, reporter, fs, &dep.name, &dep.version)?;
        }

        reporter.success(&format!(
            "Successfully added all dependencies in bundle '{}'",
            name
        ));
        return Ok(());
    }

    reporter.info("Adding embedded module...");
    let name = target
        .module
        .as_ref()
        .expect("ArgGroup guarantees one of bundle/module");

    let source = &alias_registry
        .resolve_module(name)
        .ok_or_else(|| anyhow::anyhow!("Module '{}' not found. Run 'uinit alias list'.", name))?;

    let local_path = path.as_ref().map(PathBuf::from).unwrap_or_else(|| {
        unity_project
            .assets_dir()
            .join(&project_context.project_name)
            .join("Scripts")
            .join(source.folder_name())
    });

    reporter.info(&format!(
        "Copying embedded files to destination: {:?}",
        &local_path
    ));

    match source {
        EmbeddedModule::Core => write_embedded_files::<CoreModuleAssets>(&local_path, fs)?,
        EmbeddedModule::Tools => write_embedded_files::<ToolsModuleAssets>(&local_path, fs)?,
    }

    reporter.info("Creating assembly definition file if one does not already exist");
    ensure_assembly(
        &local_path,
        &project_context,
        reporter,
        fs,
        &Environment::new(),
    )?;

    reporter.success(&format!(
        "Successfully added embedded module '{}' to project at {:?}",
        name, &local_path
    ));

    Ok(())
}

pub fn handle_import(
    url: &str,
    path: &str,
    reporter: &Reporter,
    fs: &FileSystem,
) -> anyhow::Result<()> {
    let target = parse_github_url(url)?;
    let dest = PathBuf::from(path);

    let confirmation = reporter.prompt(&format!(
        "This will import from '{}' into '{}'.\nAre you sure?",
        url, path
    ));
    if !confirmation {
        return Ok(());
    }

    match target {
        GitHubTarget::Directory {
            repo_url,
            branch,
            path_in_repo,
        } => fetch_directory(reporter, fs, &repo_url, &branch, &path_in_repo, &dest)?,
        GitHubTarget::File {
            repo_url,
            branch,
            path_in_repo,
        } => fetch_file(reporter, fs, &repo_url, &branch, &path_in_repo, &dest)?,
    }

    reporter.success(&format!("Successfully imported into '{:?}'.", dest));
    Ok(())
}

fn ensure_assembly(
    local_path: &Path,
    ctx: &ProjectContext,
    reporter: &Reporter,
    fs: &FileSystem,
    env: &Environment,
) -> anyhow::Result<()> {
    let already_has_asmdef = std::fs::read_dir(local_path)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .any(|e| e.path().extension().is_some_and(|ext| ext == "asmdef"))
        })
        .unwrap_or(false);
    if !already_has_asmdef {
        let assembly_name = local_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(&ctx.project_name);
        create_assembly_definition(
            local_path,
            constants::ASSEMBLY_DEF_RUNTIME_JINJA,
            ctx,
            reporter,
            fs,
            "runtime",
            assembly_name,
            None,
            env,
        )?;
    }
    Ok(())
}

fn write_embedded_files<T: rust_embed::RustEmbed>(
    dest_dir: &Path,
    fs: &FileSystem,
) -> anyhow::Result<()> {
    fs.create_dirs(dest_dir)?;
    for file_path in T::iter() {
        let file = T::get(&file_path)
            .ok_or_else(|| anyhow::anyhow!("embedded file '{}' listed but not found", file_path))?;
        let dest_file = dest_dir.join(file_path.as_ref());
        if let Some(parent) = dest_file.parent() {
            fs.create_dirs(parent)?;
        }
        let content = std::str::from_utf8(&file.data)
            .with_context(|| format!("Embedded file '{}' is not valid UTF-8", file_path))?
            .to_string();
        fs.write_to_file(&content, &dest_file)?;
    }
    Ok(())
}

pub enum GitHubTarget {
    Directory {
        repo_url: String,
        branch: String,
        path_in_repo: String,
    },
    File {
        repo_url: String,
        branch: String,
        path_in_repo: String,
    },
}

pub fn parse_github_url(input: &str) -> anyhow::Result<GitHubTarget> {
    let cleaned = input.trim().trim_end_matches('/');
    let cleaned = cleaned.split(['?', '#']).next().unwrap_or(cleaned);

    let rest = cleaned
        .strip_prefix("https://github.com/")
        .ok_or_else(|| anyhow::anyhow!("Expected a github.com URL, got '{}'", input))?;
    let mut segments = rest.split('/');

    let owner = segments
        .next()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| anyhow::anyhow!("Could not find a repo owner in '{}'", input))?;
    let repo = segments
        .next()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| anyhow::anyhow!("Could not find a repo name in '{}'", input))?
        .trim_end_matches(".git");
    let repo_url = format!("https://github.com/{}/{}", owner, repo);

    match segments.next() {
        None => Ok(GitHubTarget::Directory {
            repo_url,
            branch: "HEAD".to_string(),
            path_in_repo: String::new(),
        }),
        Some(kind @ ("tree" | "blob")) => {
            let branch = segments
                .next()
                .ok_or_else(|| anyhow::anyhow!("Could not find a branch in '{}'", input))?
                .to_string();
            let path_in_repo = segments.collect::<Vec<_>>().join("/");
            if path_in_repo.is_empty() {
                anyhow::bail!("Expected a path after the branch in '{}'", input);
            }
            Ok(if kind == "blob" {
                GitHubTarget::File {
                    repo_url,
                    branch,
                    path_in_repo,
                }
            } else {
                GitHubTarget::Directory {
                    repo_url,
                    branch,
                    path_in_repo,
                }
            })
        }
        Some(other) => anyhow::bail!("Unrecognized URL segment '{}' in '{}'", other, input),
    }
}

fn fetch_directory(
    reporter: &Reporter,
    fs: &FileSystem,
    repo: &str,
    branch: &str,
    remote_folder_path: &str,
    local_dest_path: &Path,
) -> anyhow::Result<()> {
    reporter.info("Creating temporary directory for git pull");
    let temp_dir: &Path = Path::new(".uinit_temp");
    if temp_dir.exists() {
        fs.remove_dir_recursive(temp_dir)?;
    }

    reporter.info("Git: Initialising git in temporary directory");
    Command::new("git").arg("init").arg(temp_dir).output()?;
    let cmd_dir = temp_dir;
    reporter.info("Git: Adding repo origin as remote");
    Command::new("git")
        .current_dir(cmd_dir)
        .args(["remote", "add", "origin", repo])
        .output()?;
    Command::new("git")
        .current_dir(cmd_dir)
        .args(["sparse-checkout", "init", "--cone"])
        .output()?;
    Command::new("git")
        .current_dir(cmd_dir)
        .args(["sparse-checkout", "set", remote_folder_path])
        .output()?;

    reporter.info("Pulling directory with sparse checkout");
    let pull_status = Command::new("git")
        .current_dir(cmd_dir)
        .args(["pull", "--depth", "1", "origin", branch])
        .status()?;
    if !pull_status.success() {
        anyhow::bail!(
            "Git pull failed for branch '{}'. Check the URL and branch name.",
            branch
        );
    }

    let downloaded_path = cmd_dir.join(remote_folder_path);
    if downloaded_path.exists() {
        reporter.info("Copying pulled files into target destination path");
        fs.copy_dir_recursive(&downloaded_path, local_dest_path)?;
    } else {
        anyhow::bail!(
            "Path '{}' not found on branch '{}' of {}.",
            remote_folder_path,
            branch,
            repo
        );
    }

    reporter.info("Cleaning up temporary directory");
    fs.remove_dir_recursive(temp_dir)?;
    Ok(())
}

fn fetch_file(
    reporter: &Reporter,
    fs: &FileSystem,
    repo: &str,
    branch: &str,
    remote_file_path: &str,
    local_dest_path: &Path,
) -> anyhow::Result<()> {
    reporter.info("Creating temporary directory for git pull");
    let temp_dir: &Path = Path::new(".uinit_temp");
    if temp_dir.exists() {
        fs.remove_dir_recursive(temp_dir)?;
    }

    reporter.info("Git: Initialising git in temporary directory");
    Command::new("git").arg("init").arg(temp_dir).output()?;
    let cmd_dir = temp_dir;
    reporter.info("Git: Adding repo origin as remote");
    Command::new("git")
        .current_dir(cmd_dir)
        .args(["remote", "add", "origin", repo])
        .output()?;
    Command::new("git")
        .current_dir(cmd_dir)
        .args(["sparse-checkout", "init", "--no-cone"])
        .output()?;
    Command::new("git")
        .current_dir(cmd_dir)
        .args(["sparse-checkout", "set", remote_file_path])
        .output()?;

    let pull_status = Command::new("git")
        .current_dir(cmd_dir)
        .args(["pull", "--depth", "1", "origin", branch])
        .status()?;
    if !pull_status.success() {
        anyhow::bail!(
            "Git pull failed for branch '{}'. Check the URL and branch name.",
            branch
        );
    }

    let file_name = Path::new(remote_file_path)
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("Invalid remote file path: {}", remote_file_path))?;

    let downloaded_file = cmd_dir.join(remote_file_path);
    reporter.info("Ensuring destination path exists");
    fs.create_dirs(local_dest_path)?;
    let target_path = local_dest_path.join(file_name); // file lands inside the given directory, keeps its name

    reporter.info("Copying pulled files into target destination path");
    fs.copy_file(&downloaded_file, &target_path)?;

    reporter.info("Cleaning up temporary directory");
    fs.remove_dir_recursive(temp_dir)?;
    Ok(())
}
