use crate::projects::Project;
use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use uuid::Uuid;

pub struct ProjectCommands<'a, R: Repository<Project>> {
    projects: &'a R,
}

impl<'a, R: Repository<Project>> ProjectCommands<'a, R> {
    pub fn new(projects: &'a R) -> Self {
        Self { projects }
    }

    pub async fn add(&self, title: &str) -> Result<Project, AppErr> {
        self.projects
            .save(Project {
                id: Uuid::nil(),
                title: title.to_owned(),
            })
            .await
    }
}
