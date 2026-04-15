use crate::projects::ports::project_repo::ProjectRepository;
use crate::shared::error::AppErr;

pub struct ProjectCommands<'a, R: ProjectRepository> {
    projects: &'a R,
}

impl<'a, R: ProjectRepository> ProjectCommands<'a, R> {
    pub fn new(projects: &'a R) -> Self {
        Self { projects }
    }

    pub fn add(&self, title: &str) -> Result<i64, AppErr> {
        self.projects.add(title)
    }
}
