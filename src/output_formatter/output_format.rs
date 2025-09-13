use anyhow::Result;
use serde::{Serialize, Deserialize};
use crate::api_client::Response;

/// acceptable output types (user controlled)
#[derive(Debug, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum OutputFormat {
    Json,
    StackTrace,
    AzureDevOps,
    JUnit,
}

/// convert arbitrary string to enum
impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "stacktrace" => Ok(OutputFormat::StackTrace),
            "azure" => Ok(OutputFormat::AzureDevOps),
            "junit" => Ok(OutputFormat::JUnit),
            _ => Err(format!("Unknown output format: {}", s)),
        }
    }
}

pub trait OutputFormatter {
    /// anything which can format is a formatter
    /// does this belong elsewhere? not sure on the organisation atm...
    fn format(&self, response: &Response) -> Result<String, >;
}