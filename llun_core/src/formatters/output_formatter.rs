use crate::api_client::Response;

#[derive(Debug, thiserror::Error)]
pub enum OutputFormatterError {
    #[error("Failed to load models response to JSON {0}")]
    ModelResponseNotLoadable(#[from] serde_json::Error),
    #[error("Failed to produce the relevant format {0}")]
    FormatError(String),
    #[error("I/O wasnt successful {0}")]
    IoError(String),
}

pub trait OutputFormatter {
    /// anything which can format is a formatter
    /// does this belong elsewhere? not sure on the organisation atm...
    fn format(&self, response: &Response) -> Result<String, OutputFormatterError>;
}
