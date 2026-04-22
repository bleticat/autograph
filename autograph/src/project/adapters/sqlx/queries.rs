use crate::card::entity::Card;
use crate::project::entity::{Project, ProjectData, SectionWithCards};
use crate::project::queries::ProjectQueries;
use crate::section::entity::Section;
use crate::shared::adapters::sqlx::database::SqlxConnection;
use crate::shared::error::AppErr;
use sqlx::Row;
use uuid::Uuid;

pub struct SqlxProjectQueries {
    conn: SqlxConnection,
}

impl SqlxProjectQueries {
    pub fn new(conn: SqlxConnection) -> Self {
        Self { conn }
    }
}

impl ProjectQueries for SqlxProjectQueries {
    async fn filter(&self, limit: u32, offset: u32) -> Result<Vec<Project>, AppErr> {
        let rows = sqlx::query("SELECT id, title FROM projects ORDER BY rowid LIMIT ?1 OFFSET ?2")
            .bind(i64::from(limit))
            .bind(i64::from(offset))
            .fetch_all(&self.conn)
            .await?;
        let projects = rows.into_iter().map(map_project).collect();
        Ok(projects)
    }

    async fn get_project(&self, project_id: Uuid) -> Result<Option<ProjectData>, AppErr> {
        let project = sqlx::query("SELECT id, title FROM projects WHERE id = ?1")
            .bind(project_id)
            .fetch_optional(&self.conn)
            .await?
            .map(map_project);

        let Some(project) = project else {
            return Ok(None);
        };

        let section_rows = sqlx::query(
            "SELECT id, title, project_id FROM sections WHERE project_id = ?1 ORDER BY rowid",
        )
        .bind(project_id)
        .fetch_all(&self.conn)
        .await?;
        let sections = section_rows
            .into_iter()
            .map(map_section)
            .collect::<Vec<_>>();

        let card_rows = sqlx::query(
            "SELECT id, title, description, deadline, completed, project_id, section_id FROM cards WHERE project_id = ?1 ORDER BY rowid",
        )
        .bind(project_id)
        .fetch_all(&self.conn)
        .await?;
        let mut cards_without_section = Vec::new();
        let mut section_cards = sections
            .iter()
            .map(|section| (section.id, Vec::new()))
            .collect::<std::collections::HashMap<_, Vec<Card>>>();

        for card in card_rows.into_iter().map(map_card) {
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

fn map_project(row: sqlx::sqlite::SqliteRow) -> Project {
    Project {
        id: row.get(0),
        title: row.get(1),
    }
}

fn map_section(row: sqlx::sqlite::SqliteRow) -> Section {
    Section {
        id: row.get(0),
        title: row.get(1),
        project_id: row.get(2),
    }
}

fn map_card(row: sqlx::sqlite::SqliteRow) -> Card {
    Card {
        id: row.get(0),
        title: row.get(1),
        description: row.get(2),
        deadline: row.get(3),
        completed: row.get::<bool, _>(4),
        project_id: row.get(5),
        section_id: row.get(6),
    }
}
