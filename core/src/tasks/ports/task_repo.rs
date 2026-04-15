use crate::shared::error::AppErr;
use crate::shared::ports::database::Transaction;
use crate::tasks::Todo;

pub enum TodoSave {
    Upsert(Todo),
    Delete(i64),
}

pub trait TodoRepository: From<Self::Tx> {
    type Tx: Transaction;
    fn get(&self, id: i64) -> Result<Option<Todo>, AppErr>;
    fn save(&self, change: TodoSave) -> Result<i64, AppErr>;
}
