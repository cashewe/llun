use async_openai::{
    Client,
    types::{
        ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
};

use crate::api_client::Response;

#[derive(Debug, thiserror::Error)]
pub enum OpenAiClientError{
    #[error("Empty response from model")]
    EmptyResponse,
    #[error("OpenAI API request failed: {0}")]
    ApiRequestFailed(#[from] async_openai::error::OpenAIError),
    #[error("Failed to parse response as JSON: {0}")]
    JsonParseError(#[from] serde_json::Error),
}

#[derive(Debug, Clone)]
pub struct OpenAiPublicClient {
    pub client: Client<async_openai::config::OpenAIConfig>,
}

impl OpenAiPublicClient {
    /// assumes user has an openai key set
    /// we will need a different setup for alternate scenarios
    /// taken from https://docs.rs/async-openai/0.29.3/async_openai/
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::new();

        Ok(Self { client })
    }

    /// get the models response to our lovely prompts
    /// taken from https://github.com/64bit/async-openai/blob/main/examples/chat/src/main.rs
    pub async fn scan_files(&self, system_prompt: &String, user_prompt: &String) -> Result<Response, OpenAiClientError> {
        let request = CreateChatCompletionRequestArgs::default()
            .max_tokens(1024u32)
            .model("gpt-4o")
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
        let formatted_response: Response = serde_json::from_str(content)?;
        
        Ok(formatted_response)
    }
}