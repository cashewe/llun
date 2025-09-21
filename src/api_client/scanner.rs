use crate::api_client::Response;
use async_openai::error::OpenAIError;

#[derive(Debug, thiserror::Error)]
pub enum ScannerError {
    #[error("Open AI client raised an error")]
    OpenAiError(#[from] OpenAIError),
    #[error("Failed to json load")]
    FailedLoadingJson(#[from] serde_json::Error),
    #[error("Failed dealing with OpenAiClient {0}")]
    OpenAiClientError(String),
}

/// abstract concept of a tool that can scan files
/// In most cases, id imagine this will be a wrapper on an LLM client
#[async_trait::async_trait]
pub trait Scanner {
    async fn scan_files(&self, system_prompt: &str, user_prompt: &str, model: &str) -> Result<Response, ScannerError>;
}