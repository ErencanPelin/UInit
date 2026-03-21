use std::path::Path;

use anyhow::{Context, Ok};

/// Creates the directory structure. Returns Ok(true) if created, Ok(false) if it already existed.
pub fn create_dirs(path: &Path) -> anyhow::Result<bool> {
    if path.exists() {
        return Ok(false);
    }
    std::fs::create_dir_all(path)
        .with_context(|| format!("Failed to create directory at {:?}", path))?;

    Ok(true)
}

/// Creates a new file. Returns Ok(true) if created, Ok(false) if it already existed.
pub fn create_file(path: &Path) -> anyhow::Result<bool> {
    // We use .create_new(true) to not overwrite any existing files
    let result = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path);

    match result {
        Result::Ok(_) => Ok(true),
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => Ok(false),
        Err(e) => {
            // Rethrow the error with additional context
            Err(anyhow::Error::new(e).context(format!("Failed to create file at {:?}", path)))
        }
    }
}

/// Recursively copies a directory from `src` to `dst`. Creates the destination directory if it doesn't exist.
pub fn copy_dir_recursive(src: &Path, dst: &Path) -> anyhow::Result<()> {
    // Ensure destination exists
    create_dirs(dst)?;

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

/// Writes content to a file. Returns Ok(()) on success, or an Error if the write failed.
pub fn write_to_file(content: &String, path: &Path) -> anyhow::Result<()> {
    let temp_path = path.with_extension("tmp");

    std::fs::write(&temp_path, content)
        .with_context(|| format!("Failed to write to file at {:?}", path))?;

    std::fs::rename(&temp_path, path).with_context(|| "Failed to write swap file")?;

    Ok(())
}
