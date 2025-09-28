use crate::api_client::{OpenAiClientError, PromptManagerError, ScannerError, ScannerManagerError};
use crate::files::{FileError, FileManagerError, FileSetError};
use crate::formatters::{OutputFormatterError, OutputManagerError};
use crate::rules::{RuleError, RuleManagerError};
use crate::per_file_ignorer::PerFileIgnorerError;

/// all possible custom errors from the llun library
#[derive(Debug, thiserror::Error)]
pub enum LlunCoreError {
    #[error("Error in OpenAiClient")]
    OpenAiClientError(#[from] OpenAiClientError),
    #[error("Error in PromptManager")]
    PromptManagerError(#[from] PromptManagerError),
    #[error("Error in Scanner")]
    ScannerError(#[from] ScannerError),
    #[error("Error in ScannerManager")]
    ScannerManagerError(#[from] ScannerManagerError),
    #[error("Error in File")]
    FileError(#[from] FileError),
    #[error("Error in FileSet")]
    FileSetError(#[from] FileSetError),
    #[error("Error in FileManager")]
    FileManagerError(#[from] FileManagerError),
    #[error("Error in OutputFormatter")]
    OutputFormatterError(#[from] OutputFormatterError),
    #[error("Error in OutputManager")]
    OutputManagerError(#[from] OutputManagerError),
    #[error("Error in Rule")]
    RuleError(#[from] RuleError),
    #[error("Error in RuleManager")]
    RuleManagerError(#[from] RuleManagerError),
    #[error("Error in PerFileIgnorer")]
    PerFileIgnorerError(#[from] PerFileIgnorerError)
}
