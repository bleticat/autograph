pub mod shared;
pub mod tasks;

pub use shared::adapters::rustqlite_database::{RustqliteDatabase, RustqliteTransaction};
pub use shared::error::AppErr;
pub use shared::ports::{Connection, Database, Transaction};
pub use tasks::adapters::{SqliteTaskQueries, SqliteTodoRepository};
pub use tasks::{TaskCommands, TaskQueries, Todo, TodoRepository};
