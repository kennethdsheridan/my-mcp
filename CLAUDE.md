# Claude Code Assistant Instructions

This document contains instructions for Claude Code when working on the linear-mcp project.

## Project Overview

Linear-MCP is a Model Context Protocol server built in Rust using Ports and Adapters architecture. It provides Linear API integration through MCP tools and resources.

## Architecture

The project follows strict Ports and Adapters (Hexagonal Architecture) pattern:

```
src/
├── domain/     - Pure domain objects, no dependencies
├── core/       - Pure business logic, no external dependencies  
├── ports/      - Interface traits (contracts)
└── adapters/   - External system implementations
```

## Development Guidelines

### Code Organization
- **Domain**: Keep domain objects pure - only data structures with minimal behavior
- **Core**: Business logic must have NO external dependencies
- **Ports**: Define trait interfaces for external systems
- **Adapters**: Implement port traits for specific technologies

### Adding New Features

When adding Linear API functionality:

1. **Domain Layer**: Add/update domain models in appropriate files
2. **Ports**: Add method signatures to `LinearService` trait
3. **Core**: Add business logic methods to `Application`
4. **Adapters**: Implement in `LinearClient` and expose via `McpServerImpl`

### Dependencies

- Use `anyhow::Result` for error handling
- Use `async-trait` for async traits
- Use `serde` for serialization
- Keep external dependencies in adapters only

### Testing Commands

```bash
cargo build        # Build project
cargo test         # Run tests  
cargo fmt          # Format code
cargo clippy       # Lint code
```

### Environment Setup

Required environment variables:
- `LINEAR_API_TOKEN` - Get from https://linear.app/settings/api
- `RUST_LOG` - Set logging level (debug, info, warn, error)

### Common Tasks

#### Add New MCP Tool
1. Add domain model if needed
2. Add business logic to `Application`
3. Add trait method to `LinearService`  
4. Implement in `LinearClient`
5. Add tool definition and handler to `McpServerImpl`

#### Add New Linear API Endpoint
1. Add GraphQL query to `LinearClient`
2. Add parsing logic for response
3. Add trait method to `LinearService`
4. Add business logic to `Application`

### Architecture Principles

1. **Dependency Direction**: Dependencies flow inward toward domain
2. **Pure Core**: Core has no knowledge of external systems
3. **Interface Segregation**: Small, focused trait interfaces
4. **Dependency Injection**: Dependencies injected at application startup

### Debugging

- Use `tracing::debug!`, `info!`, `warn!`, `error!` for logging
- Set `RUST_LOG=debug` for verbose logging
- Check Linear API responses in `LinearClient` methods

### Code Style

- Follow standard Rust conventions
- Use `cargo fmt` for formatting
- Address `cargo clippy` warnings
- Write descriptive error messages
- Include debug logging for important operations

## Implementation Status

### Completed
- ✅ Basic project structure
- ✅ Domain models (Issue, User, Label, Project)
- ✅ Core application logic
- ✅ Linear GraphQL client (partial)
- ✅ MCP server implementation
- ✅ Main application bootstrap

### TODO
- ⏳ Complete Linear API implementations (search, create, update)
- ⏳ Add comprehensive error handling
- ⏳ Add unit tests
- ⏳ Add integration tests
- ⏳ Performance optimizations
- ⏳ Add more MCP tools (teams, projects, labels)

## Key Files

- `src/main.rs` - Application entry point and dependency injection
- `src/core/application.rs` - Core business logic
- `src/ports/linear_service.rs` - Linear API interface
- `src/adapters/linear_client.rs` - Linear GraphQL implementation  
- `src/adapters/mcp_server_impl.rs` - MCP protocol implementation

Remember to maintain the architecture boundaries and keep the core domain pure!