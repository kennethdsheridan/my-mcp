use anyhow::Result;
use std::sync::Arc;
use tracing::{info, debug};

use crate::domain::{Ticket, TicketFilter, StateType, Workspace};
use crate::domain::workspace::User;
use crate::ports::TicketService;

pub struct Application {
    ticket_service: Arc<dyn TicketService + Send + Sync>,
}

impl Application {
    pub fn new(ticket_service: Arc<dyn TicketService + Send + Sync>) -> Self {
        Self { ticket_service }
    }

    pub async fn get_assigned_tickets(&self, user_id: &str) -> Result<Vec<Ticket>> {
        debug!("Getting assigned tickets for user: {}", user_id);
        let tickets = self.ticket_service.get_assigned_tickets(user_id).await?;
        info!("Retrieved {} assigned tickets for user {}", tickets.len(), user_id);
        Ok(tickets)
    }

    pub async fn get_current_user(&self) -> Result<User> {
        debug!("Getting current user information");
        let user = self.ticket_service.get_current_user().await?;
        info!("Retrieved current user: {}", user.name);
        Ok(user)
    }

    pub async fn search_tickets(&self, query: &str) -> Result<Vec<Ticket>> {
        debug!("Searching tickets with query: {}", query);
        
        let filter = TicketFilter {
            assignee_id: None,
            project_id: None,
            state_type: None,
            priority: None,
            labels: None,
            search_query: Some(query.to_string()),
            custom_filters: std::collections::HashMap::new(),
        };

        let tickets = self.ticket_service.search_tickets(&filter).await?;
        info!("Found {} tickets for query: {}", tickets.len(), query);
        Ok(tickets)
    }

    pub async fn get_ticket(&self, ticket_id: &str) -> Result<Option<Ticket>> {
        debug!("Getting ticket: {}", ticket_id);
        let ticket = self.ticket_service.get_ticket(ticket_id).await?;
        
        match &ticket {
            Some(t) => info!("Retrieved ticket: {} - {}", t.identifier, t.title),
            None => info!("Ticket not found: {}", ticket_id),
        }
        
        Ok(ticket)
    }

    pub async fn get_my_active_tickets(&self) -> Result<Vec<Ticket>> {
        debug!("Getting active tickets for current user");
        let user = self.get_current_user().await?;
        let all_tickets = self.get_assigned_tickets(&user.id).await?;
        
        let active_tickets: Vec<Ticket> = all_tickets
            .into_iter()
            .filter(|ticket| match ticket.state.type_ {
                StateType::Open | StateType::InProgress => true,
                StateType::Closed | StateType::Cancelled => false,
                StateType::Custom(_) => true, // Include custom states as active by default
            })
            .collect();

        info!("Retrieved {} active tickets for current user", active_tickets.len());
        Ok(active_tickets)
    }

    pub async fn get_workspace(&self) -> Result<Workspace> {
        debug!("Getting workspace information");
        let workspace = self.ticket_service.get_workspace().await?;
        info!("Retrieved workspace: {}", workspace.name);
        Ok(workspace)
    }
}