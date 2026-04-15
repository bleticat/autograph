pub mod adapters;
pub mod commands;
pub mod ports;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    pub title: String,
}
