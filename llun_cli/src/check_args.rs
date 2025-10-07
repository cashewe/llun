use serde::{Deserialize, Serialize};
use clap::Parser;
use std::path::PathBuf;
use llun_core::api_client::AvailableScanner;
use llun_core::formatters::OutputFormat;

/// Arguments for the check cli command
/// NOTE: skip_serialisation_if must be set to allow toml values to
/// not be overwritten by emty values
#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct CheckArgs {
    /// paths from root to desired directory or specific file
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub path: Vec<PathBuf>,

    /// paths otherwise targetted by 'path' that should be skipped from scanning
    #[arg(short, long)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude: Vec<PathBuf>,

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

    /// openai model to use under the hood
    #[arg(short = 'M', long)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// default ignore all files in the gitignore, to avoid leaking secrets etc...
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    #[serde(default)]
    pub no_respect_gitignore: bool,

    /// type of output to give
    #[arg(short, long)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub output_format: Vec<OutputFormat>,

    /// llm provider
    #[arg(long)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<AvailableScanner>,

    /// user provided context (i.e. commit message) to help llun understand the point
    #[arg(short, long)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,

    /// utilise USC to improve the reliability of the model response
    #[arg(long, action = clap::ArgAction::SetTrue)]
    #[serde(default)]
    pub production_mode: bool,

    /// files to ignore certain rule violations on i.e. 'main.py::RULE01'
    #[arg(long)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub per_file_ignores: Vec<String>,

    /// verbosity of the command, stacks with more 'v's
    #[arg(short = 'v', action = clap::ArgAction::Count)]
    pub verbose: u8,
}