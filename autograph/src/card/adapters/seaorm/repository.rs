use crate::card::entity::Card;
use crate::shared::adapters::seaorm::models::card as card_model;
use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use sea_orm::{ActiveModelTrait, DatabaseTransaction, EntityTrait, Set};
use uuid::Uuid;

/// Repository scoped to an active SeaORM transaction.
///
/// `'tx` is explicit because the repository stores `&DatabaseTransaction`; it
/// must not outlive the transaction used by its queries.
pub struct SeaOrmCardRepository<'tx> {
    tx: &'tx DatabaseTransaction,
}

// The impl names `'tx` so `new` can return a repository tied to the incoming
// transaction borrow.
impl<'tx> SeaOrmCardRepository<'tx> {
    pub fn new(tx: &'tx DatabaseTransaction) -> Self {
        Self { tx }
    }
}

// `'_` makes the trait impl cover repositories tied to any transaction borrow.
impl Repository<Card> for SeaOrmCardRepository<'_> {
    async fn get(&mut self, id: Uuid) -> Result<Option<Card>, AppErr> {
        Ok(card_model::Entity::find_by_id(id)
            .one(self.tx)
            .await?
            .map(to_card))
    }

    async fn save(&mut self, card: Card) -> Result<Card, AppErr> {
        if card.id.is_nil() {
            let saved = card_model::ActiveModel {
                id: Set(Uuid::new_v4()),
                title: Set(card.title),
                description: Set(card.description),
                deadline: Set(card.deadline),
                completed: Set(card.completed),
                project_id: Set(card.project_id),
                section_id: Set(card.section_id),
            }
            .insert(self.tx)
            .await?;

            Ok(to_card(saved))
        } else {
            let saved = card_model::ActiveModel {
                id: Set(card.id),
                title: Set(card.title),
                description: Set(card.description),
                deadline: Set(card.deadline),
                completed: Set(card.completed),
                project_id: Set(card.project_id),
                section_id: Set(card.section_id),
            }
            .update(self.tx)
            .await?;

            Ok(to_card(saved))
        }
    }

    async fn delete(&mut self, id: Uuid) -> Result<(), AppErr> {
        card_model::Entity::delete_by_id(id).exec(self.tx).await?;
        Ok(())
    }
}

fn to_card(model: card_model::Model) -> Card {
    Card {
        id: model.id,
        title: model.title,
        description: model.description,
        deadline: model.deadline,
        completed: model.completed,
        project_id: model.project_id,
        section_id: model.section_id,
    }
}
