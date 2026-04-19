pub mod adapters;
pub mod commands;
pub mod ports;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub deadline: Option<String>,
    pub completed: bool,
    pub project_id: Option<Uuid>,
}
