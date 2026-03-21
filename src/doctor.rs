use anyhow::Ok;

use crate::{config::UinitConfig, new_project::ProjectContext, unity_project::UnityProject};

pub fn handle_doctor(unity_project: &UnityProject, fix: bool) -> anyhow::Result<()> {
    let config = UinitConfig::load(&unity_project.root)?;
    let ctx = ProjectContext::from_config(&config);

    // 2. Run Checks
    // We collect them into a list so we can iterate and report uniformly
    let results = [
        (
            "Project Settings",
            validate_project_settings(&ctx, unity_project, fix)?,
        ),
        (
            "Project Structure",
            validate_project_structure(&ctx, unity_project)?,
        ),
    ];

    let mut total_issues = 0;

    // 3. Centralized Reporting
    for (name, issues) in results {
        if issues.is_empty() {
            println!("✅ {} is healthy", name);
        } else {
            total_issues += issues.len();
            for issue in issues {
                println!("{}", issue);
            }
        }
    }

    if total_issues == 0 {
        println!("\n✨ Everything looks great!");
    } else {
        println!(
            "\n❌ Found {} issues total. Run with --fix to fix.",
            total_issues
        );
    }

    Ok(())
}

fn validate_project_structure(
    ctx: &ProjectContext,
    unity_project: &UnityProject,
) -> anyhow::Result<Vec<String>> {
    let mut issues = Vec::new();
    println!("Validating project structure...");

    Ok(issues)
}

fn validate_project_settings(
    ctx: &ProjectContext,
    unity_project: &UnityProject,
    apply_fix: bool,
) -> anyhow::Result<Vec<String>> {
    let mut result = Vec::new();
    let settings_path = unity_project
        .project_settings_dir()
        .join("ProjectSettings.asset");

    let content = std::fs::read_to_string(&settings_path)?;
    let mut settings: serde_yaml::Value = serde_yaml::from_str(&content)?;

    // Use a scope or a separate block to get a mutable handle to PlayerSettings
    let player = settings
        .get_mut("PlayerSettings")
        .ok_or_else(|| anyhow::anyhow!("'PlayerSettings' block missing"))?;

    let configuration_map = [
        ("companyName", &ctx.company, "Company Name"),
        ("productName", &ctx.project_name, "Product Name"),
    ];

    let mut needs_save = false;

    for (key, expected, label) in configuration_map {
        // 1. Get the current value
        let actual = player.get(key).and_then(|v| v.as_str()).unwrap_or("");

        if actual != *expected {
            if apply_fix {
                // 2. Perform the actual mutation in the YAML tree
                if let Some(val) = player.get_mut(key) {
                    *val = serde_yaml::Value::String(expected.to_string());
                    needs_save = true;
                }
                result.push(format!("✅ Fixed {}: updated to '{}'", label, expected));
            } else {
                result.push(format!(
                    "⚠️ Mismatch in {}: expected '{}', but found '{}'",
                    label, expected, actual
                ));
            }
        }
    }

    // 3. Save only if we modified something and fix was requested
    if apply_fix && needs_save {
        let output = serde_yaml::to_string(&settings)?;
        std::fs::write(&settings_path, output)?;
    }

    Ok(result)
}
