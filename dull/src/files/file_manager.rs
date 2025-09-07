use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::{PathBuf, Path};

use crate::files::FileSet;


// claude suggested these custom errors
#[derive(Debug, thiserror::Error)]
pub enum FileManagerError {
    #[error("Path doesn't exist: {0}")]
    PathNotFound(String),
    #[error("Failed to load file: {0}")]
    FileSetLoadError(String),
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
}

/// The cli / toml values that a user can use to control files
#[derive(Debug, Default, Clone)]
pub struct FileSelectionConfig {
    pub path: PathBuf,
    pub exclude: Vec<PathBuf>,
}

#[derive(Debug, Default, Clone)]
pub struct FileManager {}

impl FileManager {
    /// load the files into a FileSet based on the users provided config
    pub fn load_fileset(config: &FileSelectionConfig) -> Result<FileSet, FileManagerError> {
        Self::validate_path(&config.path)?;
        let exclude_set: HashSet<PathBuf> = config.exclude.iter().cloned().collect();
        let files = Self::collect_files(&config.path, &exclude_set)?;

        FileSet::load_from_files(files).map_err(|e| FileManagerError::FileSetLoadError(e.to_string()))
    }

    /// validate that the provided path exists
    pub fn validate_path(path: &PathBuf) -> Result<(), FileManagerError> {
        if !path.exists() {
            return Err(FileManagerError::PathNotFound(path.to_string_lossy().to_string()));
        }; 

        Ok(())
    }

    /// get the selected filepaths
    pub fn collect_files(
        path: &Path,
        exclude_set: &HashSet<PathBuf>
    ) -> Result<Vec<PathBuf>, FileManagerError> {
        let mut files = Vec::new();
        Self::collect_files_recursive(path, exclude_set, &mut files)?;

        Ok(files)
    }

    /// recursively update the mutable files param
    pub fn collect_files_recursive(
        path: &Path,
        exclude_set: &HashSet<PathBuf>,
        files: &mut Vec<PathBuf>
    ) -> Result<(), FileManagerError> {
        if exclude_set.contains(path) {
            return Ok(());
        };

        if path.is_file() {
            files.push(path.to_path_buf());
        } else if path.is_dir() {
            let entries = fs::read_dir(path)?;

            for entry in entries {
                let entry = entry?;
                let entry_path = entry.path();

                if exclude_set.contains(&entry_path) {
                    continue;
                }

                Self::collect_files_recursive(&entry_path, exclude_set, files)?;
            }
        }

        Ok(())
    }

    /// CLI facing entry point
    pub fn load_from_cli(path: PathBuf, exclude: Vec<PathBuf>) -> Result<FileSet, FileManagerError> {
        let config = FileSelectionConfig{
            path,
            exclude,
        };

        Ok(Self::load_fileset(&config)?)
    }
}

