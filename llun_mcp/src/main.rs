use llun_mcp::LlunServer;
use rmcp::{transport::stdio, ServiceExt};
use tracing_subscriber::{self, EnvFilter};
use tracing::{info, error};

/// Run with: npx @modelcontextprotocol/inspector cargo run --bin llun-mcp
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into())
        )
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .with_target(false)
        .compact()
        .init();

    info!("Starting llun MCP server");

    let service = LlunServer::new()
        .serve(stdio())
        .await
        .inspect_err(|e| {
            error!("Server error: {:?}", e);
        })?;

    service.waiting().await?;    
    Ok(())
}