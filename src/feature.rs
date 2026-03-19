use std::path::Path;

use crate::{
    config::UinitConfig, constants, fs, new_project::ProjectContext, unity_project::UnityProject,
};
use anyhow::{Context, bail};
use minijinja::{Environment, context};

pub fn init_feature(feature_name: &str, unity_project: &UnityProject) -> anyhow::Result<()> {
    // Reconstruct the project context from existing metadata so feature generation can run later.
    let config = UinitConfig::load(&unity_project.root)?;
    let ctx = ProjectContext {
        template: config.project.template,
        project_name: config.project.project_name.as_str(),
        company: config.project.company.as_str(),
        email: config.project.email.as_str(),
        year: config.project.year,
    };

    // Create folders for feature domain inside /Assets/<ProjectName>/Scripts
    let feature_folder = unity_project
        .root
        .join(format!("Assets/{}/Scripts", ctx.project_name))
        .join(feature_name);

    if feature_folder.exists() {
        bail!(format!("Feature {} is already defined.", feature_name))
    }

    let runtime_folder = feature_folder.join("Runtime");
    let editor_folder = feature_folder.join("Editor");
    let tests_folder = feature_folder.join("Tests");

    fs::create_directory(&runtime_folder)?;
    println!("Created directory: {}", runtime_folder.display());
    fs::create_directory(&editor_folder)?;
    println!("Created directory: {}", editor_folder.display());
    fs::create_directory(&tests_folder)?;
    println!("Created directory: {}", tests_folder.display());

    // Create assembly definition files for the feature domain
    let env = Environment::new();
    let runtime_assembly_name = create_assembly_definition(
        &runtime_folder,
        constants::ASSEMBLY_DEF_RUNTIME_JINJA,
        &ctx,
        "runtime",
        feature_name,
        None,
        &env,
    )?;
    create_assembly_definition(
        &editor_folder,
        constants::ASSEMBLY_DEF_EDITOR_JINJA,
        &ctx,
        "editor",
        feature_name,
        Some(&[runtime_assembly_name.clone()]),
        &env,
    )?;
    create_assembly_definition(
        &tests_folder,
        constants::ASSEMBLY_DEF_TESTS_JINJA,
        &ctx,
        "tests",
        feature_name,
        Some(&[runtime_assembly_name.clone()]),
        &env,
    )?;

    Ok(())
}

// TODO: move to a shared folder
pub fn create_assembly_definition(
    path: &Path,
    template_source: &str,
    ctx: &ProjectContext,
    assembly_type: &str,
    feature_name: &str,
    dependencies: Option<&[String]>,
    env: &Environment,
) -> anyhow::Result<String> {
    let asmdef = render_jinja_template(
        &template_source,
        &feature_name,
        &dependencies.unwrap_or(&[]),
        &ctx,
        &env,
    )
    .with_context(|| {
        format!(
            "Failed when trying to render Jinja2 template for assembly definition.\n
             Assembly Type: {}",
            &assembly_type
        )
    })?;

    let assembly_name_file_name = format!(
        "com.{}.{}.{}.{}.asmdef",
        &ctx.company, &ctx.project_name, &feature_name, &assembly_type
    )
    .to_lowercase();

    std::fs::write(path.join(&assembly_name_file_name), asmdef)?;

    println!(
        "Created assembly definition: {}",
        path.join(&assembly_name_file_name).display()
    );

    // return the assembly name for reference in other asmdefs
    Ok((assembly_name_file_name.replace(".asmdef", "")).to_string())
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
