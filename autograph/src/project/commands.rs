use crate::project::entity::Project;
use crate::shared::error::AppErr;
use crate::shared::ports::database::Database;
use crate::shared::ports::repository::Repository;
use crate::shared::ports::unit_of_work::UnitOfWork;
use uuid::Uuid;

/// Command facade that holds a reference to the database and manages its own
/// transactions. Each command method opens a transaction, performs its work,
/// and commits (or rolls back on error).
pub struct ProjectCommands<'db, D: Database> {
    db: &'db D,
}

impl<'db, D: Database> ProjectCommands<'db, D> {
    pub fn new(db: &'db D) -> Self {
        Self { db }
    }

    pub async fn add(&self, title: &str) -> Result<Project, AppErr> {
        let title = title.to_owned();
        self.db
            .begin(async move |uow| {
                uow.project()
                    .save(Project {
                        id: Uuid::nil(),
                        title,
                    })
                    .await
            })
            .await
    }
}
