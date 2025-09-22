use async_openai::{
    Client,
    types::{
        ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
};
use crate::api_client::{Response, Scanner, ScannerError};

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
    #[error("Relevant secrets must be set in environment {0}")]
    MissingEnvVar(String),
}

#[derive(Debug, Clone)]
pub enum OpenAiScanner {
    Public(Client<async_openai::config::OpenAIConfig>),
    Azure(Client<async_openai::config::AzureConfig>),
}

#[async_trait::async_trait]
impl Scanner for OpenAiScanner {
/// get the models response to our lovely prompts
    /// taken from https://github.com/64bit/async-openai/blob/main/examples/chat/src/main.rs
    async fn scan_files(&self, system_prompt: &str, user_prompt: &str, model: &str) -> Result<Response, ScannerError> {
        let request = CreateChatCompletionRequestArgs::default()
            .model(model)
            .temperature(0.1)
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

        let response = match self {
            OpenAiScanner::Public(client) => client.chat().create(request).await?,
            OpenAiScanner::Azure(client) => client.chat().create(request).await?,
        };
        let content = response.choices.first().and_then(|choice| choice.message.content.as_ref()).ok_or(ScannerError::OpenAiClientError("Empty response".to_string()))?;
        let cleaned_content = Self::extract_json_from_response(content)
            .map_err(Self::map_openai_client_error)?;
        let formatted_response: Response = serde_json::from_str(cleaned_content)?;
        
        Ok(formatted_response)
    }
}

impl OpenAiScanner {
    /// instantiate a public openai instance
    pub fn new() -> Result<Self, OpenAiClientError> {
        let client = Client::new(); // it auto pulls the key env var
        Ok(Self::Public(client))
    }

    /// instantiate an azure openai instance
    pub fn new_azure() -> Result<Self, OpenAiClientError> {
        // the docs dont seem to suggest it can auto pull these sadly :( :( :(
        let api_key = std::env::var("AZURE_OPENAI_API_KEY")
            .map_err(|_| OpenAiClientError::MissingEnvVar("AZURE_OPENAI_API_KEY".to_string()))?;
        
        let endpoint = std::env::var("AZURE_OPENAI_ENDPOINT")
            .map_err(|_| OpenAiClientError::MissingEnvVar("AZURE_OPENAI_ENDPOINT".to_string()))?;
        
        let api_version = std::env::var("AZURE_OPENAI_API_VERSION")
            .map_err(|_| OpenAiClientError::MissingEnvVar("AZURE_OPENAI_API_VERSION".to_string()))?;

        let deployment = std::env::var("AZURE_OPENAI_DEPLOYMENT")
            .map_err(|_| OpenAiClientError::MissingEnvVar("AZURE_OPENAI_DEPLOYMENT".to_string()))?;

        let config = async_openai::config::AzureConfig::new()
            .with_api_key(api_key)
            .with_api_base(endpoint)
            .with_deployment_id(deployment)
            .with_api_version(api_version);

        let client = Client::with_config(config);
        Ok(Self::Azure(client))
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

    fn map_openai_client_error(err: OpenAiClientError) -> ScannerError {
        ScannerError::OpenAiClientError(format!("{err}"))
    }
}