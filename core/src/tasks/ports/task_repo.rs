use crate::shared::error::AppErr;
use crate::shared::ports::database::Transaction;
use crate::tasks::Todo;
use uuid::Uuid;

pub trait TodoRepository: From<Self::Tx> {
    type Tx: Transaction;
    fn get(&self, id: Uuid) -> Result<Option<Todo>, AppErr>;
    fn save(&self, todo: &Todo) -> Result<Uuid, AppErr>;
    fn delete(&self, id: Uuid) -> Result<(), AppErr>;
}
