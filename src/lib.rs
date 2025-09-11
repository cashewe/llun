use std::path::PathBuf;
use pyo3::prelude::*;
use clap::{Parser, Subcommand};
use pyo3::types::PyModule;
use pyo3::Bound;

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

pub async fn run_with_args(args: Vec<String>) -> Result<i32, Box<dyn std::error::Error>> {
    let cli = Cli::try_parse_from(args)?;
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
    Ok(0)
}

/// enerates a python function to be used in python by pythonistas
#[pyfunction]
fn run_cli(args: Vec<String>) -> PyResult<i32> {
    let rt = tokio::runtime::Runtime::new().map_err(|e| {
        pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create tokio runtime: {}", e))
    })?;
    
    match rt.block_on(run_with_args(args)) {
        Ok(exit_code) => Ok(exit_code),
        Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!("{}", e))),
    }
}

/// registers an importable python module called _rust
#[pymodule]
fn tynnu(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_cli, m)?)?;
    Ok(())
}