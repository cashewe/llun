use llun_mcp::LlunServer;
use rmcp::{transport::stdio, ServiceExt};
use tracing_subscriber::{self, EnvFilter};
use tracing::info;

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
        .init();

    info!("Starting llun MCP server");

    let service = LlunServer::new()
        .serve(stdio())
        .await
        .inspect_err(|e| {
            tracing::error!("Server error: {:?}", e);
        })?;

    service.waiting().await?;    
    tracing::info!("llun MCP server shut down");
    Ok(())
}