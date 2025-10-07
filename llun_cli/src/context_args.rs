use serde::{Deserialize, Serialize};
use clap::Parser;


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AgentFormat {
    CopilotInstructions,
    Agents,
}

/// convert arbitrary string to enum
impl std::str::FromStr for AgentFormat {
    type Err = String;

    /// this feels like something that might already exist in some default crate
    /// go check for this...
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "copilot-instructions" => Ok(AgentFormat::CopilotInstructions),
            "agents" => Ok(AgentFormat::Agents),
            _ => Err(format!("Unknown output format: {}", s)),
        }
    }
}

/// Arguments for the install cli command
#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct ContextArgs {
    /// rules to utilise in the scan (overrides default values)
    #[arg(short, long)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub select: Vec<String>,

    /// rules to add to the default to utilise in the scan
    #[arg(long)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extend_select: Vec<String>,

    /// rules to ignore from the default list
    #[arg(short, long)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ignore: Vec<String>,

    /// format to install rules into
    #[arg(short, long, value_enum)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_format: Option<AgentFormat>,

    /// verbosity of the command, stacks with more 'v's
    #[arg(short = 'v', action = clap::ArgAction::Count)]
    pub verbose: u8,
}