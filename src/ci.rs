use std::path::PathBuf;

use crate::{
    constants::WORKFLOW_TEMPATES,
    enums::{CiHost, WorkflowType},
    fs,
    reporter::Reporter,
    unity_project::UnityProject,
};

pub fn handle_ci(
    ci_host: &CiHost,
    workflow_type: &WorkflowType,
    unity_project: &UnityProject,
    reporter: &Reporter,
) -> anyhow::Result<()> {
    reporter.info(&format!("Finding templates for CI host '{}'", ci_host));
    let (_, workflows_for_host) = WORKFLOW_TEMPATES
        .iter()
        .find(|(host_type, _)| *host_type == *ci_host)
        .ok_or_else(|| anyhow::anyhow!("CI host '{}' not found.", ci_host))?;

    reporter.info(&format!(
        "Finding template for workflow '{}'",
        workflow_type
    ));

    let (_, workflow_template, file_name) = workflows_for_host
        .iter()
        .find(|(workflow_t, _, _)| *workflow_t == *workflow_type)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "CI workflow template '{}' not found for host '{}'",
                workflow_type,
                ci_host
            )
        })?;

    reporter.info("Creating required folder structure");
    let dir_path = unity_project.root.join(get_dest_path_for_ci_host(&ci_host));
    let file_path = dir_path.join(file_name);
    fs::create_dirs(&dir_path)?;

    reporter.info("Creating new file for template");
    if !fs::create_file(&file_path)? {
        // ask if we want to overwrite the file that already exists
        let confirmation = reporter.prompt(&format!(
            "A file at {:?} already exists. Do you wish to overwrite it anyway?",
            file_path
        ));

        if !confirmation {
            return Ok(());
        }
    }

    reporter.info("Writing template to file");
    fs::write_to_file(&workflow_template.to_string(), &file_path)?;

    reporter.success(&format!(
        "Created {} workflow for {}",
        workflow_type, ci_host
    ));

    println!(
        "[info] Make sure to review the created file and setup any secrets requred by it in your repo."
    );

    Ok(())
}

fn get_dest_path_for_ci_host(ci_host: &CiHost) -> PathBuf {
    return match ci_host {
        CiHost::Github => PathBuf::from(".github/workflows"),
    };
}
