use crate::section::entity::Section;
use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use crate::shared::ports::unit_of_work::UnitOfWork;
use uuid::Uuid;

/// Command facade scoped to a borrowed unit of work.
///
/// `'uow` is explicit because the facade stores `&mut U`; this prevents command
/// operations from outliving the transaction they mutate.
pub struct SectionCommands<'uow, U: UnitOfWork> {
    uow: &'uow mut U,
}

// The impl names `'uow` so `new` can return a facade tied to the incoming
// mutable unit-of-work borrow.
impl<'uow, U: UnitOfWork> SectionCommands<'uow, U> {
    pub fn new(uow: &'uow mut U) -> Self {
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
