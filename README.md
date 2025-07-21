# Linear MCP: Linear Model Context Protocol Server

A high-performance Model Context Protocol (MCP) server built in Rust using Ports and Adapters (Hexagonal) architecture for Linear API integration.

## Features

- **MCP Tools**:
  - `linear_get_assigned_issues` - Get issues assigned to a specific user
  - `linear_get_current_user` - Get current authenticated user information
  - `linear_search_issues` - Search issues using text queries
  - `linear_get_issue` - Get specific issue by ID

- **MCP Resources**:
  - `linear://issues/assigned` - Current user's assigned issues
  - `linear://user/current` - Current user information

- **Architecture**: Clean Ports and Adapters pattern with clear separation of concerns
- **Performance**: Built in Rust for memory safety and high performance
- **Extensible**: Easy to add new Linear API endpoints or other service integrations

## Architecture

```
src/
├── domain/          # Pure domain objects (Issue, User, Project, etc.)
├── core/            # Pure business logic (Application)
├── ports/           # Interface contracts (LinearService, McpServer traits)
└── adapters/        # External integrations (LinearClient, McpServerImpl)
```

### Key Components

- **Domain Layer**: Pure data structures representing Linear entities
- **Core Layer**: Business logic with no external dependencies
- **Ports Layer**: Trait definitions for external interfaces
- **Adapters Layer**: Concrete implementations for Linear API and MCP protocol

## Prerequisites

- Rust 1.70+ with Cargo
- Linear API token (get from https://linear.app/settings/api)

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

3. Edit `.env` and add your Linear API token:
```
LINEAR_API_TOKEN=your_linear_api_token_here
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
# Test Linear API connection
cargo run --bin test_linear

# List available teams
cargo run --bin list_teams

# Create example Digital Ocean issues
cargo run --bin create_do_issues
```

3. **Run MCP Server**:
```bash
# Start the MCP server for AI assistant integration
cargo run
```

## CLI Commands

The project includes several CLI utilities for interacting with Linear:

### Main Binaries

| Command | Description |
|---------|-------------|
| `cargo run --bin linear-mcp` | Start MCP server for AI assistants |
| `cargo run --bin test_linear` | Test API connection and fetch your assigned issues |
| `cargo run --bin list_teams` | List all available teams in your Linear workspace |
| `cargo run --bin create_do_issues` | Create Digital Ocean IPMI integration issues |

### Examples

```bash
# Test your Linear API setup
cargo run --bin test_linear

# See all teams and find team IDs
cargo run --bin list_teams

# Create structured issues for a project
cargo run --bin create_do_issues

# Run the MCP server (for AI assistant integration)
cargo run --bin linear-mcp

# Run with debug logging
RUST_LOG=debug cargo run --bin test_linear
```

## Usage

### CLI Utilities

Perfect for testing, automation, and one-off operations:

```bash
# Development workflow
cargo build                    # Build all binaries
cargo run --bin test_linear   # Verify API connection
cargo run --bin list_teams    # Explore your workspace
```

### Integration with AI Assistants

The server implements the Model Context Protocol standard and can be integrated with any MCP-compatible AI assistant.

Example configuration for Claude Code:
```json
{
  "mcpServers": {
    "linear": {
      "command": "/path/to/linear-mcp/target/release/linear-mcp",
      "env": {
        "LINEAR_API_TOKEN": "your_token_here"
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

### Adding New Tools

1. Add domain models in `src/domain/`
2. Add business logic in `src/core/application.rs`
3. Add port trait method in `src/ports/linear_service.rs`
4. Implement in `src/adapters/linear_client.rs`
5. Add MCP tool in `src/adapters/mcp_server_impl.rs`

## Dependencies

- `rmcp` - Official Rust MCP SDK
- `hyper` - HTTP client for Linear GraphQL API
- `tokio` - Async runtime
- `serde` - JSON serialization
- `anyhow` - Error handling
- `tracing` - Structured logging

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes following the existing architecture patterns
4. Add tests for new functionality
5. Submit a pull request

## License

MIT OR Apache-2.0