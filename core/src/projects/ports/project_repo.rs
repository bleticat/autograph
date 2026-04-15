use crate::projects::Project;
use crate::shared::error::AppErr;
use crate::shared::ports::database::Transaction;
use uuid::Uuid;

pub trait ProjectRepository: From<Self::Tx> {
    type Tx: Transaction;
    fn get(&self, id: Uuid) -> Result<Option<Project>, AppErr>;
    fn save(&self, project: &Project) -> Result<Uuid, AppErr>;
    fn delete(&self, id: Uuid) -> Result<(), AppErr>;
}
