use std::path::Path;

/// Creates the whole directory structure based on the path provided.
pub fn create_directory(path: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(path)?;
    Ok(())
}

/// Creates a new file at the specified path. Fails if the file already exists.
pub fn create_file(path: &Path) -> std::io::Result<()> {
    std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)?;
    Ok(())
}
