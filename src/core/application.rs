use anyhow::Result;
use std::sync::Arc;
use tracing::{info, debug};

use crate::domain::{Issue, IssueFilter, User, IssueStateType};
use crate::ports::LinearService;

pub struct Application {
    linear_service: Arc<dyn LinearService + Send + Sync>,
}

impl Application {
    pub fn new(linear_service: Arc<dyn LinearService + Send + Sync>) -> Self {
        Self { linear_service }
    }

    pub async fn get_assigned_issues(&self, user_id: &str) -> Result<Vec<Issue>> {
        debug!("Getting assigned issues for user: {}", user_id);
        let issues = self.linear_service.get_assigned_issues(user_id).await?;
        info!("Retrieved {} assigned issues for user {}", issues.len(), user_id);
        Ok(issues)
    }

    pub async fn get_current_user(&self) -> Result<User> {
        debug!("Getting current user information");
        let user = self.linear_service.get_current_user().await?;
        info!("Retrieved current user: {}", user.name);
        Ok(user)
    }

    pub async fn search_issues(&self, query: &str) -> Result<Vec<Issue>> {
        debug!("Searching issues with query: {}", query);
        
        let filter = IssueFilter {
            assignee_id: None,
            project_id: None,
            state_type: None,
            priority: None,
            labels: None,
            search_query: Some(query.to_string()),
        };

        let issues = self.linear_service.search_issues(&filter).await?;
        info!("Found {} issues for query: {}", issues.len(), query);
        Ok(issues)
    }

    pub async fn get_issue(&self, issue_id: &str) -> Result<Option<Issue>> {
        debug!("Getting issue: {}", issue_id);
        let issue = self.linear_service.get_issue(issue_id).await?;
        
        match &issue {
            Some(i) => info!("Retrieved issue: {} - {}", i.identifier, i.title),
            None => info!("Issue not found: {}", issue_id),
        }
        
        Ok(issue)
    }

    pub async fn get_my_active_issues(&self) -> Result<Vec<Issue>> {
        debug!("Getting active issues for current user");
        let user = self.get_current_user().await?;
        let all_issues = self.get_assigned_issues(&user.id).await?;
        
        let active_issues: Vec<Issue> = all_issues
            .into_iter()
            .filter(|issue| match issue.state.type_ {
                IssueStateType::Unstarted | IssueStateType::Started => true,
                IssueStateType::Completed | IssueStateType::Canceled => false,
            })
            .collect();

        info!("Retrieved {} active issues for current user", active_issues.len());
        Ok(active_issues)
    }
}