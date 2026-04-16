use crate::shared::error::AppErr;
use std::future::Future;
use uuid::Uuid;

pub trait Repository<Entity> {
    fn get(&self, id: Uuid) -> impl Future<Output = Result<Option<Entity>, AppErr>> + Send + '_;
    fn save<'a>(
        &'a self,
        entity: &'a Entity,
    ) -> impl Future<Output = Result<Uuid, AppErr>> + Send + 'a;
    fn delete(&self, id: Uuid) -> impl Future<Output = Result<(), AppErr>> + Send + '_;
}
