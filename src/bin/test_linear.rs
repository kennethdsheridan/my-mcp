use anyhow::Result;
use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::EnvFilter;

use generic_mcp::{LinearClient, LinearService};

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

    info!("Fetching current user information...");
    let current_user = linear_client.get_current_user().await?;
    println!("Current User: {} ({})", current_user.name, current_user.email);
    println!("User ID: {}", current_user.id);

    info!("Fetching assigned issues...");
    let assigned_issues = linear_client.get_assigned_issues(&current_user.id).await?;
    
    println!("\n=== TASK SUMMARY FOR {} ===", current_user.name);
    println!("Total assigned issues: {}", assigned_issues.len());
    
    if assigned_issues.is_empty() {
        println!("No issues currently assigned.");
    } else {
        println!("\nAssigned Issues:");
        for (i, issue) in assigned_issues.iter().enumerate() {
            println!("{}. {} - {}", i + 1, issue.identifier, issue.title);
            println!("   Status: {}", issue.state.name);
            println!("   Priority: {:?}", issue.priority);
            if let Some(description) = &issue.description {
                let short_desc = if description.len() > 100 {
                    format!("{}...", &description[..100])
                } else {
                    description.clone()
                };
                println!("   Description: {}", short_desc);
            }
            println!("   URL: {}", issue.url);
            println!();
        }
    }

    Ok(())
}