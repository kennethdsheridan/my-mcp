// Generic service interfaces
pub mod ticket_service;
pub mod mcp_server;

pub use ticket_service::*;
pub use mcp_server::*;

// Legacy Linear-specific interface (for backward compatibility)
pub mod linear_service;
pub use linear_service::*;