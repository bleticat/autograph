use crate::section::entity::Section;
use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use crate::shared::ports::unit_of_work::UnitOfWork;
use uuid::Uuid;

// 'a ties the command handler to the borrowed UnitOfWork, ensuring the handler
// cannot outlive the transaction context it operates on.
pub struct SectionCommands<'a, U: UnitOfWork> {
    uow: &'a mut U,
}

impl<'a, U: UnitOfWork> SectionCommands<'a, U> {
    pub fn new(uow: &'a mut U) -> Self {
        Self { uow }
    }

    pub async fn add(&mut self, title: &str, project_id: Uuid) -> Result<Section, AppErr> {
        self.uow
            .section()
            .save(Section {
                id: Uuid::nil(),
                title: title.to_owned(),
                project_id,
            })
            .await
    }

    pub async fn edit(&mut self, id: Uuid, title: &str) -> Result<(), AppErr> {
        let section = self.uow.section().get(id).await?;
        if let Some(mut section) = section {
            section.title = title.to_owned();
            self.uow.section().save(section).await?;
        }
        Ok(())
    }

    pub async fn delete(&mut self, id: Uuid) -> Result<(), AppErr> {
        self.uow.section().delete(id).await
    }
}
