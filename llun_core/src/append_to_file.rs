use std::fs;
use std::io::Write;
use std::path::PathBuf;

/// Appends content to a file, creating parent directories and the file if needed
pub fn append_to_file(path: &PathBuf, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let existing_content = fs::read_to_string(path).unwrap_or_default();
    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;

    if !existing_content.is_empty() {
        write!(file, "{}\n\n", existing_content)?;
    }
    write!(file, "{}", content)?;

    Ok(())
}
