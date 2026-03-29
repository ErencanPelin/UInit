use anyhow::Ok;

use crate::new_project::{add_package, get_project_packages};
use crate::project_context::ProjectContext;
use crate::reporter::Reporter;
use crate::{config::UinitConfig, constants::PROJECT_TEMPLATES, unity_project::UnityProject};

pub fn handle_doctor(
    unity_project: &UnityProject,
    reporter: &Reporter,
    fix: bool,
) -> anyhow::Result<()> {
    println!(
        "🚀 Uinit: Running doctor with auto-fix set to '{}' ...",
        fix
    );

    reporter.info("Loading uinit.toml config file");
    let config = UinitConfig::load(&unity_project.root)?;
    let ctx = ProjectContext::from_config(&config);

    // 2. Run Checks
    // We collect them into a list so we can iterate and report uniformly
    let results = [
        (
            "Project Settings",
            validate_project_settings(&ctx, unity_project, reporter, fix)?,
        ),
        (
            "Project Structure",
            validate_project_structure(&ctx, unity_project, reporter, fix)?,
        ),
    ];

    let mut total_issues = 0;

    // 3. Centralized Reporting
    println!("\nSummary:\n");
    for (name, issues) in results {
        if issues.is_empty() {
            reporter.success(&format!("{} is healthy.", name));
        } else {
            total_issues += issues.len();
            for issue in issues {
                println!("{}.", issue);
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
    reporter: &Reporter,
    apply_fix: bool,
) -> anyhow::Result<Vec<String>> {
    let mut result = Vec::new();

    reporter.info("Checking current template from uinit.toml");
    let template = PROJECT_TEMPLATES
        .iter()
        .find(|(name, _, _)| *name == ctx.template_alias)
        .ok_or_else(|| anyhow::anyhow!("Unknown template alias: {}", ctx.template_alias))?;

    let (_, paths, _dependencies) = template;

    // 2. Trawl through paths
    reporter.info("Checking to make sure all paths from template exist.");
    for path_template in *paths {
        // Replace {} with project name (e.g., "Assets/MyGame/Scripts/")
        let relative_path = path_template.replace("{}", &ctx.project_name);
        let full_path = unity_project.root.join(&relative_path);
        reporter.info(&format!("Checking: {:?}.", full_path));

        if relative_path.ends_with('/') {
            // Check Directory
            if !full_path.is_dir() {
                if apply_fix {
                    std::fs::create_dir_all(&full_path)?;
                    result.push(format!("  ✅ Created missing directory: {}", relative_path));
                } else {
                    reporter.info(&format!("Creating: {:?}.", full_path));
                    result.push(format!("  ⚠️  Missing directory: {}", relative_path));
                }
            }
        } else {
            // Check Files (README, etc.)
            if !full_path.is_file() {
                if apply_fix {
                    std::fs::write(&full_path, "")?; // Or render a template
                    result.push(format!("  ✅ Created missing file: {}", relative_path));
                } else {
                    result.push(format!("  ⚠️  Missing file: {}", relative_path));
                }
            }
        }
    }

    // Check to see if dependencies match template
    reporter.info("Validating project contains dependencies from template.");
    let project_deps = get_project_packages(&unity_project, &reporter)?;
    let template_deps = _dependencies.to_vec();

    for dep in template_deps {
        if !project_deps.contains_key(dep.0) {
            if apply_fix {
                add_package(&unity_project, &reporter, dep.0, dep.1)?;
            } else {
                result.push(format!("  ⚠️  Missing package dependency: {}", dep.0));
            }
        }
    }

    Ok(result)
}

fn validate_project_settings(
    ctx: &ProjectContext,
    unity_project: &UnityProject,
    reporter: &Reporter,
    apply_fix: bool,
) -> anyhow::Result<Vec<String>> {
    let mut result = Vec::new();
    reporter.info("Checking if project settings match uinit.toml");

    reporter.info("Reading ProjectSettings.asset.");
    let settings_path = unity_project
        .project_settings_dir()
        .join("ProjectSettings.asset");

    let content = std::fs::read_to_string(&settings_path)?;
    let mut settings: serde_yaml::Value = serde_yaml::from_str(&content)?;

    let player = settings
        .get_mut("PlayerSettings")
        .ok_or_else(|| anyhow::anyhow!("'PlayerSettings' block missing"))?;

    let configuration_map = [
        ("companyName", &ctx.company, "Company Name"),
        ("productName", &ctx.project_name, "Product Name"),
    ];

    let mut needs_save = false;

    for (key, expected, label) in configuration_map {
        reporter.info(&format!("Checking if {} is {}.", key, expected));
        let actual = player.get(key).and_then(|v| v.as_str()).unwrap_or("");

        if actual != *expected {
            if apply_fix {
                if let Some(val) = player.get_mut(key) {
                    *val = serde_yaml::Value::String(expected.to_string());
                    needs_save = true;
                }
                reporter.info(&format!("Changed {} to {}.", key, expected));
                result.push(format!("  ✅ Fixed {}: updated to '{}'", label, expected));
            } else {
                result.push(format!(
                    "  ⚠️  Mismatch in {}: expected '{}', but found '{}'",
                    label, expected, actual
                ));
            }
        }
    }

    if apply_fix && needs_save {
        reporter.info("Saving updated ProjectSettings.asset to disk.");
        let output = serde_yaml::to_string(&settings)?;
        std::fs::write(&settings_path, output)?;
    }

    Ok(result)
}
