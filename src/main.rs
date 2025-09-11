use std::path::PathBuf;
use clap::{Parser, Subcommand};

pub mod data;
pub mod rules;
pub mod files;
pub mod api_client;

pub use rules::RuleManager;
pub use files::FileManager;
pub use api_client::{PromptManager, OpenAiPublicClient};

/// CLI for the application
#[derive(Parser)]
#[command(name = "tynnu")]
#[command(about = "LLM backed technical strategy tool", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Run LLM based architectural survey")]
    Check {
        path: PathBuf,
        #[arg(short, long)]
        exclude: Vec<PathBuf>,
        #[arg(short, long)]
        select: Vec<String>,
        #[arg(long)]
        extend_select: Vec<String>,
        #[arg(short, long)]
        ignore: Vec<String>,
        #[arg(short = 'M', long, default_value = "gpt-4o")]
        model: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let rule_manager = RuleManager::new()?;
    let openai_client = OpenAiPublicClient::new()?;
    
    match cli.command {
        Commands::Check { path, exclude, select, extend_select, ignore, model } => {
            let files = FileManager::load_from_cli(path, exclude)?;
            let rules = rule_manager.load_from_cli(select, extend_select, ignore)?;

            let prompt_manager = PromptManager::new(&rules, &files)?;
            let model_response = openai_client.scan_files(&prompt_manager.system_prompt, &prompt_manager.user_prompt, model).await?;
            println!("{}", serde_json::to_string_pretty(&model_response)?);
        }
    }
    Ok(())
}
