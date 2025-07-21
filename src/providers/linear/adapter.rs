use async_trait::async_trait;
use anyhow::{Result, anyhow};
use std::collections::HashMap;

use crate::domain::{
    Ticket, TicketFilter, CreateTicketRequest, UpdateTicketRequest,
    Label, CreateLabelRequest, Project, ProjectMilestone, Workspace,
    Priority, State, StateType,
    // Legacy Linear types for mapping
    Issue, IssuePriority, IssueState, IssueStateType
};
use crate::domain::workspace::Team;
use crate::domain::workspace::User;
use crate::ports::{TicketService, ProviderConfig, LinearService};
use crate::adapters::LinearClient;

pub struct LinearAdapter {
    client: LinearClient,
}

impl LinearAdapter {
    pub fn new(config: ProviderConfig) -> Result<Self> {
        if config.provider_type != "linear" {
            return Err(anyhow!("Invalid provider type for LinearAdapter: {}", config.provider_type));
        }
        
        let client = LinearClient::new(config.api_token)?;
        Ok(Self { client })
    }

    fn map_issue_to_ticket(&self, issue: Issue) -> Ticket {
        Ticket {
            id: issue.id,
            identifier: issue.identifier,
            title: issue.title,
            description: issue.description,
            priority: self.map_issue_priority_to_priority(issue.priority),
            state: self.map_issue_state_to_state(issue.state),
            assignee_id: issue.assignee_id,
            creator_id: issue.creator_id,
            project_id: issue.project_id,
            labels: issue.labels,
            created_at: issue.created_at,
            updated_at: issue.updated_at,
            due_date: issue.due_date,
            estimate: issue.estimate,
            url: issue.url,
            custom_fields: HashMap::new(),
        }
    }

    fn map_issue_priority_to_priority(&self, priority: IssuePriority) -> Priority {
        match priority {
            IssuePriority::NoPriority => Priority::None,
            IssuePriority::Urgent => Priority::Highest,
            IssuePriority::High => Priority::High,
            IssuePriority::Medium => Priority::Medium,
            IssuePriority::Low => Priority::Low,
        }
    }

    fn map_issue_state_to_state(&self, state: IssueState) -> State {
        State {
            id: state.id,
            name: state.name,
            type_: self.map_issue_state_type_to_state_type(state.type_),
            position: state.position,
        }
    }

    fn map_issue_state_type_to_state_type(&self, state_type: IssueStateType) -> StateType {
        match state_type {
            IssueStateType::Unstarted => StateType::Open,
            IssueStateType::Started => StateType::InProgress,
            IssueStateType::Completed => StateType::Closed,
            IssueStateType::Canceled => StateType::Cancelled,
        }
    }

    fn map_priority_to_issue_priority(&self, priority: Priority) -> IssuePriority {
        match priority {
            Priority::None => IssuePriority::NoPriority,
            Priority::Lowest | Priority::Low => IssuePriority::Low,
            Priority::Medium => IssuePriority::Medium,
            Priority::High => IssuePriority::High,
            Priority::Highest => IssuePriority::Urgent,
            Priority::Custom(_) => IssuePriority::Medium,
        }
    }
}

#[async_trait]
impl TicketService for LinearAdapter {
    async fn get_assigned_tickets(&self, user_id: &str) -> Result<Vec<Ticket>> {
        let issues = self.client.get_assigned_issues(user_id).await?;
        Ok(issues.into_iter().map(|issue| self.map_issue_to_ticket(issue)).collect())
    }

    async fn search_tickets(&self, filter: &TicketFilter) -> Result<Vec<Ticket>> {
        // Map generic filter to Linear-specific filter
        let linear_filter = crate::domain::IssueFilter {
            assignee_id: filter.assignee_id.clone(),
            project_id: filter.project_id.clone(),
            state_type: filter.state_type.as_ref().map(|st| match st {
                StateType::Open => IssueStateType::Unstarted,
                StateType::InProgress => IssueStateType::Started,
                StateType::Closed => IssueStateType::Completed,
                StateType::Cancelled => IssueStateType::Canceled,
                StateType::Custom(_) => IssueStateType::Unstarted, // Default mapping
            }),
            priority: filter.priority.as_ref().map(|p| self.map_priority_to_issue_priority(p.clone())),
            labels: filter.labels.clone(),
            search_query: filter.search_query.clone(),
        };

        let issues = self.client.search_issues(&linear_filter).await?;
        Ok(issues.into_iter().map(|issue| self.map_issue_to_ticket(issue)).collect())
    }

    async fn get_ticket(&self, ticket_id: &str) -> Result<Option<Ticket>> {
        let issue_opt = self.client.get_issue(ticket_id).await?;
        Ok(issue_opt.map(|issue| self.map_issue_to_ticket(issue)))
    }

    async fn create_ticket(&self, request: &CreateTicketRequest) -> Result<Ticket> {
        // Map generic request to Linear-specific request
        let linear_request = crate::domain::CreateIssueRequest {
            title: request.title.clone(),
            description: request.description.clone(),
            priority: request.priority.as_ref().map(|p| self.map_priority_to_issue_priority(p.clone())),
            assignee_id: request.assignee_id.clone(),
            team_id: request.team_id.clone(),
            project_id: request.project_id.clone(),
            label_ids: request.label_ids.clone(),
            due_date: request.due_date,
            estimate: request.estimate,
        };

        let issue = self.client.create_issue(&linear_request).await?;
        Ok(self.map_issue_to_ticket(issue))
    }

    async fn update_ticket(&self, request: &UpdateTicketRequest) -> Result<Ticket> {
        // Map generic request to Linear-specific request
        let linear_request = crate::domain::UpdateIssueRequest {
            id: request.id.clone(),
            title: request.title.clone(),
            description: request.description.clone(),
            priority: request.priority.as_ref().map(|p| self.map_priority_to_issue_priority(p.clone())),
            assignee_id: request.assignee_id.clone(),
            state_id: request.state_id.clone(),
            label_ids: request.label_ids.clone(),
            due_date: request.due_date,
            estimate: request.estimate,
        };

        let issue = self.client.update_issue(&linear_request).await?;
        Ok(self.map_issue_to_ticket(issue))
    }

    async fn get_current_user(&self) -> Result<User> {
        self.client.get_current_user().await
    }

    async fn get_user(&self, user_id: &str) -> Result<Option<User>> {
        // Linear client doesn't have get_user method yet
        todo!("Implement get_user in LinearClient first")
    }

    async fn get_teams(&self) -> Result<Vec<Team>> {
        self.client.get_teams().await
    }

    async fn get_team_members(&self, team_id: &str) -> Result<Vec<User>> {
        self.client.get_team_members(team_id).await
    }

    async fn get_labels(&self) -> Result<Vec<Label>> {
        self.client.get_labels().await
    }

    async fn create_label(&self, request: &CreateLabelRequest) -> Result<Label> {
        self.client.create_label(request).await
    }

    async fn get_projects(&self) -> Result<Vec<Project>> {
        self.client.get_projects().await
    }

    async fn get_project(&self, project_id: &str) -> Result<Option<Project>> {
        self.client.get_project(project_id).await
    }

    async fn get_project_milestones(&self, project_id: &str) -> Result<Vec<ProjectMilestone>> {
        self.client.get_project_milestones(project_id).await
    }

    async fn get_workspace(&self) -> Result<Workspace> {
        // Linear doesn't have a direct workspace concept, so we'll construct one
        let user = self.get_current_user().await?;
        let teams = self.get_teams().await?;
        
        Ok(Workspace {
            id: "linear-workspace".to_string(),
            name: format!("{}'s Linear Workspace", user.name),
            description: Some("Linear workspace".to_string()),
            url: "https://linear.app".to_string(),
            teams,
            custom_fields: HashMap::new(),
        })
    }
}