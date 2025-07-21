use anyhow::Result;
use dotenv::dotenv;
use std::env;
use tracing::info;
use tracing_subscriber::EnvFilter;

use generic_mcp::{LinearClient, LinearService};
use generic_mcp::domain::{CreateIssueRequest, IssuePriority};

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

    // Get Kenny's user ID
    let current_user = linear_client.get_current_user().await?;
    let kenny_user_id = current_user.id.clone();

    // Get METAL team ID
    let teams = linear_client.get_teams().await?;
    let metal_team_id = teams.iter()
        .find(|t| t.key == "METAL")
        .ok_or_else(|| anyhow::anyhow!("METAL team not found"))?
        .id.clone();

    info!("Creating Digital Ocean IPMI integration issues for user: {} in team: Metal", current_user.name);

    // Main parent issue
    let main_issue = CreateIssueRequest {
        title: "Add Digital Ocean IPMI Power Control API Integration to sysmanager-svr".to_string(),
        description: Some(r#"## Overview

Integrate Digital Ocean's IPMI Power Control API functionality into the existing sysmanager-svr tool to enable remote power management of Digital Ocean GPU nodes.

## Background

Digital Ocean provides IPMI power control API endpoints for managing GPU servers:
- ATL: `https://ipmiatl1.psdev.sh/api/v1/<cluster>/`
- NYC2: `https://ipminy2.psdev.sh/api/v1/<cluster>/`

This integration would allow sysmanager-svr to manage both local hardware and Digital Ocean nodes through a unified interface.

## API Endpoints to Integrate
- `GET /<cluster>/list` - List all servers (hostname, serial)
- `GET /<cluster>/<id>/power/status` - Check power status
- `POST /<cluster>/<id>/power/on` - Power on server
- `POST /<cluster>/<id>/power/off` - Power off server  
- `POST /<cluster>/<id>/power/cycle` - Power cycle server

## Authentication
- Bearer token authentication via `Authorization: Bearer <token>` header
- Token provisioning through one-time access link (7-day validity)

## Technical Specifications

### API Base URLs
- ATL Cluster: `https://ipmiatl1.psdev.sh/api/v1/`
- NYC2 Cluster: `https://ipminy2.psdev.sh/api/v1/`

### Server Identification
- Servers can be identified by `hostname` or `serial` number
- Serial number fallback for renamed hosts: `sudo dmidecode -s system-serial-number`

### Response Formats
```json
// List response
[
  {"hostname": "atl1g1r10bm1", "serial": "7HCTMW3"},
  {"hostname": "atl1g1r10bm2", "serial": "J4G9NW3"}
]

// Status response  
{"hostname": "atl1g1r10bm1", "status": "on"}

// Action response
{"hostname": "atl1g1r10bm1", "status": "initiate cycle"}
```

## Success Criteria
- [ ] Successfully list Digital Ocean servers through sysmanager-svr
- [ ] Perform power operations on Digital Ocean nodes
- [ ] Consistent user experience across local and cloud providers
- [ ] Proper error handling and user feedback
- [ ] Integration tests with actual Digital Ocean API

## Related Issues
- ENG-2064: Add Digital Ocean Provider Support to Parrot VM Orchestrator  
- ENG-2077: Test and Validate on Digital Ocean Hardware
- METAL-37: Consistent BIOS settings

## Repository
https://github.com/sfcompute/sfcompute/tree/main/infra/metal/sysmanager-svr"#.to_string()),
        priority: Some(IssuePriority::High),
        assignee_id: Some(kenny_user_id.clone()),
        team_id: Some(metal_team_id.clone()),
        project_id: None,
        label_ids: None,
        due_date: None,
        estimate: Some(21.0), // 21 story points for the entire epic
    };

    info!("Creating main Digital Ocean IPMI integration issue...");
    let main_issue_result = linear_client.create_issue(&main_issue).await?;
    println!("✅ Created main issue: {} - {}", main_issue_result.identifier, main_issue_result.title);
    println!("   URL: {}", main_issue_result.url);

    // Subtask 1: Configuration
    let config_issue = CreateIssueRequest {
        title: "DO IPMI: Add Digital Ocean provider configuration to sysmanager-svr".to_string(),
        description: Some(r#"## Configuration Tasks

- [ ] Add Digital Ocean provider configuration to sysmanager-svr
- [ ] Support for multiple clusters (ATL, NYC2)
- [ ] Secure token storage and management
- [ ] Environment variable support for API endpoints and tokens

## Implementation Details

### Configuration Structure
```toml
[providers.digital_ocean]
enabled = true
clusters = ["atl", "nyc2"]

[providers.digital_ocean.clusters.atl]
api_base = "https://ipmiatl1.psdev.sh/api/v1/"
token_env = "DO_IPMI_ATL_TOKEN"

[providers.digital_ocean.clusters.nyc2]
api_base = "https://ipminy2.psdev.sh/api/v1/"
token_env = "DO_IPMI_NYC2_TOKEN"
```

### Environment Variables
- `DO_IPMI_ATL_TOKEN` - Bearer token for ATL cluster
- `DO_IPMI_NYC2_TOKEN` - Bearer token for NYC2 cluster

## Acceptance Criteria
- [ ] Configuration file supports Digital Ocean provider
- [ ] Multiple cluster configuration
- [ ] Secure token management
- [ ] Environment variable fallback for tokens"#.to_string()),
        priority: Some(IssuePriority::High),
        assignee_id: Some(kenny_user_id.clone()),
        team_id: Some(metal_team_id.clone()),
        project_id: None,
        label_ids: None,
        due_date: None,
        estimate: Some(5.0),
    };

    info!("Creating configuration subtask...");
    let config_result = linear_client.create_issue(&config_issue).await?;
    println!("✅ Created subtask: {} - {}", config_result.identifier, config_result.title);

    // Subtask 2: API Client Implementation
    let api_client_issue = CreateIssueRequest {
        title: "DO IPMI: Implement HTTP client for Digital Ocean IPMI API".to_string(),
        description: Some(r#"## API Client Implementation Tasks

- [ ] HTTP client for Digital Ocean IPMI API
- [ ] Error handling and retry logic
- [ ] Response parsing and validation
- [ ] Rate limiting consideration

## Implementation Details

### Core API Methods
```rust
pub struct DigitalOceanIPMIClient {
    base_url: String,
    token: String,
    client: reqwest::Client,
}

impl DigitalOceanIPMIClient {
    pub async fn list_servers(&self, cluster: &str) -> Result<Vec<Server>>;
    pub async fn power_status(&self, cluster: &str, server_id: &str) -> Result<PowerStatus>;
    pub async fn power_on(&self, cluster: &str, server_id: &str) -> Result<PowerAction>;
    pub async fn power_off(&self, cluster: &str, server_id: &str) -> Result<PowerAction>;
    pub async fn power_cycle(&self, cluster: &str, server_id: &str) -> Result<PowerAction>;
}
```

### Error Handling
- Network timeout handling
- HTTP status code mapping
- API error response parsing
- Retry logic for transient failures

### Rate Limiting
- Respect API rate limits
- Exponential backoff for retries
- Queue management for bulk operations

## Acceptance Criteria
- [ ] All API endpoints implemented
- [ ] Robust error handling
- [ ] Response parsing and validation
- [ ] Unit tests for client methods
- [ ] Integration tests with mock server"#.to_string()),
        priority: Some(IssuePriority::High),
        assignee_id: Some(kenny_user_id.clone()),
        team_id: Some(metal_team_id.clone()),
        project_id: None,
        label_ids: None,
        due_date: None,
        estimate: Some(8.0),
    };

    info!("Creating API client subtask...");
    let api_client_result = linear_client.create_issue(&api_client_issue).await?;
    println!("✅ Created subtask: {} - {}", api_client_result.identifier, api_client_result.title);

    // Subtask 3: CLI Interface
    let cli_issue = CreateIssueRequest {
        title: "DO IPMI: Add Digital Ocean CLI commands to sysmanager-svr".to_string(),
        description: Some(r#"## CLI Implementation Tasks

- [ ] `sysmanager-svr do list <cluster>` - List Digital Ocean servers
- [ ] `sysmanager-svr do status <cluster> <server>` - Check power status
- [ ] `sysmanager-svr do power-on <cluster> <server>` - Power on server
- [ ] `sysmanager-svr do power-off <cluster> <server>` - Power off server
- [ ] `sysmanager-svr do power-cycle <cluster> <server>` - Power cycle server

## Command Structure

### List Command
```bash
sysmanager-svr do list atl
# Output:
# HOSTNAME        SERIAL    STATUS
# atl1g1r10bm1    7HCTMW3   on
# atl1g1r10bm2    J4G9NW3   off
```

### Status Command  
```bash
sysmanager-svr do status atl atl1g1r10bm1
# Output: atl1g1r10bm1: on
```

### Power Commands
```bash
sysmanager-svr do power-on atl atl1g1r10bm1
sysmanager-svr do power-off atl atl1g1r10bm1
sysmanager-svr do power-cycle atl atl1g1r10bm1
```

## Implementation Details

### Argument Parsing
- Cluster validation (atl, nyc2)
- Server ID validation (hostname or serial)
- Interactive confirmation for destructive operations
- Batch operations support

### Output Formatting
- Table format for list commands
- JSON output option (`--json`)
- Quiet mode (`--quiet`)
- Verbose logging (`--verbose`)

## Acceptance Criteria
- [ ] All CLI commands implemented
- [ ] Input validation and error messages
- [ ] Consistent output formatting
- [ ] Help documentation for each command
- [ ] Tab completion support"#.to_string()),
        priority: Some(IssuePriority::Medium),
        assignee_id: Some(kenny_user_id.clone()),
        team_id: Some(metal_team_id.clone()),
        project_id: None,
        label_ids: None,
        due_date: None,
        estimate: Some(5.0),
    };

    info!("Creating CLI subtask...");
    let cli_result = linear_client.create_issue(&cli_issue).await?;
    println!("✅ Created subtask: {} - {}", cli_result.identifier, cli_result.title);

    // Subtask 4: Integration Features
    let integration_issue = CreateIssueRequest {
        title: "DO IPMI: Add unified server management across local and Digital Ocean nodes".to_string(),
        description: Some(r#"## Integration Features Tasks

- [ ] Unified server listing across local and Digital Ocean nodes
- [ ] Consistent command interface regardless of provider
- [ ] Configuration profiles for different clusters
- [ ] Logging and audit trail for power operations

## Implementation Details

### Unified Server Listing
```bash
sysmanager-svr list --all
# Output combines local and Digital Ocean servers
# PROVIDER    CLUSTER    HOSTNAME        SERIAL    STATUS
# local       -          local-node1     ABC123    on
# do          atl        atl1g1r10bm1    7HCTMW3   on
# do          nyc2       nyc2g1r10bm1    XYZ789    off
```

### Provider Abstraction
```rust
pub trait PowerManager {
    async fn list_servers(&self) -> Result<Vec<Server>>;
    async fn power_status(&self, server_id: &str) -> Result<PowerStatus>;
    async fn power_on(&self, server_id: &str) -> Result<PowerAction>;
    async fn power_off(&self, server_id: &str) -> Result<PowerAction>;
    async fn power_cycle(&self, server_id: &str) -> Result<PowerAction>;
}
```

### Configuration Profiles
- Profile-based configuration for different environments
- Default cluster selection
- Bulk operation profiles

### Audit Trail
- Log all power operations with timestamps
- User identification for operations
- Success/failure tracking
- Export audit logs

## Acceptance Criteria
- [ ] Unified listing across all providers
- [ ] Consistent command interface
- [ ] Configuration profile support
- [ ] Comprehensive audit logging
- [ ] Provider abstraction implemented"#.to_string()),
        priority: Some(IssuePriority::Medium),
        assignee_id: Some(kenny_user_id.clone()),
        team_id: Some(metal_team_id.clone()),
        project_id: None,
        label_ids: None,
        due_date: None,
        estimate: Some(3.0),
    };

    info!("Creating integration subtask...");
    let integration_result = linear_client.create_issue(&integration_issue).await?;
    println!("✅ Created subtask: {} - {}", integration_result.identifier, integration_result.title);

    println!("\n🎉 Successfully created Digital Ocean IPMI integration epic with {} subtasks!", 4);
    println!("\n📋 Summary:");
    println!("Main Issue: {} - {}", main_issue_result.identifier, main_issue_result.title);
    println!("  ├─ {} - {}", config_result.identifier, config_result.title);
    println!("  ├─ {} - {}", api_client_result.identifier, api_client_result.title);
    println!("  ├─ {} - {}", cli_result.identifier, cli_result.title);
    println!("  └─ {} - {}", integration_result.identifier, integration_result.title);
    
    println!("\nAll issues assigned to: {}", current_user.name);
    println!("Total estimated effort: 21 story points");

    Ok(())
}