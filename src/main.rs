use std::path::PathBuf;
use clap::{Parser, Subcommand};
use figment::{Figment, providers::{Serialized, Toml, Format}};
use serde::{Serialize, Deserialize};

pub mod data;
pub mod rules;
pub mod files;
pub mod api_client;
pub mod output_formatter;

pub use data::DEFAULT_CONFIG;
pub use rules::RuleManager;
pub use files::FileManager;
pub use api_client::{PromptManager, ScannerManager, AvailableScanner};
pub use output_formatter::{OutputFormat, OutputManager};

/// CLI for the application
#[derive(Parser)]
#[command(name = "llun")]
#[command(about = "LLM backed technical strategy tool", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Run LLM based architectural survey")]
    Check (Args),
}

/// Arguments for the check cli command
/// NOTE: skip_serialisation_if must be set to allow toml values to
/// not be overwritten by emty values
#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct Args {
    /// path from root to desired directory or specific file
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PathBuf>,

    /// paths otherwise targetted by 'path' that should be skipped from scanning
    #[arg(short, long)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    exclude: Vec<PathBuf>,

    /// rules to utilise in the scan (overrides default values)
    #[arg(short, long)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    select: Vec<String>,

    /// rules to add to the default to utilise in the scan
    #[arg(long)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    extend_select: Vec<String>,

    /// rules to ignore from the default list
    #[arg(short, long)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    ignore: Vec<String>,

    /// openai model to use under the hood
    #[arg(short = 'M', long)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    model: Option<String>,

    /// default ignore all files in the gitignore, to avoid leaking secrets etc...
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    #[serde(default)]
    no_respect_gitignore: bool,

    /// type of output to give
    #[arg(short, long)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    output_format: Vec<OutputFormat>,

    /// llm provider
    #[arg(short, long)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    provider: Option<AvailableScanner>,
}

#[allow(dead_code)]  // the codes not dead, just uncalled in the repo
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let rule_manager = RuleManager::new()?;
    let scanner_manager = ScannerManager::new()?;
    let output_manager = OutputManager::new();
    
    match cli.command {
        Commands::Check(cli_args) => {
            let config: Args = Figment::new()
                .merge(Toml::string(DEFAULT_CONFIG)) // default values are set in the data file
                .merge(Toml::file("pyproject.toml").nested())
                .merge(Toml::file("llun.toml"))
                .merge(Serialized::defaults(cli_args))
                .select("tool.llun")
                .extract()?;

            let files = FileManager::load_from_cli(config.path.expect("A path must be set"), config.exclude, config.no_respect_gitignore)?;
            let rules = rule_manager.load_from_cli(config.select, config.extend_select, config.ignore)?;

            let prompt_manager = PromptManager::new(&rules, &files)?;
            let model_response = scanner_manager.run_scan(&prompt_manager.system_prompt, &prompt_manager.user_prompt, config.model.expect("A model must be provided"), config.provider.expect("A provider must be provided.")).await?;
            
            output_manager.process_response(&model_response, &config.output_format)?
        }
    }
    Ok(())
}
