use anyhow::Result;
use crate::api_client::Response;

pub trait OutputFormatter {
    /// anything which can format is a formatter
    /// does this belong elsewhere? not sure on the organisation atm...
    fn format(&self, response: &Response) -> Result<String>;
}
