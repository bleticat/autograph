use crate::project::entity::Project;
use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use crate::shared::ports::unit_of_work::UnitOfWork;
use uuid::Uuid;

// 'a ties the command handler to the borrowed UnitOfWork, ensuring the handler
// cannot outlive the transaction context it operates on.
pub struct ProjectCommands<'a, U: UnitOfWork> {
    uow: &'a mut U,
}

impl<'a, U: UnitOfWork> ProjectCommands<'a, U> {
    pub fn new(uow: &'a mut U) -> Self {
        Self { uow }
    }

    pub async fn add(&mut self, title: &str) -> Result<Project, AppErr> {
        self.uow
            .project()
            .save(Project {
                id: Uuid::nil(),
                title: title.to_owned(),
            })
            .await
    }
}
