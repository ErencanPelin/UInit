use std::path::Path;

use crate::{
    constants, fs, metadata::ProjectMetadata, new_project::ProjectContext,
    unity_project::UnityProject,
};
use minijinja::{Environment, context};

pub fn init_feature(
    feature_name: &str,
    unity_project: &UnityProject,
) -> Result<(), Box<dyn std::error::Error>> {
    // Reconstruct the project context from existing metadata so feature generation can run later.
    let metadata = ProjectMetadata::load(&unity_project.root)?;
    let ctx = ProjectContext {
        template: metadata.template,
        project_name: metadata.project_name.as_str(),
        company: metadata.company.as_str(),
        email: metadata.email.as_str(),
        year: metadata.year,
    };

    // Create folders for feature domain inside /Assets/<ProjectName>/Scripts
    let feature_folder = unity_project
        .root
        .join(format!("Assets/{}/Scripts", ctx.project_name))
        .join(feature_name);

    let runtime_folder = feature_folder.join("Runtime");
    let editor_folder = feature_folder.join("Editor");
    let tests_folder = feature_folder.join("Tests");

    fs::create_directory(&runtime_folder).map_err(|e| Box::new(e))?;
    println!("Created directory: {}", runtime_folder.display());
    fs::create_directory(&editor_folder).map_err(|e| Box::new(e))?;
    println!("Created directory: {}", editor_folder.display());
    fs::create_directory(&tests_folder).map_err(|e| Box::new(e))?;
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

    // TODO: we need to link the assemblies together so that editor and tests
    // can reference runtime assembly. This requires modifying the references array in the asmdef files
    // ideally we reference the GUID but we don't have access to that until Unity generates it, so we should
    // access by name

    Ok(())
}

fn create_assembly_definition(
    path: &Path,
    template_source: &str,
    ctx: &ProjectContext,
    assembly_type: &str,
    feature_name: &str,
    dependencies: Option<&[String]>,
    env: &Environment,
) -> Result<String, Box<dyn std::error::Error>> {
    let asmdef = render_jinja_template(
        &template_source,
        &feature_name,
        &dependencies.unwrap_or(&[]),
        &ctx,
        &env,
    )?;
    let assembly_name_file_name = format!(
        "com.{}.{}.{}.{}.asmdef",
        &ctx.company, &ctx.project_name, &feature_name, &assembly_type
    );

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
) -> Result<String, minijinja::Error> {
    env.render_str(
        template_source,
        context!(
            project_name => ctx.project_name,
            company => ctx.company,
            feature_name => feature_name,
            assembly_references => assembly_references,
        ),
    )
}
