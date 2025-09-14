use anyhow::Result;
use crate::api_client::Response;

/// abstract concept of a tool that can scan files
/// In most cases, id imagine this will be a wrapper on an LLM client
#[async_trait::async_trait]
pub trait Scanner {
    async fn scan_files(&self, system_prompt: &String, user_prompt: &String, model: String) -> Result<Response>;
}