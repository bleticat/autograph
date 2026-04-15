use crate::shared::error::AppErr;
use crate::shared::ports::database::Transaction;
use uuid::Uuid;

#[allow(async_fn_in_trait)]
pub trait Repository<Entity>: From<Self::Tx> {
    type Tx: Transaction;
    async fn get(&self, id: Uuid) -> Result<Option<Entity>, AppErr>;
    async fn save(&self, entity: &Entity) -> Result<Uuid, AppErr>;
    async fn delete(&self, id: Uuid) -> Result<(), AppErr>;
}
