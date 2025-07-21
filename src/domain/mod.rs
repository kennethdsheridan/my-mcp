// Generic domain models
pub mod ticket;
pub mod workspace;
pub mod label;
pub mod project;

pub use ticket::*;
pub use workspace::*;
pub use label::*;
pub use project::*;

// Legacy Linear-specific types (for backward compatibility)
pub mod issue;
pub mod user;

pub use issue::*;
pub use user::*;