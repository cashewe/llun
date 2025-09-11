use std::path::PathBuf;
use clap::{Parser, Subcommand};
use figment::{Figment, providers::{Serialized, Toml, Format}};
use serde::{Serialize, Deserialize};

pub mod data;
pub mod rules;
pub mod files;
pub mod api_client;

pub use rules::RuleManager;
pub use files::FileManager;
pub use api_client::{PromptManager, OpenAiPublicClient};

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

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct Args {
    /// path from root to desired directory or specific file
    path: PathBuf,

    /// paths otherwise targetted by 'path' that should be skipped from scanning
    #[arg(short, long)]
    #[serde(default)]
    exclude: Vec<PathBuf>,

    /// rules to utilise in the scan (overrides default values)
    #[arg(short, long)]
    #[serde(default)]
    select: Vec<String>,

    /// rules to add to the default to utilise in the scan
    #[arg(long)]
    #[serde(default)]
    extend_select: Vec<String>,

    /// rules to ignore from the default list
    #[arg(short, long)]
    #[serde(default)]
    ignore: Vec<String>,

    /// openai model to use under the hood, defaults to 'gpt-4o'
    #[arg(short = 'M', long, default_value = "gpt-4o")]
    #[serde(default)]
    model: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let rule_manager = RuleManager::new()?;
    let openai_client = OpenAiPublicClient::new()?;
    
    match cli.command {
        Commands::Check(cli_args) => {
            let config: Args = Figment::new()
                .merge(Toml::file("pyproject.toml").nested())
                .merge(Toml::file("llun.toml"))
                .merge(Serialized::defaults(cli_args))
                .select("tool.llun")
                .extract()?;

            let files = FileManager::load_from_cli(config.path, config.exclude)?;
            let rules = rule_manager.load_from_cli(config.select, config.extend_select, config.ignore)?;

            let prompt_manager = PromptManager::new(&rules, &files)?;
            let model_response = openai_client.scan_files(&prompt_manager.system_prompt, &prompt_manager.user_prompt, config.model).await?;
            println!("{}", serde_json::to_string_pretty(&model_response)?);
        }
    }
    Ok(())
}
