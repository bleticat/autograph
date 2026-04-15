use crate::shared::error::AppErr;
use crate::shared::ports::database::Transaction;
use crate::tasks::Todo;

pub trait TodoRepository: From<Self::Tx> {
    type Tx: Transaction;
    fn get(&self, id: i64) -> Result<Option<Todo>, AppErr>;
    fn save(&self, todo: &Todo) -> Result<i64, AppErr>;
    fn delete(&self, id: i64) -> Result<(), AppErr>;
}
