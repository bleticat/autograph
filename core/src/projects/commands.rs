use crate::projects::ports::project_repo::ProjectRepository;
use crate::projects::Project;
use crate::shared::error::AppErr;
use futures::executor::block_on;
use uuid::Uuid;

pub struct ProjectCommands<'a, R: ProjectRepository> {
    projects: &'a R,
}

impl<'a, R: ProjectRepository> ProjectCommands<'a, R> {
    pub fn new(projects: &'a R) -> Self {
        Self { projects }
    }

    pub fn add(&self, title: &str) -> Result<Uuid, AppErr> {
        block_on(self.projects.save(&Project {
            id: Uuid::nil(),
            title: title.to_owned(),
        }))
    }
}
