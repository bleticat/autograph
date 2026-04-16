pub mod projects;
pub mod shared;
pub mod tasks;

pub use projects::adapters::sqlx_project_queries::SqliteProjectQueries;
pub use projects::commands::ProjectCommands;
pub use projects::ports::project_queries::ProjectQueries;
pub use projects::Project;
pub use shared::adapters::database::sqlx_database::{SqlxDatabase, SqlxUnitOfWork};
pub use shared::error::AppErr;
pub use shared::ports::database::Database;
pub use shared::ports::unit_of_work::UnitOfWork;
pub use tasks::adapters::sqlx_task_queries::SqliteTaskQueries;
pub use tasks::commands::TaskCommands;
pub use tasks::ports::task_queries::TaskQueries;
pub use tasks::Todo;
