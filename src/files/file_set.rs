use std::fmt;
use super::file::{File, FileError};

#[derive(Debug, thiserror::Error)]
pub enum FileSetError {
    #[error("Rule failed to be read file")]
    FileReadError(#[from] FileError),
}

#[derive(Debug, Default)]
pub struct FileSet {
    files: Vec<File>,
}

impl FileSet {
    pub fn new() -> Self {
        Self::default()
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