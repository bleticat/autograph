use crate::projects::Project;
use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use uuid::Uuid;

pub struct ProjectCommands<R: Repository<Project>> {
    repo: R,
}

impl<R: Repository<Project>> ProjectCommands<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn add(&mut self, title: &str) -> Result<Project, AppErr> {
        self.repo
            .save(Project {
                id: Uuid::nil(),
                title: title.to_owned(),
            })
            .await
    }
}
