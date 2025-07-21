use async_trait::async_trait;
use anyhow::Result;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct McpTool {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

#[derive(Debug, Clone)]
pub struct McpResource {
    pub uri: String,
    pub name: String,
    pub description: Option<String>,
    pub mime_type: Option<String>,
}

#[async_trait]
pub trait McpServer {
    async fn list_tools(&self) -> Result<Vec<McpTool>>;
    
    async fn call_tool(&self, name: &str, arguments: Value) -> Result<Value>;
    
    async fn list_resources(&self) -> Result<Vec<McpResource>>;
    
    async fn read_resource(&self, uri: &str) -> Result<Value>;
    
    async fn start_server(&self) -> Result<()>;
    
    async fn stop_server(&self) -> Result<()>;
}