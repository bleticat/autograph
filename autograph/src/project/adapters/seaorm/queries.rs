use crate::card::adapters::seaorm::history::{load_history_map, to_card};
use crate::card::entity::Card;
use crate::project::entity::{Project, ProjectData, SectionWithCards};
use crate::project::queries::ProjectQueries;
use crate::section::entity::Section;
use crate::shared::adapters::seaorm::database::SeaOrmConnection;
use crate::shared::adapters::seaorm::models::{
    card as card_model, project as project_model, section as section_model,
};
use crate::shared::error::AppErr;
use sea_orm::{
    ColumnTrait, EntityTrait, QueryFilter as SeaOrmQueryFilter, QueryOrder, QuerySelect,
    sea_query::Expr,
};
use std::collections::HashMap;
use uuid::Uuid;

pub struct SeaOrmProjectQueries {
    conn: SeaOrmConnection,
}

impl SeaOrmProjectQueries {
    pub fn new(conn: SeaOrmConnection) -> Self {
        Self { conn }
    }
}

impl ProjectQueries for SeaOrmProjectQueries {
    async fn filter(&self, limit: u32, offset: u32) -> Result<Vec<Project>, AppErr> {
        let projects = project_model::Entity::find()
            .order_by_asc(Expr::cust("rowid"))
            .limit(u64::from(limit))
            .offset(u64::from(offset))
            .all(&self.conn)
            .await?;

        Ok(projects.into_iter().map(to_project).collect())
    }

    async fn get_project(&self, project_id: Uuid) -> Result<Option<ProjectData>, AppErr> {
        let project = project_model::Entity::find_by_id(project_id)
            .one(&self.conn)
            .await?
            .map(to_project);

        let Some(project) = project else {
            return Ok(None);
        };

        let sections = section_model::Entity::find()
            .filter(section_model::Column::ProjectId.eq(project_id))
            .order_by_asc(Expr::cust("rowid"))
            .all(&self.conn)
            .await?
            .into_iter()
            .map(to_section)
            .collect::<Vec<_>>();

        let cards = card_model::Entity::find()
            .filter(card_model::Column::ProjectId.eq(project_id))
            .filter(card_model::Column::Deleted.eq(false))
            .order_by_asc(Expr::cust("rowid"))
            .all(&self.conn)
            .await?;
        let card_ids = cards.iter().map(|card| card.id).collect::<Vec<_>>();
        let history_map = load_history_map(&self.conn, &card_ids).await?;
        let cards = cards
            .into_iter()
            .map(|card| {
                let history = history_map.get(&card.id).cloned().unwrap_or_default();
                to_card(card, history)
            })
            .collect::<Vec<_>>();

        let mut cards_without_section = Vec::new();
        let mut section_cards = sections
            .iter()
            .map(|section| (section.id, Vec::new()))
            .collect::<HashMap<_, Vec<Card>>>();

        for card in cards {
            match card.section_id {
                Some(section_id) => section_cards.entry(section_id).or_default().push(card),
                None => cards_without_section.push(card),
            }
        }

        let sections = sections
            .into_iter()
            .map(|section| SectionWithCards {
                cards: section_cards.remove(&section.id).unwrap_or_default(),
                section,
            })
            .collect();

        Ok(Some(ProjectData {
            project,
            sections,
            cards_without_section,
        }))
    }
}

fn to_project(model: project_model::Model) -> Project {
    Project {
        id: model.id,
        title: model.title,
    }
}

fn to_section(model: section_model::Model) -> Section {
    Section {
        id: model.id,
        title: model.title,
        project_id: model.project_id,
    }
}
