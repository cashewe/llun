use anyhow::Result;
use crate::output_formatter::OutputFormatter;
use crate::api_client::Response;

pub struct JsonFormatter;

/// make use of the output formatter abstraction
impl OutputFormatter for JsonFormatter {
    fn format (&self, response: &Response) -> Result<String> {
        Ok(serde_json::to_string_pretty(response)?)
    }
}