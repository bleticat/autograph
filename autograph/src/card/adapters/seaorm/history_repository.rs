use crate::card::adapters::seaorm::history::{load_history, serialize_history};
use crate::card::entity::CardHistory;
use crate::card::history_repository::CardHistoryRepository;
use crate::shared::adapters::seaorm::models::card_history as card_history_model;
use crate::shared::error::AppErr;
use sea_orm::{ActiveModelTrait, DatabaseTransaction, EntityTrait, Set};
use uuid::Uuid;

pub struct SeaOrmCardHistoryRepository<'a> {
    tx: &'a DatabaseTransaction,
}

impl<'a> SeaOrmCardHistoryRepository<'a> {
    pub fn new(tx: &'a DatabaseTransaction) -> Self {
        Self { tx }
    }
}

impl<'a> CardHistoryRepository for SeaOrmCardHistoryRepository<'a> {
    async fn get_history(&mut self, id: Uuid) -> Result<Vec<CardHistory>, AppErr> {
        load_history(self.tx, id).await
    }

    async fn append_history(
        &mut self,
        id: Uuid,
        items: Vec<CardHistory>,
    ) -> Result<(), AppErr> {
        let mut history = load_history(self.tx, id).await?;
        history.extend(items);
        let serialized = serialize_history(&history)?;

        if card_history_model::Entity::find_by_id(id)
            .one(self.tx)
            .await?
            .is_some()
        {
            card_history_model::ActiveModel {
                card_id: Set(id),
                items: Set(serialized),
            }
            .update(self.tx)
            .await?;
        } else {
            card_history_model::ActiveModel {
                card_id: Set(id),
                items: Set(serialized),
            }
            .insert(self.tx)
            .await?;
        }

        Ok(())
    }
}
