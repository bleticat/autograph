use crate::card::entity::Card;
use crate::card::queries::CardQueries;
use crate::shared::adapters::seaorm::database::SeaOrmConnection;
use crate::shared::adapters::seaorm::models::card as card_model;
use crate::shared::error::AppErr;
use crate::shared::filter::QueryFilter;
use chrono::{DateTime, Utc};
use sea_orm::{
    ColumnTrait, EntityTrait, QueryFilter as SeaOrmQueryFilter, QueryOrder, QuerySelect,
    sea_query::Expr,
};
use uuid::Uuid;

pub struct SeaOrmCardQueries {
    conn: SeaOrmConnection,
}

impl SeaOrmCardQueries {
    pub fn new(conn: SeaOrmConnection) -> Self {
        Self { conn }
    }
}

impl CardQueries for SeaOrmCardQueries {
    async fn filter(
        &self,
        limit: u32,
        offset: u32,
        deadline: QueryFilter<DateTime<Utc>>,
        project_id: QueryFilter<Uuid>,
        section_id: QueryFilter<Uuid>,
    ) -> Result<Vec<Card>, AppErr> {
        let mut query = card_model::Entity::find()
            .order_by_asc(Expr::cust("rowid"))
            .limit(u64::from(limit))
            .offset(u64::from(offset));

        query = match project_id {
            QueryFilter::Val(project_id) => {
                query.filter(card_model::Column::ProjectId.eq(project_id))
            }
            QueryFilter::None => query.filter(card_model::Column::ProjectId.is_null()),
            QueryFilter::Ignore => query,
        };

        query = match section_id {
            QueryFilter::Val(section_id) => {
                query.filter(card_model::Column::SectionId.eq(section_id))
            }
            QueryFilter::None => query.filter(card_model::Column::SectionId.is_null()),
            QueryFilter::Ignore => query,
        };

        query = match deadline {
            QueryFilter::Val(deadline) => query.filter(card_model::Column::Deadline.eq(deadline)),
            QueryFilter::None => query.filter(card_model::Column::Deadline.is_null()),
            QueryFilter::Ignore => query,
        };

        let cards = query.all(&self.conn).await?;
        Ok(cards.into_iter().map(to_card).collect())
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
