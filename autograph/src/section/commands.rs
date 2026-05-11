use crate::section::entity::Section;
use crate::shared::error::AppErr;
use crate::shared::ports::database::Database;
use crate::shared::ports::repository::Repository;
use crate::shared::ports::unit_of_work::UnitOfWork;
use autograph_macros::transaction;
use uuid::Uuid;

/// Command facade that holds a reference to the database and manages its own
/// transactions. Each command method opens a transaction, performs its work,
/// and commits (or rolls back on error).
pub struct SectionCommands<'db, D: Database> {
    db: &'db D,
}

impl<'db, D: Database> SectionCommands<'db, D> {
    pub fn new(db: &'db D) -> Self {
        Self { db }
    }

    #[transaction]
    pub async fn add(&self, title: &str, project_id: Uuid) -> Result<Section, AppErr> {
        let title = title.to_owned();
        uow.section()
            .save(Section {
                id: Uuid::nil(),
                title,
                project_id,
            })
            .await
    }

    #[transaction]
    pub async fn edit(&self, id: Uuid, title: &str) -> Result<(), AppErr> {
        let title = title.to_owned();
        let section = uow.section().get(id).await?;
        if let Some(mut section) = section {
            section.title = title;
            uow.section().save(section).await?;
        }
        Ok(())
    }

    #[transaction]
    pub async fn delete(&self, id: Uuid) -> Result<(), AppErr> {
        uow.section().delete(id).await
    }
}
