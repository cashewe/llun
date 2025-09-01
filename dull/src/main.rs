use clap::{Parser, Subcommand};
use include_dir::{include_dir, Dir};
use std::path::PathBuf;
use std::collections::HashSet;

static RULES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/data/rules");

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
        #[arg(short, long, required = true)]
        select: Vec<String>, // vec<string> means i can have multiple 'select' calls
    },
}

fn main() {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Check { path, select } => {
            let valid_rules = load_rules();  // will probs need to move this into the rulesmanager class once i figure out oop
            println!("checking{:?}", path);

            for s in select {
                if !valid_rules.contains(s) {
                    eprintln!("Invalid rule name selected {}", s);
                    std::process::exit(1);
                }
                println!("Selected: {}", s);
            }
        }
    }
}

fn load_rules() -> HashSet<String> {
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
