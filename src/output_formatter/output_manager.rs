use std::collections::HashMap;
use crate::output_formatter::{OutputFormat, OutputFormatter, OutputFormatterError};
use crate::api_client::Response;
use crate::output_formatter::{JsonFormatter, AzureFormatter};

#[derive(Debug, thiserror::Error)]
pub enum OutputManagerError {
    #[error("Failed to format the model output using the desired method: {0}")]
    OutputFormattingFailed(#[from] OutputFormatterError),
}

pub struct OutputManager {
    formatters: HashMap<OutputFormat, Box<dyn OutputFormatter>>,
}

/// manager the output format of the object in a cool, scalable way
/// is there no option for dynamic registry in rust?
impl OutputManager {
    /// register all formatters to the object
    pub fn new() -> Self {
        let mut formatters: HashMap<OutputFormat, Box<dyn OutputFormatter>> = HashMap::new();

        formatters.insert(OutputFormat::Json, Box::new(JsonFormatter));
        formatters.insert(OutputFormat::Azure, Box::new(AzureFormatter));

        Self{ formatters}
    }

    /// use the selected formats in order
    pub fn process_response(&self, response: &Response, output_formats: &Vec<OutputFormat>) -> Result<(), OutputManagerError> {
        output_formats
            .iter()
            .filter_map(|format| self.formatters.get(format).map(|formatter| formatter))
            .try_for_each(|formatter| -> Result<(), OutputManagerError> {
                Ok(println!("{}", formatter.format(response)?))
            })?;
        
        Ok(())
    }
}