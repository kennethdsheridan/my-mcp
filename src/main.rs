use anyhow::Result;
use std::env;
use std::sync::Arc;
use tracing::{info, error};
use tracing_subscriber::{fmt, EnvFilter};

use my_mcp::{
    Application,
    LinearClient,
    McpServerImpl,
    McpServer,
};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("Starting my-mcp server...");

    let linear_api_token = env::var("LINEAR_API_TOKEN")
        .map_err(|_| anyhow::anyhow!("LINEAR_API_TOKEN environment variable is required"))?;

    info!("Creating Linear client...");
    let linear_client = LinearClient::new(linear_api_token)?;
    let linear_service = Arc::new(linear_client);

    info!("Creating application...");
    let application = Arc::new(Application::new(linear_service));

    info!("Creating MCP server...");
    let mcp_server = McpServerImpl::new(application.clone());

    info!("Starting MCP server...");
    mcp_server.start_server().await?;

    info!("MCP server is ready to accept connections");

    tokio::signal::ctrl_c().await?;
    info!("Received shutdown signal");

    mcp_server.stop_server().await?;
    info!("MCP server stopped");

    Ok(())
}