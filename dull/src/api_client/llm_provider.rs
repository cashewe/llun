#[derive(Debug, Clone)]
pub struct LlmConfig {
    pub model: String,
    pub temperature: f32,
    pub max_tokens: Option<u32>,
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            model: "gpt-4".to_string(),
            temperature: 0.1,
            max_tokens: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum LlmProvider {
    OpenAi,
}

impl std::str::FromStr for LlmProvider {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "openai" => Ok(LlmProvider::OpenAi),
            _ => Err(format!("Unregistered Provider: {}", s)), 
        }
    }
}