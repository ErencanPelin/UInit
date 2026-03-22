use std::collections::HashMap;

use anyhow::{Context, Ok};
use minijinja::Environment;

use crate::config::UinitConfig;
use crate::constants::{
    CHANGELOG_TEMPLATE, GITIGNORE_TEMPLATE, LICENSE_JINJA, PACKAGE_JINJA, PROJECT_TEMPLATES,
    README_JINJA,
};
use crate::fs;
use crate::project_context::ProjectContext;
use crate::reporter::Reporter;
use crate::unity_project::UnityProject;

pub fn init_project(
    ctx: &ProjectContext,
    unity_project: &UnityProject,
    reporter: &Reporter,
) -> anyhow::Result<()> {
    println!(
        "🚀 Uinit: Initialising '{}' with '{}' template...",
        ctx.project_name, ctx.template_alias
    );

    create_from_template(ctx, unity_project, reporter)
        .with_context(|| format!("Failed to apply template: {}", ctx.template_alias))?;

    modify_project_settings(ctx, unity_project, reporter)
        .with_context(|| "Failed to update Unity ProjectSettings.asset.")?;

    // write config file
    reporter.info("Updating uinit.toml config file.");
    let config: UinitConfig = ctx.into();
    config
        .save(&unity_project.root)
        .with_context(|| "Failed to save uinit config to disk.")?;

    println!("\n✨ '{}' initialized successfully.", ctx.project_name);
    Ok(())
}

fn create_from_template(
    ctx: &ProjectContext,
    unity_project: &UnityProject,
    reporter: &Reporter,
) -> anyhow::Result<()> {
    reporter.info("Creating project from template.");
    let env = Environment::new();
    let template = PROJECT_TEMPLATES
        .iter()
        .find(|(name, _, _)| *name == ctx.template_alias)
        .ok_or_else(|| anyhow::anyhow!("Template '{}' not found", ctx.template_alias))?;

    let (_, paths, deps) = template;

    // Define which files get which templates
    // TODO: move to constants.rs
    let template_map = HashMap::from([
        ("CHANGELOG.md", CHANGELOG_TEMPLATE),
        (".gitignore", GITIGNORE_TEMPLATE),
        ("LICENSE", LICENSE_JINJA),
        ("package.json", PACKAGE_JINJA),
        ("README.md", README_JINJA),
    ]);

    // create each defined file and path
    for &raw_path in *paths {
        let template_path = raw_path.replace("{}", &ctx.project_name);
        let full_path = unity_project.root.join(&template_path);
        let relative_path: String = unity_project.rel_path(&full_path);

        if raw_path.ends_with('/') {
            if fs::create_dirs(&full_path)? {
                reporter.info(&format!("Creating folder {:?}", full_path));
                println!("  📁 Created: {}", relative_path);
            } else {
                reporter.info(&format!(
                    "Skipped: directory already exists at path {:?}",
                    full_path
                ));
            }
            continue;
        }

        if fs::create_file(&full_path)? {
            reporter.info(&format!("Creating file {:?}", full_path));
            println!("  📄 Created: {}", relative_path);
        }

        // Determine content
        let file_name = full_path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if let Some(&raw_template) = template_map.get(file_name) {
            let content = if raw_template.contains("{{") {
                reporter.info(&format!("Rendering jinja2 file {:?}", full_path));
                env.render_str(raw_template, ctx)
                    .with_context(|| format!("Failed to render template {}", file_name))?
            } else {
                raw_template.to_string()
            };
            reporter.info(&format!("Writing to file {:?}", full_path));
            fs::write_to_file(&content, &full_path)?;
        }
    }

    // Dependencies
    // TODO: split this into its own function
    for (pkg, ver) in *deps {
        add_package(unity_project, reporter, pkg, ver)?;
    }

    Ok(())
}

fn modify_project_settings(
    ctx: &ProjectContext,
    unity_project: &UnityProject,
    reporter: &Reporter,
) -> anyhow::Result<()> {
    let path = unity_project
        .project_settings_dir()
        .join("ProjectSettings.asset");

    reporter.info("Reading current ProjectSettings.asset.");
    let raw_yaml =
        std::fs::read_to_string(&path).with_context(|| "Failed to read ProjectSettings.asset")?;

    reporter.info("Parsing current ProjectSettings.asset from yaml.");
    let mut settings: serde_yaml::Value =
        serde_yaml::from_str(&raw_yaml).with_context(|| "Failed to parse ProjectSettings.asset")?;

    reporter.info("Updating companyName and productName in project settings.");
    if let Some(player) = settings.get_mut("PlayerSettings") {
        if let Some(c) = player.get_mut("companyName") {
            *c = ctx.company.clone().into();
        }
        if let Some(p) = player.get_mut("productName") {
            *p = ctx.project_name.clone().into();
        }
    }

    let yaml_out =
        serde_yaml::to_string(&settings).with_context(|| "Failed to serialize settings")?;

    reporter.info("Writing updated project settings back to disk.");
    fs::write_to_file(&yaml_out, &path)?;
    println!("  ✅ Updated companyName and productName in project settings.");

    Ok(())
}

pub fn add_package(
    unity_project: &UnityProject,
    reporter: &Reporter,
    package_name: &str,
    version: &str,
) -> anyhow::Result<()> {
    reporter.info("Reading packages manifest.json.");
    let path = unity_project.packages_dir().join("manifest.json");

    let mut manifest: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&path)?)
        .with_context(|| "Failed to parse manifest.json")?;

    reporter.info("Getting dependencies.");
    let deps = manifest
        .get_mut("dependencies")
        .and_then(|d| d.as_object_mut())
        .ok_or_else(|| anyhow::anyhow!("manifest.json missing 'dependencies' object"))?;

    // Check if we actually need to change anything
    reporter.info("Checking if package already exists and current version.");
    let existing_version = deps.get(package_name).and_then(|v| v.as_str());

    if existing_version == Some(version) {
        println!(
            "  📦 Package {} is already at version {}.",
            package_name, version
        );
        return Ok(());
    }

    // Update or Insert
    let existing = deps.insert(package_name.to_string(), version.into());

    reporter.info("Updating packages.");
    let output = serde_json::to_string_pretty(&manifest)?;
    fs::write_to_file(&output, &path)?;

    if existing.is_none() {
        println!(
            "  📦 Added package {} {} in manifest.json",
            package_name, version
        );
    } else {
        println!(
            "  📦 Updated package {} to {} in manifest.json",
            package_name, version
        );
    }
    Ok(())
}
