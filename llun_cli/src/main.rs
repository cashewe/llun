use clap::{Parser, Subcommand};
use figment::{
    Figment,
    providers::{Format, Serialized, Toml},
};
use tracing::info;
use std::path::PathBuf;

use llun_core::api_client::{PromptManager, ScannerManager};
use llun_core::data::DEFAULT_CONFIG;
use llun_core::files::FileManager;
use llun_core::formatters::OutputManager;
use llun_core::rules::RuleManager;
use llun_core::per_file_ignorer::PerFileIgnorer;
use llun_core::append_to_file::append_to_file;

pub mod logging;
use logging::init_tracing;

pub mod context_args;
use context_args::{ContextArgs, AgentFormat};

pub mod check_args;
use check_args::CheckArgs;

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

    #[command(about = "Provide architectural context to copilot-instructions.md or AGENTS.md")]
    Context(ContextArgs),
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
        Commands::Context(cli_args) => {
            let config: ContextArgs = Figment::new()
                .merge(Toml::string(DEFAULT_CONFIG))
                .merge(Toml::file("pyproject.toml").nested())
                .merge(Toml::file("llun.toml"))
                .merge(Serialized::defaults(cli_args))
                .select("tool.llun")
                .extract()?;

            init_tracing(config.verbose);
            info!("Beginning context creation...");

            info!("Loading selected rules...");
            let rule_manager = RuleManager::new()?;
            let rules = rule_manager.load_from_cli(config.select, config.extend_select, config.ignore)?;

            info!("Generating agent prompt...");
            let prompt = PromptManager::load_system_prompt("system_prompt_agents.txt")?;
            let contextual_prompt = format!("{}\n{}", prompt, rules);

            let format = config.agent_format.expect("A format must be provided.");
            let target_path = match format {
                AgentFormat::CopilotInstructions => PathBuf::from(".github/copilot-instructions.md"),
                AgentFormat::Agents => PathBuf::from("AGENTS.md"),
            };

            info!("Implementing rules context to {:?}...", target_path);
            append_to_file(&target_path, &contextual_prompt)?;
        }
    }
    Ok(())
}
