use async_trait::async_trait;
use anyhow::Result;

use crate::domain::{
    Ticket, TicketFilter, CreateTicketRequest, UpdateTicketRequest,
    User, Team, Label, CreateLabelRequest, Project, ProjectMilestone,
    Workspace
};

/// Generic ticket/issue management service interface
#[async_trait]
pub trait TicketService {
    // Ticket operations
    async fn get_assigned_tickets(&self, user_id: &str) -> Result<Vec<Ticket>>;
    async fn search_tickets(&self, filter: &TicketFilter) -> Result<Vec<Ticket>>;
    async fn get_ticket(&self, ticket_id: &str) -> Result<Option<Ticket>>;
    async fn create_ticket(&self, request: &CreateTicketRequest) -> Result<Ticket>;
    async fn update_ticket(&self, request: &UpdateTicketRequest) -> Result<Ticket>;

    // User operations
    async fn get_current_user(&self) -> Result<User>;
    async fn get_user(&self, user_id: &str) -> Result<Option<User>>;

    // Team operations
    async fn get_teams(&self) -> Result<Vec<Team>>;
    async fn get_team_members(&self, team_id: &str) -> Result<Vec<User>>;

    // Label operations
    async fn get_labels(&self) -> Result<Vec<Label>>;
    async fn create_label(&self, request: &CreateLabelRequest) -> Result<Label>;

    // Project operations
    async fn get_projects(&self) -> Result<Vec<Project>>;
    async fn get_project(&self, project_id: &str) -> Result<Option<Project>>;
    async fn get_project_milestones(&self, project_id: &str) -> Result<Vec<ProjectMilestone>>;

    // Workspace operations
    async fn get_workspace(&self) -> Result<Workspace>;
}

/// Provider-specific configuration
#[derive(Debug, Clone)]
pub struct ProviderConfig {
    pub provider_type: String,
    pub api_token: String,
    pub base_url: Option<String>,
    pub workspace_id: Option<String>,
}