use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub id: String,
    pub identifier: String,
    pub title: String,
    pub description: Option<String>,
    pub priority: IssuePriority,
    pub state: IssueState,
    pub assignee_id: Option<String>,
    pub creator_id: String,
    pub project_id: Option<String>,
    pub labels: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub due_date: Option<DateTime<Utc>>,
    pub estimate: Option<f32>,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueState {
    pub id: String,
    pub name: String,
    pub type_: IssueStateType,
    pub position: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueStateType {
    Unstarted,
    Started,
    Completed,
    Canceled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssuePriority {
    NoPriority,
    Urgent,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueFilter {
    pub assignee_id: Option<String>,
    pub project_id: Option<String>,
    pub state_type: Option<IssueStateType>,
    pub priority: Option<IssuePriority>,
    pub labels: Option<Vec<String>>,
    pub search_query: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIssueRequest {
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<IssuePriority>,
    pub assignee_id: Option<String>,
    pub project_id: Option<String>,
    pub label_ids: Option<Vec<String>>,
    pub due_date: Option<DateTime<Utc>>,
    pub estimate: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateIssueRequest {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub priority: Option<IssuePriority>,
    pub assignee_id: Option<String>,
    pub state_id: Option<String>,
    pub label_ids: Option<Vec<String>>,
    pub due_date: Option<DateTime<Utc>>,
    pub estimate: Option<f32>,
}