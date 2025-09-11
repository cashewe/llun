use std::fmt;
use std::path::PathBuf;
use super::file::File;

#[derive(Debug, Default)]
pub struct FileSet {
    files: Vec<File>,
}

impl FileSet {
    pub fn new() -> Self {
        Self::default()
    }

        /// load the file data in
    pub fn load_from_files(file_paths: Vec<PathBuf>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut collection = Self::new();

        for file_path in file_paths {
            match File::from_file(file_path.to_string_lossy().to_string()) {
                Ok(file) => collection.add_file(file),
                Err(e) => eprintln!("Failed to load file: {}", e),
            }
        }
        Ok(collection)
    }

    /// add a rule to the Vec
    pub fn add_file(&mut self, file: File) {
        self.files.push(file);
    }
}

/// we will (hopefully) use display to insert into a markdown message?
impl fmt::Display for FileSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "# Files\n\n")?;

        for file in &self.files {
            write!(f, "\n---\n{}\n", file)?;
        }
        Ok(())
    }
}

/// owned iteration, may want to implement borrowed itteration in future?
impl IntoIterator for FileSet {
    type Item = File;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.files.into_iter()
    }
}