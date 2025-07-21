use anyhow::Result;
use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use tracing::{info, error};
use tracing_subscriber::{fmt, EnvFilter};

use generic_mcp::{
    Application,
    McpServerImpl,
    McpServer,
    ProviderConfig,
};

#[cfg(feature = "linear")]
use generic_mcp::providers::LinearAdapter;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("Starting generic-mcp server...");

    // Default to Linear provider for now
    let provider = env::var("MCP_PROVIDER").unwrap_or_else(|_| "linear".to_string());
    
    let ticket_service = match provider.as_str() {
        #[cfg(feature = "linear")]
        "linear" => {
            let linear_api_token = env::var("LINEAR_API_TOKEN")
                .map_err(|_| anyhow::anyhow!("LINEAR_API_TOKEN environment variable is required for Linear provider"))?;
            
            let config = ProviderConfig {
                provider_type: "linear".to_string(),
                api_token: linear_api_token,
                base_url: None,
                workspace_id: None,
            };
            
            info!("Creating Linear provider adapter...");
            Arc::new(LinearAdapter::new(config)?) as Arc<dyn generic_mcp::TicketService + Send + Sync>
        },
        _ => {
            return Err(anyhow::anyhow!("Unsupported provider: {}. Available providers: linear", provider));
        }
    };

    info!("Creating application...");
    let application = Arc::new(Application::new(ticket_service));

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