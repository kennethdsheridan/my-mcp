use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    pub id: String,
    pub identifier: String,
    pub title: String,
    pub description: Option<String>,
    pub priority: Priority,
    pub state: State,
    pub assignee_id: Option<String>,
    pub creator_id: String,
    pub project_id: Option<String>,
    pub labels: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub due_date: Option<DateTime<Utc>>,
    pub estimate: Option<f32>,
    pub url: String,
    pub custom_fields: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub id: String,
    pub name: String,
    pub type_: StateType,
    pub position: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StateType {
    Open,
    InProgress,
    Closed,
    Cancelled,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    None,
    Lowest,
    Low,
    Medium,
    High,
    Highest,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketFilter {
    pub assignee_id: Option<String>,
    pub project_id: Option<String>,
    pub state_type: Option<StateType>,
    pub priority: Option<Priority>,
    pub labels: Option<Vec<String>>,
    pub search_query: Option<String>,
    pub custom_filters: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTicketRequest {
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<Priority>,
    pub assignee_id: Option<String>,
    pub team_id: Option<String>,
    pub project_id: Option<String>,
    pub label_ids: Option<Vec<String>>,
    pub due_date: Option<DateTime<Utc>>,
    pub estimate: Option<f32>,
    pub custom_fields: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTicketRequest {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub priority: Option<Priority>,
    pub assignee_id: Option<String>,
    pub state_id: Option<String>,
    pub label_ids: Option<Vec<String>>,
    pub due_date: Option<DateTime<Utc>>,
    pub estimate: Option<f32>,
    pub custom_fields: Option<HashMap<String, serde_json::Value>>,
}