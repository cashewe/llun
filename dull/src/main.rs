use std::path::{PathBuf};

use clap::{Parser, Subcommand};

mod data;
mod rules;
use rules::RuleManager;
mod files;
use files::FileManager;
mod api_client;
use api_client::{PromptManager, OpenAiPublicClient};

/// cli for the application
#[derive(Parser)]
#[command(name = "dull")]
#[command(about = "LLM backed technical strategy tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // the 'check' command registered with the following args
    Check {
        path: PathBuf,
        #[arg(short, long)]
        exclude: Vec<PathBuf>,  // exclude specified files from your path if its a dir
        #[arg(short, long)]
        select: Vec<String>, // if you dont select, itll use the defaults
        #[arg(long)]
        extend_select: Vec<String>, // your selection should strictly extend the default set
        #[arg(short, long)]
        ignore: Vec<String>, // if you use ignore itll remove the rule even if selected
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let rule_manager = RuleManager::new()?;
    let openai_client = OpenAiPublicClient::new()?;
    
    match cli.command {
        Commands::Check { path, exclude, select, extend_select, ignore } => {
            let files = FileManager::load_from_cli(path, exclude)?;
            let rules = rule_manager.load_from_cli(select, extend_select, ignore)?;

            let prompt_manager = PromptManager::new(&rules, &files)?;
            let model_response = openai_client.scan_files(&prompt_manager.system_prompt, &prompt_manager.user_prompt).await?;
            println!("{:?}", model_response);
        }
    }
    Ok(())
}
