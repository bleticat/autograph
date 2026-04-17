use crate::projects::Project;
use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use crate::shared::ports::unit_of_work::UnitOfWork;
use uuid::Uuid;

pub struct ProjectCommands<'a, U: UnitOfWork> {
    uow: &'a mut U,
}

impl<'a, U: UnitOfWork> ProjectCommands<'a, U> {
    pub fn new(uow: &'a mut U) -> Self {
        Self { uow }
    }

    pub async fn add(&mut self, title: &str) -> Result<Project, AppErr> {
        self.uow
            .projects()
            .save(Project {
                id: Uuid::nil(),
                title: title.to_owned(),
            })
            .await
    }
}
