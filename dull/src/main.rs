use clap::{Parser, Subcommand};
use include_dir::{include_dir, Dir};
use std::path::PathBuf;
use std::collections::HashSet;

static RULES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/data/rules");
const DEFAULT_RULES: &str = include_str!("../data/default_rules.txt");

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
        select: Option<Vec<String>>, // vec<string> means i can have multiple 'select' calls, Option<> means i can also have none and default
    },
}

fn main() {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Check { path, select } => {
            let valid_rules = validate_rules();  // will probs need to move this into the rulesmanager class once i figure out oop
            let selected_rules = if let Some(list) = select {
                list.clone()
            } else {
                load_default_rules()
            };  // if select is a list with at least one value, use it, else default back

            println!("checking{:?}", path);

            for s in &selected_rules {
                if !valid_rules.contains(s) {
                    eprintln!("Invalid rule name selected {}", s);
                    std::process::exit(1);
                }
                println!("Selected: {}", s);
            }
        }
    }
}

/// validate that the requested rules have matching file names in the rules folder
fn validate_rules() -> HashSet<String> {
    RULES_DIR
        .files()
        .filter_map(|file| {
            file
                .path()
                .file_stem()
                .and_then(|stem| stem.to_str())
                .map(|s| s.to_string())
        })
        .collect()
}

/// load the default rules list into a vector of strings
fn load_default_rules() -> Vec<String> {
    DEFAULT_RULES
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect()
}