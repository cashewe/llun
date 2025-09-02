use std::path::{PathBuf, Path};
use std::collections::HashSet;
use std::fs;
use std::io;

use clap::{Parser, Subcommand};

mod data;
use data::{RULES_DIR, DEFAULT_RULES};

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Check { path, exclude, select, extend_select, ignore } => {
            if !path.exists() {
                eprintln!("Error: Path '{}' does not exist", path.display());
                std::process::exit(1);
            }

            let exclude_set: HashSet<PathBuf> = exclude.into_iter().collect();
            let valid_rules = validate_rules()?;  // will probs need to move this into the rulesmanager class once i figure out oop
            let mut selected_rules = if select.is_empty() {
                load_default_rules()?
            } else {
                select
            };  // if select is a list with at least one value, use it, else default back
            selected_rules.extend(extend_select);

            for rule in &selected_rules {
                if !valid_rules.contains(rule) {
                    eprintln!("Invalid rule name selected {}", rule);
                    std::process::exit(1);
                }
            }

            let filtered_selected_rules: Vec<&String> = selected_rules
                .iter()
                .filter(|rule| !ignore.contains(rule))
                .collect();

            println!("Selected rules: {:?}", filtered_selected_rules);

            let files_to_check = collect_files(&path, &exclude_set)?;
            println!("Files to check:");
            for file in &files_to_check {
                println!("  {}", file.display());
            }

        }
    }

    Ok(())
}

/// validate that the requested rules have matching file names in the rules folder
fn validate_rules() -> Result<HashSet<String>, Box<dyn std::error::Error>> {
    let rules = RULES_DIR
        .files()
        .filter_map(|file| {
            file
                .path()
                .file_stem()
                .and_then(|stem| stem.to_str())
                .map(|s| s.to_string())
        })
        .collect();

    Ok(rules)
}

/// load the default rules list into a vector of strings
fn load_default_rules() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let rules = DEFAULT_RULES
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();
    
    Ok(rules)
}

/// get the selected filepaths
fn collect_files(
    path: &Path,
    exclude_set: &HashSet<PathBuf>
) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    collect_files_recursive(path, exclude_set, &mut files)?;
    Ok(files)
}

/// recursively update the mutable files param
fn collect_files_recursive(
    path: &Path,
    exclude_set: &HashSet<PathBuf>,
    files: &mut Vec<PathBuf>
) -> io::Result<()> {
    if exclude_set.contains(path) {
        return Ok(());
    };

    if path.is_file() {
        files.push(path.to_path_buf());
    } else if path.is_dir() {
        let entries = fs::read_dir(path)?;

        for entry in entries {
            let entry = entry?;
            let entry_path = entry.path();

            if exclude_set.contains(&entry_path) {
                continue;
            }

            collect_files_recursive(&entry_path, exclude_set, files)?;
        }
    }

    Ok(())
}