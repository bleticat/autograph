use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Section {
    pub id: Uuid,
    pub title: String,
    pub project_id: Uuid,
}
