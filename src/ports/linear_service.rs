use async_trait::async_trait;
use anyhow::Result;

use crate::domain::{
    Issue, IssueFilter, CreateIssueRequest, UpdateIssueRequest,
    User, Team, Label, CreateLabelRequest, Project, ProjectMilestone
};

#[async_trait]
pub trait LinearService {
    async fn get_assigned_issues(&self, user_id: &str) -> Result<Vec<Issue>>;
    
    async fn search_issues(&self, filter: &IssueFilter) -> Result<Vec<Issue>>;
    
    async fn get_issue(&self, issue_id: &str) -> Result<Option<Issue>>;
    
    async fn create_issue(&self, request: &CreateIssueRequest) -> Result<Issue>;
    
    async fn update_issue(&self, request: &UpdateIssueRequest) -> Result<Issue>;
    
    async fn get_current_user(&self) -> Result<User>;
    
    async fn get_teams(&self) -> Result<Vec<Team>>;
    
    async fn get_team_members(&self, team_id: &str) -> Result<Vec<User>>;
    
    async fn get_labels(&self) -> Result<Vec<Label>>;
    
    async fn create_label(&self, request: &CreateLabelRequest) -> Result<Label>;
    
    async fn get_projects(&self) -> Result<Vec<Project>>;
    
    async fn get_project(&self, project_id: &str) -> Result<Option<Project>>;
    
    async fn get_project_milestones(&self, project_id: &str) -> Result<Vec<ProjectMilestone>>;
}