use async_trait::async_trait;
use anyhow::{Result, anyhow};
use serde_json::{Value, json};
use std::sync::Arc;
use tracing::{info, error, debug};

use crate::ports::{McpServer, McpTool, McpResource, LinearService};
use crate::core::Application;

pub struct McpServerImpl {
    application: Arc<Application>,
}

impl McpServerImpl {
    pub fn new(application: Arc<Application>) -> Self {
        Self { application }
    }

    fn create_tool_schema(name: &str, description: &str, properties: Value) -> Value {
        json!({
            "type": "object",
            "properties": properties,
            "required": []
        })
    }

    async fn handle_get_assigned_issues(&self, args: Value) -> Result<Value> {
        let user_id = args.get("user_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("user_id is required"))?;

        let issues = self.application.get_assigned_tickets(user_id).await?;
        Ok(json!({
            "issues": issues,
            "count": issues.len()
        }))
    }

    async fn handle_get_current_user(&self) -> Result<Value> {
        let user = self.application.get_current_user().await?;
        Ok(json!({ "user": user }))
    }

    async fn handle_search_issues(&self, args: Value) -> Result<Value> {
        let query = args.get("query")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let issues = self.application.search_tickets(query).await?;
        Ok(json!({
            "issues": issues,
            "count": issues.len(),
            "query": query
        }))
    }

    async fn handle_get_issue(&self, args: Value) -> Result<Value> {
        let issue_id = args.get("issue_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("issue_id is required"))?;

        let issue = self.application.get_ticket(issue_id).await?;
        Ok(json!({ "issue": issue }))
    }
}

#[async_trait]
impl McpServer for McpServerImpl {
    async fn list_tools(&self) -> Result<Vec<McpTool>> {
        Ok(vec![
            McpTool {
                name: "linear_get_assigned_issues".to_string(),
                description: "Get issues assigned to a specific user".to_string(),
                input_schema: Self::create_tool_schema(
                    "linear_get_assigned_issues",
                    "Get assigned issues for a user",
                    json!({
                        "user_id": {
                            "type": "string",
                            "description": "The ID of the user to get assigned issues for"
                        }
                    })
                ),
            },
            McpTool {
                name: "linear_get_current_user".to_string(),
                description: "Get information about the current authenticated user".to_string(),
                input_schema: Self::create_tool_schema(
                    "linear_get_current_user",
                    "Get current user info",
                    json!({})
                ),
            },
            McpTool {
                name: "linear_search_issues".to_string(),
                description: "Search for issues using a text query".to_string(),
                input_schema: Self::create_tool_schema(
                    "linear_search_issues",
                    "Search issues",
                    json!({
                        "query": {
                            "type": "string",
                            "description": "Search query to find issues"
                        }
                    })
                ),
            },
            McpTool {
                name: "linear_get_issue".to_string(),
                description: "Get a specific issue by ID".to_string(),
                input_schema: Self::create_tool_schema(
                    "linear_get_issue",
                    "Get issue by ID",
                    json!({
                        "issue_id": {
                            "type": "string",
                            "description": "The ID of the issue to retrieve"
                        }
                    })
                ),
            },
        ])
    }

    async fn call_tool(&self, name: &str, arguments: Value) -> Result<Value> {
        debug!("Calling tool: {} with arguments: {}", name, arguments);

        let result = match name {
            "linear_get_assigned_issues" => self.handle_get_assigned_issues(arguments).await,
            "linear_get_current_user" => self.handle_get_current_user().await,
            "linear_search_issues" => self.handle_search_issues(arguments).await,
            "linear_get_issue" => self.handle_get_issue(arguments).await,
            _ => Err(anyhow!("Unknown tool: {}", name)),
        };

        match &result {
            Ok(value) => info!("Tool {} completed successfully", name),
            Err(e) => error!("Tool {} failed: {}", name, e),
        }

        result
    }

    async fn list_resources(&self) -> Result<Vec<McpResource>> {
        Ok(vec![
            McpResource {
                uri: "linear://issues/assigned".to_string(),
                name: "Assigned Issues".to_string(),
                description: Some("Issues assigned to the current user".to_string()),
                mime_type: Some("application/json".to_string()),
            },
            McpResource {
                uri: "linear://user/current".to_string(),
                name: "Current User".to_string(),
                description: Some("Information about the current authenticated user".to_string()),
                mime_type: Some("application/json".to_string()),
            },
        ])
    }

    async fn read_resource(&self, uri: &str) -> Result<Value> {
        debug!("Reading resource: {}", uri);

        match uri {
            "linear://issues/assigned" => {
                let user = self.application.get_current_user().await?;
                let issues = self.application.get_assigned_tickets(&user.id).await?;
                Ok(json!({
                    "uri": uri,
                    "mimeType": "application/json",
                    "text": serde_json::to_string_pretty(&issues)?
                }))
            },
            "linear://user/current" => {
                let user = self.application.get_current_user().await?;
                Ok(json!({
                    "uri": uri,
                    "mimeType": "application/json", 
                    "text": serde_json::to_string_pretty(&user)?
                }))
            },
            _ => Err(anyhow!("Unknown resource: {}", uri)),
        }
    }

    async fn start_server(&self) -> Result<()> {
        info!("MCP server starting...");
        Ok(())
    }

    async fn stop_server(&self) -> Result<()> {
        info!("MCP server stopping...");
        Ok(())
    }
}