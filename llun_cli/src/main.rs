use clap::{Parser, Subcommand};
use figment::{
    Figment,
    providers::{Format, Serialized, Toml},
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::info;

use llun_core::api_client::{AvailableScanner, PromptManager, ScannerManager};
use llun_core::data::DEFAULT_CONFIG;
use llun_core::files::FileManager;
use llun_core::formatters::{OutputFormat, OutputManager};
use llun_core::rules::RuleManager;
use llun_core::per_file_ignorer::PerFileIgnorer;

pub mod logging;
use logging::init_tracing;

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
    Check(CheckArgs),
}

/// Arguments for the check cli command
/// NOTE: skip_serialisation_if must be set to allow toml values to
/// not be overwritten by emty values
#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct CheckArgs {
    /// paths from root to desired directory or specific file
    #[serde(skip_serializing_if = "Vec::is_empty")]
    path: Vec<PathBuf>,

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
    #[arg(long)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    provider: Option<AvailableScanner>,

    /// user provided context (i.e. commit message) to help llun understand the point
    #[arg(short, long)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    context: Option<String>,

    /// utilise USC to improve the reliability of the model response
    #[arg(long, action = clap::ArgAction::SetTrue)]
    #[serde(default)]
    production_mode: bool,

    /// files to ignore certain rule violations on i.e. 'main.py::RULE01'
    #[arg(long)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    per_file_ignores: Vec<String>,

    /// verbosity of the command, stacks with more 'v's
    #[arg(short = 'v', action = clap::ArgAction::Count)]
    verbose: u8,
}

#[allow(dead_code)] // the codes not dead, just uncalled in the repo
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check(cli_args) => {
            let config: CheckArgs = Figment::new()
                .merge(Toml::string(DEFAULT_CONFIG)) // default values are set in the data file
                .merge(Toml::file("pyproject.toml").nested())
                .merge(Toml::file("llun.toml"))
                .merge(Serialized::defaults(cli_args))
                .select("tool.llun")
                .extract()?;

            init_tracing(config.verbose);
            info!("Beginning application...");

            info!("Setting up managers...");
            let rule_manager = RuleManager::new()?;
            let scanner_manager = ScannerManager::new()?;
            let output_manager = OutputManager::new();
            let per_file_ignorer = PerFileIgnorer::new(config.per_file_ignores)?;

            info!("Reading selected files...");
            let files = FileManager::load_from_cli(
                config.path,
                config.exclude,
                config.no_respect_gitignore,
            )?;
            info!("Loading selected rules...");
            let rules =
                rule_manager.load_from_cli(config.select, config.extend_select, config.ignore)?;

            let prompt_manager = PromptManager::new(&rules, &files, &config.context)?;

            info!("Querying selected endpoint...");
            let model_response = scanner_manager
                .run_scan(
                    &prompt_manager.system_prompt_scan,
                    &prompt_manager.user_prompt,
                    &config.model.expect("A model must be provided"),
                    &prompt_manager.system_prompt_consistency,
                    config.provider.expect("A provider must be provided."),
                    config.production_mode,
                )
                .await?;

            let filtered_response = per_file_ignorer.apply_ignores(model_response);

            info!("Processing response...");
            output_manager.process_response(&filtered_response, &config.output_format)?
        }
    }
    Ok(())
}
