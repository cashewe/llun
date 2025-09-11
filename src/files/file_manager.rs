use std::collections::HashSet;
use std::io;
use std::path::{PathBuf, Path};
use ignore::{WalkBuilder};

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
    #[error("walk error: {0}")]
    WalkError(#[from] ignore::Error),
}

/// The cli / toml values that a user can use to control files
#[derive(Debug, Default, Clone)]
pub struct FileSelectionConfig {
    pub path: PathBuf,
    pub exclude: Vec<PathBuf>,
    pub no_respect_gitignore: bool,
}

#[derive(Debug, Default, Clone)]
pub struct FileManager {}

impl FileManager {
    /// load the files into a FileSet based on the users provided config
    pub fn load_fileset(config: &FileSelectionConfig) -> Result<FileSet, FileManagerError> {
        Self::validate_path(&config.path)?;
        let exclude_set: HashSet<PathBuf> = config.exclude.iter().cloned().collect();
        let files = Self::collect_files(&config.path, &exclude_set, config.no_respect_gitignore)?;

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
    pub fn load_from_cli(path: PathBuf, exclude: Vec<PathBuf>, no_respect_gitignore: bool) -> Result<FileSet, FileManagerError> {
        let config = FileSelectionConfig{
            path,
            exclude,
            no_respect_gitignore,
        };

        Ok(Self::load_fileset(&config)?)
    }
}

