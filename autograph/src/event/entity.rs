use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub date: OffsetDateTime,
    pub title: String,
    pub description: String,
    pub project_id: Option<Uuid>,
}
