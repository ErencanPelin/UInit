use crate::constants;
use crate::contexts::ProjectContext;
use minijinja::{Environment, context};
use std::path::Path;

pub fn validate_current_directory(current_directory: &Path) -> std::io::Result<()> {
    match current_directory.file_name().and_then(|n| n.to_str()) {
        Some("Assets") => Ok(()),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Please run this from inside the `Assets` directory.",
        )),
    }
}

pub fn create_directory(path: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(path)?;
    Ok(())
}

pub fn create_file(path: &Path) -> std::io::Result<()> {
    std::fs::File::create(path)?;
    Ok(())
}

pub fn write_common_files(
    project_path: &Path,
    env: &Environment,
    ctx: &ProjectContext,
) -> std::io::Result<()> {
    let license_path = project_path.join("LICENSE");
    let rendered_license = render_template(&env, constants::LICENSE_CONTENT, ctx)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    std::fs::write(&license_path, rendered_license)?;

    let gitignore_path = project_path.join(".gitignore");
    std::fs::write(&gitignore_path, constants::GITIGNORE_CONTENT)?;

    Ok(())
}

pub fn render_template(
    env: &Environment,
    template_source: &str,
    ctx: &ProjectContext,
) -> Result<String, minijinja::Error> {
    env.render_str(
        template_source,
        context!(
            project_name => ctx.project_name,
            company => ctx.company,
            email => ctx.email,
            author => ctx.author,
            year => ctx.year,
        ),
    )
}
