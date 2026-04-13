pub mod commands;
pub mod ports;
pub mod sqlite;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub completed: bool,
}

pub use commands::TaskCommands;
pub use ports::{TaskQueries, TodoRepository};
