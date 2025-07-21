use async_trait::async_trait;
use anyhow::{Result, anyhow};
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;

use crate::domain::{
    Issue, IssueFilter, CreateIssueRequest, UpdateIssueRequest,
    User, Team, Label, CreateLabelRequest, Project, ProjectMilestone,
    IssuePriority, IssueState, IssueStateType, ProjectState
};
use crate::ports::LinearService;

pub struct LinearClient {
    client: Client,
    api_token: String,
    base_url: String,
}

impl LinearClient {
    pub fn new(api_token: String) -> Result<Self> {
        let client = Client::new();
        let base_url = "https://api.linear.app/graphql".to_string();
        
        Ok(Self {
            client,
            api_token,
            base_url,
        })
    }

    async fn execute_query(&self, query: &str, variables: Option<Value>) -> Result<Value> {
        let mut body = serde_json::json!({
            "query": query
        });

        if let Some(vars) = variables {
            body["variables"] = vars;
        }

        let response = self
            .client
            .post(&self.base_url)
            .header("Authorization", &self.api_token)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow!("GraphQL request failed: {} - {}", status, error_text));
        }

        let json: Value = response.json().await?;
        
        if let Some(errors) = json.get("errors") {
            return Err(anyhow!("GraphQL errors: {}", errors));
        }

        Ok(json.get("data").unwrap_or(&Value::Null).clone())
    }

    fn parse_issue(&self, issue_data: &Value) -> Result<Issue> {
        let id = issue_data["id"].as_str().unwrap_or_default().to_string();
        let identifier = issue_data["identifier"].as_str().unwrap_or_default().to_string();
        let title = issue_data["title"].as_str().unwrap_or_default().to_string();
        let description = issue_data["description"].as_str().map(|s| s.to_string());
        let url = issue_data["url"].as_str().unwrap_or_default().to_string();
        
        let priority = match issue_data["priority"].as_u64() {
            Some(0) => IssuePriority::NoPriority,
            Some(1) => IssuePriority::Urgent,
            Some(2) => IssuePriority::High,
            Some(3) => IssuePriority::Medium,
            Some(4) => IssuePriority::Low,
            _ => IssuePriority::NoPriority,
        };

        let state = IssueState {
            id: issue_data["state"]["id"].as_str().unwrap_or_default().to_string(),
            name: issue_data["state"]["name"].as_str().unwrap_or_default().to_string(),
            type_: match issue_data["state"]["type"].as_str() {
                Some("unstarted") => IssueStateType::Unstarted,
                Some("started") => IssueStateType::Started,
                Some("completed") => IssueStateType::Completed,
                Some("canceled") => IssueStateType::Canceled,
                _ => IssueStateType::Unstarted,
            },
            position: issue_data["state"]["position"].as_f64().unwrap_or(0.0) as f32,
        };

        let assignee_id = issue_data["assignee"]["id"].as_str().map(|s| s.to_string());
        let creator_id = issue_data["creator"]["id"].as_str().unwrap_or_default().to_string();
        let project_id = issue_data["project"]["id"].as_str().map(|s| s.to_string());

        let labels: Vec<String> = issue_data["labels"]["nodes"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|label| label["name"].as_str())
            .map(|s| s.to_string())
            .collect();

        let created_at = chrono::DateTime::parse_from_rfc3339(
            issue_data["createdAt"].as_str().unwrap_or("1970-01-01T00:00:00Z")
        )?.with_timezone(&chrono::Utc);

        let updated_at = chrono::DateTime::parse_from_rfc3339(
            issue_data["updatedAt"].as_str().unwrap_or("1970-01-01T00:00:00Z")
        )?.with_timezone(&chrono::Utc);

        let due_date = issue_data["dueDate"].as_str()
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&chrono::Utc));

        let estimate = issue_data["estimate"].as_f64().map(|e| e as f32);

        Ok(Issue {
            id,
            identifier,
            title,
            description,
            priority,
            state,
            assignee_id,
            creator_id,
            project_id,
            labels,
            created_at,
            updated_at,
            due_date,
            estimate,
            url,
        })
    }
}

#[async_trait]
impl LinearService for LinearClient {
    async fn get_assigned_issues(&self, user_id: &str) -> Result<Vec<Issue>> {
        let query = r#"
            query GetAssignedIssues($userId: String!) {
                user(id: $userId) {
                    assignedIssues {
                        nodes {
                            id
                            identifier
                            title
                            description
                            priority
                            url
                            createdAt
                            updatedAt
                            dueDate
                            estimate
                            state {
                                id
                                name
                                type
                                position
                            }
                            assignee {
                                id
                                name
                            }
                            creator {
                                id
                                name
                            }
                            project {
                                id
                                name
                            }
                            labels {
                                nodes {
                                    id
                                    name
                                }
                            }
                        }
                    }
                }
            }
        "#;

        let variables = serde_json::json!({
            "userId": user_id
        });

        let data = self.execute_query(query, Some(variables)).await?;
        let issues_data = data["user"]["assignedIssues"]["nodes"].as_array()
            .ok_or_else(|| anyhow!("Invalid response format"))?;

        let mut issues = Vec::new();
        for issue_data in issues_data {
            issues.push(self.parse_issue(issue_data)?);
        }

        Ok(issues)
    }

    async fn search_issues(&self, _filter: &IssueFilter) -> Result<Vec<Issue>> {
        todo!("Implement search_issues")
    }

    async fn get_issue(&self, issue_id: &str) -> Result<Option<Issue>> {
        let query = r#"
            query GetIssue($id: String!) {
                issue(id: $id) {
                    id
                    identifier
                    title
                    description
                    priority
                    url
                    createdAt
                    updatedAt
                    dueDate
                    estimate
                    state {
                        id
                        name
                        type
                        position
                    }
                    assignee {
                        id
                        name
                    }
                    creator {
                        id
                        name
                    }
                    project {
                        id
                        name
                    }
                    labels {
                        nodes {
                            id
                            name
                        }
                    }
                }
            }
        "#;

        let variables = serde_json::json!({
            "id": issue_id
        });

        let data = self.execute_query(query, Some(variables)).await?;
        
        if data["issue"].is_null() {
            return Ok(None);
        }

        let issue = self.parse_issue(&data["issue"])?;
        Ok(Some(issue))
    }

    async fn create_issue(&self, _request: &CreateIssueRequest) -> Result<Issue> {
        todo!("Implement create_issue")
    }

    async fn update_issue(&self, _request: &UpdateIssueRequest) -> Result<Issue> {
        todo!("Implement update_issue")
    }

    async fn get_current_user(&self) -> Result<User> {
        let query = r#"
            query GetCurrentUser {
                viewer {
                    id
                    name
                    email
                    avatarUrl
                    displayName
                    active
                }
            }
        "#;

        let data = self.execute_query(query, None).await?;
        let user_data = &data["viewer"];

        Ok(User {
            id: user_data["id"].as_str().unwrap_or_default().to_string(),
            name: user_data["name"].as_str().unwrap_or_default().to_string(),
            email: user_data["email"].as_str().unwrap_or_default().to_string(),
            avatar_url: user_data["avatarUrl"].as_str().map(|s| s.to_string()),
            display_name: user_data["displayName"].as_str().unwrap_or_default().to_string(),
            active: user_data["active"].as_bool().unwrap_or(true),
        })
    }

    async fn get_teams(&self) -> Result<Vec<Team>> {
        todo!("Implement get_teams")
    }

    async fn get_team_members(&self, _team_id: &str) -> Result<Vec<User>> {
        todo!("Implement get_team_members")
    }

    async fn get_labels(&self) -> Result<Vec<Label>> {
        todo!("Implement get_labels")
    }

    async fn create_label(&self, _request: &CreateLabelRequest) -> Result<Label> {
        todo!("Implement create_label")
    }

    async fn get_projects(&self) -> Result<Vec<Project>> {
        todo!("Implement get_projects")
    }

    async fn get_project(&self, _project_id: &str) -> Result<Option<Project>> {
        todo!("Implement get_project")
    }

    async fn get_project_milestones(&self, _project_id: &str) -> Result<Vec<ProjectMilestone>> {
        todo!("Implement get_project_milestones")
    }
}
