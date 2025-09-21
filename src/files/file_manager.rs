use std::collections::HashSet;
use std::io;
use std::path::{PathBuf, Path};
use ignore::{WalkBuilder};

use crate::files::{FileSet, File, FileError};


// claude suggested these custom errors
#[derive(Debug, thiserror::Error)]
pub enum FileManagerError {
    #[error("Path doesn't exist: {0}")]
    PathNotFound(String),
    #[error("Failed to load file: {0}")]
    FileSetLoadError(String),
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    #[error("walk error: {0}")]
    WalkError(#[from] ignore::Error),
    #[error("Rule failed to be read file")]
    FileReadError(#[from] FileError),
}

/// The cli / toml values that a user can use to control files
#[derive(Debug, Default, Clone)]
pub struct FileSelectionConfig {
    pub paths: Vec<PathBuf>,
    pub exclude: Vec<PathBuf>,
    pub no_respect_gitignore: bool,
}

#[derive(Debug, Default, Clone)]
pub struct FileManager {}

impl FileManager {
    /// load the files into a FileSet based on the users provided config
    pub fn load_fileset(config: &FileSelectionConfig) -> Result<FileSet, FileManagerError> {
        let mut all_files = Vec::new();
        let exclude_set: HashSet<PathBuf> = config.exclude.iter().cloned().collect();

        for path in &config.paths {
            Self::validate_path(path)?;
            let files = Self::collect_files(path, &exclude_set, config.no_respect_gitignore)?;
            all_files.extend(files);
        }

        FileManager::load_from_files(all_files).map_err(|e| FileManagerError::FileSetLoadError(e.to_string()))
    }

    /// create a fileset
    pub fn load_from_files(file_paths: Vec<PathBuf>) -> Result<FileSet, FileManagerError> {
        let mut collection = FileSet::new();

        for file_path in file_paths {
            match File::from_file(file_path.to_string_lossy().to_string()) {
                Ok(file) => collection.add_file(file),
                Err(e) => return Err(FileManagerError::FileReadError(e)),
            }
        }
        Ok(collection)
    }

    /// validate that the provided path exists
    pub fn validate_path(path: &Path) -> Result<(), FileManagerError> {
        if !path.exists() {
            return Err(FileManagerError::PathNotFound(path.to_string_lossy().to_string()));
        }; 

        Ok(())
    }

    /// get the selected filepaths
    pub fn collect_files(
        root: &Path,
        exclude_set: &HashSet<PathBuf>,
        no_respect_gitignore: bool,
    ) -> Result<Vec<PathBuf>, FileManagerError> {
        let mut files = Vec::new();

        let mut builder = WalkBuilder::new(root);
        builder.git_ignore(!no_respect_gitignore);
        builder.hidden(false);
        builder.follow_links(false);

        let walker = builder.build();

        for result in walker {
            let dent = result?;
            let path = dent.path();

            // skip directories entirely if excluded
            if exclude_set.contains(path) {
                continue;
            }

            if path.is_file() {
                files.push(path.to_path_buf());
            }
        }

        Ok(files)
    }

    /// CLI facing entry point
    pub fn load_from_cli(paths: Vec<PathBuf>, exclude: Vec<PathBuf>, no_respect_gitignore: bool) -> Result<FileSet, FileManagerError> {
        let config = FileSelectionConfig{
            paths,
            exclude,
            no_respect_gitignore,
        };

        Self::load_fileset(&config)
    }
}

