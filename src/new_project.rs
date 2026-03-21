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
use crate::unity_project::UnityProject;

pub fn init_project(ctx: &ProjectContext, unity_project: &UnityProject) -> anyhow::Result<()> {
    println!(
        "🚀 Uinit: Initialising '{}' with '{}' template...\n",
        ctx.project_name, ctx.template_alias
    );

    // 1. Filesystem & Template Generation
    create_from_template(ctx, unity_project)
        .with_context(|| format!("Failed to apply template: {}", ctx.template_alias))?;

    // 2. Unity Internal Settings Update
    modify_project_settings(ctx, unity_project)
        .context("Failed to update Unity ProjectSettings.asset.")?;

    // 3. Context Persistence
    let config: UinitConfig = ctx.into();
    config
        .save(&unity_project.root)
        .context("Failed to save uinit config to disk.")?;

    println!("\n✨ '{}' initialized successfully.", ctx.project_name);
    Ok(())
}

fn create_from_template(ctx: &ProjectContext, unity_project: &UnityProject) -> anyhow::Result<()> {
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

    for &raw_path in *paths {
        let template_path = raw_path.replace("{}", &ctx.project_name);
        let full_path = unity_project.root.join(&template_path);
        let relative_path = unity_project.rel_path(&full_path);

        if raw_path.ends_with('/') {
            if fs::create_dirs(&full_path)? {
                println!("  📁 Created: {}", relative_path);
            }
            continue;
        }

        // Create the file
        if fs::create_file(&full_path)? {
            println!("  📄 Created: {}", relative_path);
        }

        // Determine content
        let file_name = full_path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if let Some(&raw_template) = template_map.get(file_name) {
            let content = if raw_template.contains("{{") {
                env.render_str(raw_template, ctx)
                    .with_context(|| format!("Failed to render template {}", file_name))?
            } else {
                raw_template.to_string()
            };
            fs::write_to_file(&content, &full_path)?;
        }
    }

    // Dependencies
    for (pkg, ver) in *deps {
        unity_project.add_package(pkg, ver)?;
    }

    Ok(())
}

fn modify_project_settings(
    ctx: &ProjectContext,
    unity_project: &UnityProject,
) -> anyhow::Result<()> {
    let path = unity_project
        .project_settings_dir()
        .join("ProjectSettings.asset");

    let raw_yaml =
        std::fs::read_to_string(&path).with_context(|| "Failed to read ProjectSettings.asset")?;

    let mut settings: serde_yaml::Value =
        serde_yaml::from_str(&raw_yaml).with_context(|| "Failed to parse ProjectSettings.asset")?;

    // Surgical Update: Flatten the nesting
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

    // Use the bool return from your fs module to provide better feedback
    fs::write_to_file(&yaml_out, &path)?;
    println!("  ✅ Updated companyName and productName in project settings.");

    Ok(())
}
