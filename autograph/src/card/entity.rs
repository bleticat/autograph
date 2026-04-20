use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Card {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub deadline: Option<OffsetDateTime>,
    pub completed: bool,
    pub project_id: Option<Uuid>,
}
