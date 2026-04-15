use crate::projects::Project;
use crate::shared::error::AppErr;
use crate::shared::ports::database::Transaction;

pub trait ProjectRepository: From<Self::Tx> {
    type Tx: Transaction;
    fn get(&self, id: i64) -> Result<Option<Project>, AppErr>;
    fn save(&self, project: &Project) -> Result<i64, AppErr>;
}
