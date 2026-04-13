pub mod error;
pub mod shared;
pub mod tasks;

pub use error::CoreError;
pub use shared::ports::{Connection, Database, Transaction};
pub use shared::sqlite::{SqliteDatabase, SqliteTransaction};
pub use tasks::sqlite::{SqliteTaskQueries, SqliteTodoRepository};
pub use tasks::{TaskCommands, TaskQueries, Todo, TodoRepository};
