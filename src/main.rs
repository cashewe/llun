use std::env;
use tynnu::run_with_args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    match run_with_args(args).await {
        Ok(exit_code) => {
            if exit_code != 0 {
                std::process::exit(exit_code);
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
    Ok(())
}