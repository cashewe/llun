use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Hash, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AvailableScanner {
    OpenAi,
}

impl std::str::FromStr for AvailableScanner {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "openai" => Ok(AvailableScanner::OpenAi),
            _ => Err(format!("Unknown scanner: {}", s)),
        }
    }
}