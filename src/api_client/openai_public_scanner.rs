use async_openai::{
    Client,
    types::{
        ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
};
use anyhow::Result;
use crate::api_client::{Response, Scanner};

#[derive(Debug, thiserror::Error)]
pub enum OpenAiClientError{
    #[error("Empty response from model")]
    EmptyResponse,
    #[error("OpenAI API request failed: {0}")]
    ApiRequestFailed(#[from] async_openai::error::OpenAIError),
    #[error("Failed to parse response as JSON: {0}")]
    JsonParseError(#[from] serde_json::Error),
    #[error("Failed to extract json from response")]
    JsonCleaningError,
}

#[derive(Debug, Clone)]
pub struct OpenAiPublicScanner {
    pub client: Client<async_openai::config::OpenAIConfig>,
}

#[async_trait::async_trait]
impl Scanner for OpenAiPublicScanner {
/// get the models response to our lovely prompts
    /// taken from https://github.com/64bit/async-openai/blob/main/examples/chat/src/main.rs
    async fn scan_files(&self, system_prompt: &String, user_prompt: &String, model: String) -> Result<Response> {
        let request = CreateChatCompletionRequestArgs::default()
            .model(model)
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(system_prompt.to_string())
                    .build()?
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(user_prompt.to_string())
                    .build()?
                    .into(),
            ])
            .build()?;

        let response = self.client.chat().create(request).await?;
        let content = response.choices.first().and_then(|choice| choice.message.content.as_ref()).ok_or(OpenAiClientError::EmptyResponse)?;
        let cleaned_content = Self::extract_json_from_response(content)?;
        let formatted_response: Response = serde_json::from_str(cleaned_content)?;
        
        Ok(formatted_response)
    }
}

impl OpenAiPublicScanner {
    /// assumes user has an openai key set
    /// we will need a different setup for alternate scenarios
    /// taken from https://docs.rs/async-openai/0.29.3/async_openai/
    pub fn new() -> Result<Self> {
        let client = Client::new();

        Ok(Self { client })
    }

    /// https://docs.rs/async-openai/0.29.3/async_openai/types/struct.CreateChatCompletionRequest.html#structfield.response_format
    /// should be possible ^^, but when i tried it was bugging out. (just kept saying the response had the wrong schema with no elaboration)
    /// id rather prioritise a working POC for now so will put it on the back burner and loop back round
    /// in the meantime, heres the crappiest hand spun cleaning function
    /// you ever did see
    pub fn extract_json_from_response(content: &str) -> Result<&str, OpenAiClientError> {
        let start_pos = content.find('{')
            .ok_or_else(|| OpenAiClientError::JsonCleaningError)?;
        
        let end_pos = content.rfind('}')
            .ok_or_else(|| OpenAiClientError::JsonCleaningError)?;
        
        if start_pos >= end_pos {
            return Err(OpenAiClientError::JsonCleaningError);
        }
        
        Ok(&content[start_pos..=end_pos])
    }
}