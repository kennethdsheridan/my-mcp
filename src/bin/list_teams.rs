use anyhow::Result;
use dotenv::dotenv;
use std::env;
use tracing::info;
use tracing_subscriber::EnvFilter;

use linear_mcp::{LinearClient, LinearService};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let linear_api_token = env::var("LINEAR_API_TOKEN")
        .map_err(|_| anyhow::anyhow!("LINEAR_API_TOKEN environment variable is required"))?;

    info!("Creating Linear client...");
    let linear_client = LinearClient::new(linear_api_token)?;

    info!("Fetching teams...");
    let teams = linear_client.get_teams().await?;
    
    println!("Available Teams:");
    for team in &teams {
        println!("  {} ({}) - {}", team.name, team.key, team.id);
    }

    println!("\nLooking for METAL team...");
    if let Some(metal_team) = teams.iter().find(|t| t.key == "METAL") {
        println!("✅ Found METAL team: {} ({})", metal_team.name, metal_team.id);
    } else {
        println!("❌ METAL team not found. Available keys: {:?}", 
            teams.iter().map(|t| &t.key).collect::<Vec<_>>());
    }

    Ok(())
}