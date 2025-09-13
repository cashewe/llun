use crate::output_formatter::{OutputFormat, OutputFormatter};
use crate::api_client::Response;

pub struct OutputManager {
    formatters: HashMap<OutputFormat, Box<dyn OutputFormatter>>,
}

impl OutputManager {
    pub fn new() -> Self {
        let mut formatters: HashMap<OutputFormat, Box<dyn OutputFormatter>> = HashMap::new();

        formatters.insert(OutputFormat::Json, Box::new(JsonFormatter));
        formatters.insert(OutputFormat::StackTrace, Box::new(StackTraceFormatter));
        formatters.insert(OutputFormat::AzureDevOps, Box::new(AzureDevOpsFormatter));
        formatters.insert(OutputFormat::Junit, Box::new(JunitFormatter));

        Ok(Self{ formatters})
    }

    pub fn process_response(response: &Response, output_formats: &Vec<OutputFormat>) -> Result<()> {
        println!("therell be code here eventually, but for now the pizza has arrived...")
    }
}