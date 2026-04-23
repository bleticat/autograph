use crate::card::adapters::seaorm::history::{load_history, serialize_history, to_card};
use crate::card::entity::Card;
use crate::card::repository::CardEventRepository;
use crate::shared::adapters::seaorm::models::card as card_model;
use crate::shared::adapters::seaorm::models::card_history as card_history_model;
use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseTransaction, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

pub struct SeaOrmCardRepository<'a> {
    tx: &'a DatabaseTransaction,
}

impl<'a> SeaOrmCardRepository<'a> {
    pub fn new(tx: &'a DatabaseTransaction) -> Self {
        Self { tx }
    }
}

impl<'a> Repository<Card> for SeaOrmCardRepository<'a> {
    async fn get(&mut self, id: Uuid) -> Result<Option<Card>, AppErr> {
        let Some(model) = card_model::Entity::find_by_id(id).one(self.tx).await? else {
            return Ok(None);
        };

        let history = load_history(self.tx, id).await?;
        Ok(Some(to_card(model, history)))
    }

    async fn save(&mut self, card: Card) -> Result<Card, AppErr> {
        let id = if card.id.is_nil() {
            Uuid::new_v4()
        } else {
            card.id
        };

        let existing = card_model::Entity::find_by_id(id).one(self.tx).await?;
        let active_model = card_model::ActiveModel {
            id: Set(id),
            title: Set(card.title),
            description: Set(card.description),
            deadline: Set(card.deadline),
            deleted: Set(card.deleted),
            project_id: Set(card.project_id),
            section_id: Set(card.section_id),
        };

        let saved = if existing.is_some() {
            active_model.update(self.tx).await?
        } else {
            active_model.insert(self.tx).await?
        };

        let history = load_history(self.tx, saved.id).await?;
        Ok(to_card(saved, history))
    }

    async fn delete(&mut self, id: Uuid) -> Result<(), AppErr> {
        card_model::Entity::delete_by_id(id).exec(self.tx).await?;
        Ok(())
    }
}

impl<'a> CardEventRepository for SeaOrmCardRepository<'a> {
    async fn get_history(
        &mut self,
        id: Uuid,
    ) -> Result<Vec<crate::card::entity::CardHistory>, AppErr> {
        load_history(self.tx, id).await
    }

    async fn append_history(
        &mut self,
        id: Uuid,
        items: Vec<crate::card::entity::CardHistory>,
    ) -> Result<(), AppErr> {
        let mut history = load_history(self.tx, id).await?;
        history.extend(items);
        let items = serialize_history(&history)?;

        if card_history_model::Entity::find_by_id(id)
            .one(self.tx)
            .await?
            .is_some()
        {
            card_history_model::ActiveModel {
                card_id: Set(id),
                items: Set(items),
            }
            .update(self.tx)
            .await?;
        } else {
            card_history_model::ActiveModel {
                card_id: Set(id),
                items: Set(items),
            }
            .insert(self.tx)
            .await?;
        }

        Ok(())
    }

    async fn get_by_section(&mut self, section_id: Uuid) -> Result<Vec<Card>, AppErr> {
        let cards = card_model::Entity::find()
            .filter(card_model::Column::SectionId.eq(section_id))
            .all(self.tx)
            .await?;

        let card_ids = cards.iter().map(|card| card.id).collect::<Vec<_>>();
        let history_map =
            crate::card::adapters::seaorm::history::load_history_map(self.tx, &card_ids).await?;

        Ok(cards
            .into_iter()
            .map(|card| {
                let history = history_map.get(&card.id).cloned().unwrap_or_default();
                to_card(card, history)
            })
            .collect())
    }
}
