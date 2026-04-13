pub mod commands;
mod db;
pub mod ports;
pub mod queries;

pub use db::SqliteTodoRepository;
pub use ports::TodoRepository;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub completed: bool,
}
