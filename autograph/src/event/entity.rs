use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub date: DateTime<Utc>,
    pub title: String,
    pub description: String,
    pub project_id: Option<Uuid>,
}
