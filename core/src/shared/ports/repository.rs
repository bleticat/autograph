use crate::shared::error::AppErr;
use std::future::Future;
use uuid::Uuid;

pub trait Repository<'a, Entity> {
    type Tx;

    fn bind(tx: &'a Self::Tx) -> Self
    where
        Self: Sized;

    fn get(&self, id: Uuid) -> impl Future<Output = Result<Option<Entity>, AppErr>> + Send + '_;
    fn save<'b>(
        &'b self,
        entity: &'b Entity,
    ) -> impl Future<Output = Result<Uuid, AppErr>> + Send + 'b;
    fn delete(&self, id: Uuid) -> impl Future<Output = Result<(), AppErr>> + Send + '_;
}
