mod db;

pub use db::Database;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub completed: bool,
}
