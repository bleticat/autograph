pub mod shared;
pub mod tasks;

pub use shared::adapters::rustqlite_database::{RustqliteDatabase, RustqliteTransaction};
pub use shared::error::AppErr;
pub use shared::ports::database::{Connection, Database, Transaction};
pub use tasks::adapters::rustqlite_task_queries::SqliteTaskQueries;
pub use tasks::adapters::rustqlite_task_repo::SqliteTodoRepository;
pub use tasks::commands::TaskCommands;
pub use tasks::ports::task_queries::TaskQueries;
pub use tasks::ports::task_repo::TodoRepository;
pub use tasks::Todo;
