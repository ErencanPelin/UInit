use std::path::Path;

use anyhow::Context;

/// Creates the whole directory structure based on the path provided.
pub fn create_directory(path: &Path) -> anyhow::Result<()> {
    std::fs::create_dir_all(&path)
        .with_context(|| format!("Failed to create directory at {:?}", path))?;
    Ok(())
}

/// Creates a new file at the specified path. Fails if the file already exists.
pub fn create_file(path: &Path) -> anyhow::Result<()> {
    std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)?;
    Ok(())
}

/// Recursively copies a directory from `src` to `dst`. Creates the destination directory if it doesn't exist.
pub fn copy_dir_recursive(src: &Path, dst: &Path) -> anyhow::Result<()> {
    // Ensure destination exists
    std::fs::create_dir_all(dst)?;

    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let dest_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_recursive(&entry.path(), &dest_path)?;
        } else {
            std::fs::copy(&entry.path(), dest_path)?;
        }
    }
    Ok(())
}

// TODO: a shared function to write files with proper error handling
