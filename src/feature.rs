use std::path::Path;

use crate::{
    config::UinitConfig, constants, fs, project_context::ProjectContext, reporter::Reporter,
    unity_project::UnityProject,
};
use anyhow::{Context, bail};
use minijinja::{Environment, context};

pub fn init_feature(
    feature_name: &str,
    unity_project: &UnityProject,
    reporter: &Reporter,
) -> anyhow::Result<()> {
    println!("🚀 Uinit: Adding {} feature module...", feature_name);

    reporter.info("Getting project context from uinit.toml");
    let config = UinitConfig::load(&unity_project.root)?;
    let ctx: ProjectContext = ProjectContext::from_config(&config);

    // Create folders for feature domain inside /Assets/<ProjectName>/Scripts
    reporter.info("Creating folders for feature.");
    let feature_folder = unity_project
        .root
        .join(format!("Assets/{}/Scripts", ctx.project_name))
        .join(feature_name);

    if feature_folder.exists() {
        reporter.info(&format!(
            "Skipped: directory already exists {:?}.",
            feature_folder
        ));
        bail!(format!("Feature {} is already defined.", feature_name))
    }

    let runtime_folder = feature_folder.join("Runtime");
    let editor_folder = feature_folder.join("Editor");
    let tests_folder = feature_folder.join("Tests");

    reporter.info("Creating runtime folder.");
    if fs::create_dirs(&runtime_folder)? {
        println!("  📁 Created: {}", unity_project.rel_path(&runtime_folder));
    }
    reporter.info("Creating editor folder.");
    if fs::create_dirs(&editor_folder)? {
        println!("  📁 Created: {}", unity_project.rel_path(&editor_folder));
    }
    reporter.info("Creating tests folder.");
    if fs::create_dirs(&tests_folder)? {
        println!("  📁 Created: {}", unity_project.rel_path(&tests_folder));
    }

    // Create assembly definition files for the feature domain
    reporter.info("Creating required assemblies.");
    let env = Environment::new();
    let runtime_assembly_name = create_assembly_definition(
        &runtime_folder,
        constants::ASSEMBLY_DEF_RUNTIME_JINJA,
        &ctx,
        reporter,
        "runtime",
        feature_name,
        None,
        &env,
    )?;
    create_assembly_definition(
        &editor_folder,
        constants::ASSEMBLY_DEF_EDITOR_JINJA,
        &ctx,
        reporter,
        "editor",
        feature_name,
        Some(&[runtime_assembly_name.clone()]),
        &env,
    )?;
    create_assembly_definition(
        &tests_folder,
        constants::ASSEMBLY_DEF_TESTS_JINJA,
        &ctx,
        reporter,
        "tests",
        feature_name,
        Some(&[runtime_assembly_name.clone()]),
        &env,
    )?;

    println!("\n✨ '{}' initialized successfully.", feature_name);

    Ok(())
}

// TODO: move to a shared folder
pub fn create_assembly_definition(
    dir_path: &Path,
    template_source: &str,
    ctx: &ProjectContext,
    reporter: &Reporter,
    assembly_type: &str,
    feature_name: &str,
    dependencies: Option<&[String]>,
    env: &Environment,
) -> anyhow::Result<String> {
    let assembly_name = format!(
        "com.{}.{}.{}.{}",
        ctx.company, ctx.project_name, feature_name, assembly_type
    )
    .to_lowercase();

    let file_name = format!("{}.asmdef", assembly_name);
    let full_path = dir_path.join(&file_name);

    reporter.info("Rendering assembly file from jinja2 template.");
    let rendered_content = render_jinja_template(
        template_source,
        feature_name,
        dependencies.unwrap_or(&[]),
        ctx,
        env,
    )
    .with_context(|| format!("Failed to render {} asmdef", assembly_type))?;

    reporter.info("Creating assembly file on disk.");
    if fs::create_file(&full_path)? {
        println!("  ✅ Created assembly {}", file_name);
    } else {
        reporter.info("Assembly file already exists on disk.");
    }

    reporter.info("Writing to assembly file.");
    std::fs::write(&full_path, rendered_content)
        .with_context(|| format!("Failed to write asmdef to {:?}", full_path))?;

    Ok(assembly_name)
}

fn render_jinja_template(
    template_source: &str,
    feature_name: &str,
    assembly_references: &[String],
    ctx: &ProjectContext,
    env: &Environment,
) -> anyhow::Result<String> {
    let rendered = env.render_str(
        template_source,
        context!(
            project_name => ctx.project_name,
            company => ctx.company,
            feature_name => feature_name,
            assembly_references => assembly_references,
        ),
    )?;

    Ok(rendered)
}
