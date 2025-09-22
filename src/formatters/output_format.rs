use serde::{Serialize, Deserialize};

/// acceptable output types (user controlled)
#[derive(Debug, Clone, Copy, Hash, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Json,
    Azure,
    Junit,
    Summary,
}

/// convert arbitrary string to enum
impl std::str::FromStr for OutputFormat {
    type Err = String;

    /// this feels like something that might already exist in some default crate
    /// go check for this...
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "azure" => Ok(OutputFormat::Azure),
            "junit" => Ok(OutputFormat::Junit),
            "summary" => Ok(OutputFormat::Summary),
            _ => Err(format!("Unknown output format: {}", s)),
        }
    }
}
