# Generic MCP: Multi-Provider Model Context Protocol Server

A high-performance, extensible Model Context Protocol (MCP) server framework built in Rust using Ports and Adapters (Hexagonal) architecture. Supports multiple service providers including Linear, with easy extensibility for GitHub, Jira, and other platforms.

## Features

### Generic MCP Framework
- **Provider-Agnostic Design**: Clean abstraction layer supporting multiple service providers
- **Pluggable Architecture**: Easy to add new providers (GitHub, Jira, etc.)
- **Type-Safe**: Rust's type system ensures reliability and performance
- **Extensible**: Generic domain models with custom fields support

### Provider Support

#### Linear Provider
- **MCP Tools**:
  - `get_assigned_tickets` - Get tickets assigned to a specific user
  - `get_current_user` - Get current authenticated user information  
  - `search_tickets` - Search tickets using text queries
  - `get_ticket` - Get specific ticket by ID
  - `get_workspace` - Get workspace information

- **MCP Resources**:
  - `tickets://assigned` - Current user's assigned tickets
  - `user://current` - Current user information
  - `workspace://current` - Workspace information

- **Architecture**: Clean Ports and Adapters pattern with clear separation of concerns
- **Performance**: Built in Rust for memory safety and high performance
- **Extensible**: Easy to add new Linear API endpoints or other service integrations

## Architecture

Follows strict **Ports and Adapters (Hexagonal Architecture)** pattern for maximum flexibility and testability:

```
src/
├── domain/          # Generic domain objects (Ticket, User, Workspace, etc.)
├── core/            # Pure business logic (Application)
├── ports/           # Interface contracts (TicketService, McpServer traits)
├── adapters/        # MCP protocol implementation 
└── providers/       # Service-specific implementations
    ├── linear/      # Linear API integration
    ├── github/      # GitHub API integration (future)
    └── jira/        # Jira API integration (future)
```

### Key Components

- **Domain Layer**: Generic data structures that work with any ticket system
- **Core Layer**: Provider-agnostic business logic with zero external dependencies
- **Ports Layer**: Trait definitions for service providers and MCP protocol
- **Adapters Layer**: MCP protocol implementation
- **Providers Layer**: Service-specific implementations (Linear, GitHub, Jira, etc.)

## Prerequisites

- Rust 1.70+ with Cargo
- API credentials for your chosen provider:
  - **Linear**: API token from https://linear.app/settings/api
  - **GitHub**: Personal Access Token (future)
  - **Jira**: API token (future)

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd linear-mcp
```

2. Copy environment template:
```bash
cp .env.example .env
```

3. Edit `.env` and add your provider credentials:
```
# Linear Provider
LINEAR_API_TOKEN=your_linear_api_token_here

# GitHub Provider (future)
GITHUB_TOKEN=your_github_token_here

# Jira Provider (future)
JIRA_TOKEN=your_jira_token_here
JIRA_URL=https://yourcompany.atlassian.net

RUST_LOG=info
```

## Quick Start

1. **Setup Environment**:
```bash
# Clone and setup
git clone <repository-url>
cd linear-mcp
cp .env.example .env
# Edit .env with your Linear API token
```

2. **Test Your Setup**:
```bash
# Test provider API connection (Linear by default)
cargo run --bin test_provider

# List available teams/workspaces
cargo run --bin list_teams

# Create example tickets
cargo run --bin create_tickets
```

3. **Run MCP Server**:
```bash
# Start the MCP server for AI assistant integration
cargo run

# Or specify a provider
cargo run -- --provider linear
```

## CLI Commands

The project includes several CLI utilities for interacting with your chosen provider:

### Main Binaries

| Command | Description |
|---------|-------------|
| `cargo run --bin generic-mcp` | Start MCP server for AI assistants |
| `cargo run --bin test_provider` | Test API connection and fetch your assigned tickets |
| `cargo run --bin list_teams` | List all available teams/workspaces |
| `cargo run --bin create_tickets` | Create example tickets for testing |

### Examples

```bash
# Test your provider API setup
cargo run --bin test_provider

# See all teams and find team IDs
cargo run --bin list_teams

# Create structured tickets for a project
cargo run --bin create_tickets

# Run the MCP server (for AI assistant integration)
cargo run --bin generic-mcp

# Run with debug logging
RUST_LOG=debug cargo run --bin test_provider
```

## Usage

### CLI Utilities

Perfect for testing, automation, and one-off operations:

```bash
# Development workflow
cargo build                      # Build all binaries
cargo run --bin test_provider   # Verify API connection
cargo run --bin list_teams      # Explore your workspace
```

### Integration with AI Assistants

The server implements the Model Context Protocol standard and can be integrated with any MCP-compatible AI assistant.

Example configuration for Claude Code:
```json
{
  "mcpServers": {
    "tickets": {
      "command": "/path/to/generic-mcp/target/release/generic-mcp",
      "args": ["--provider", "linear"],
      "env": {
        "LINEAR_API_TOKEN": "your_token_here"
      }
    }
  }
}
```

For different providers:
```json
{
  "mcpServers": {
    "github-tickets": {
      "command": "/path/to/generic-mcp/target/release/generic-mcp",
      "args": ["--provider", "github"],
      "env": {
        "GITHUB_TOKEN": "your_github_token_here"
      }
    },
    "jira-tickets": {
      "command": "/path/to/generic-mcp/target/release/generic-mcp", 
      "args": ["--provider", "jira"],
      "env": {
        "JIRA_TOKEN": "your_jira_token_here",
        "JIRA_URL": "https://yourcompany.atlassian.net"
      }
    }
  }
}
```

## Development

### Running Tests

```bash
cargo test
```

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

### Adding New Providers

1. Create provider module in `src/providers/your_provider/`
2. Implement `TicketService` trait for your provider
3. Add feature flag in `Cargo.toml`
4. Update provider factory in main application
5. Add provider-specific configuration

### Adding New Tools

1. Add generic domain models in `src/domain/`
2. Add business logic in `src/core/application.rs` 
3. Add trait method to `src/ports/ticket_service.rs`
4. Implement in relevant provider adapters
5. Add MCP tool in `src/adapters/mcp_server_impl.rs`

## Dependencies

### Core Framework
- `rmcp` - Official Rust MCP SDK
- `tokio` - Async runtime
- `serde` - JSON serialization
- `anyhow` - Error handling
- `tracing` - Structured logging
- `async-trait` - Async trait support

### Provider Dependencies
- `reqwest` - HTTP client for REST APIs
- `graphql_client` - GraphQL client support
- `chrono` - Date/time handling

## Contributing

### Adding New Providers

1. Fork the repository
2. Create provider module: `src/providers/your_provider/`
3. Implement the `TicketService` trait
4. Add comprehensive tests
5. Update documentation
6. Submit a pull request

### Provider Implementation Guide

```rust
// src/providers/your_provider/adapter.rs
use async_trait::async_trait;
use crate::ports::TicketService;

pub struct YourProviderAdapter {
    client: YourProviderClient,
}

#[async_trait]
impl TicketService for YourProviderAdapter {
    async fn get_assigned_tickets(&self, user_id: &str) -> Result<Vec<Ticket>> {
        // Your implementation here
    }
    
    // ... implement other required methods
}
```

### Architecture Guidelines

1. **Keep Domain Pure**: No external dependencies in domain layer
2. **Provider Isolation**: Each provider is completely independent
3. **Generic First**: Design for multiple providers, not just one
4. **Type Safety**: Leverage Rust's type system for reliability
5. **Test Coverage**: Add unit and integration tests

## License

MIT OR Apache-2.0