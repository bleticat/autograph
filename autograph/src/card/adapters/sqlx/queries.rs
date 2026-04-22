use crate::card::entity::Card;
use crate::card::queries::CardQueries;
use crate::shared::adapters::sqlx::database::SqlxConnection;
use crate::shared::error::AppErr;
use crate::shared::filter::QueryFilter;
use chrono::{DateTime, Utc};
use sqlx::{QueryBuilder, Row, Sqlite};
use uuid::Uuid;

pub struct SqlxCardQueries {
    conn: SqlxConnection,
}

impl SqlxCardQueries {
    pub fn new(conn: SqlxConnection) -> Self {
        Self { conn }
    }
}

impl CardQueries for SqlxCardQueries {
    async fn filter(
        &self,
        limit: u32,
        offset: u32,
        deadline: QueryFilter<DateTime<Utc>>,
        project_id: QueryFilter<Uuid>,
        section_id: QueryFilter<Uuid>,
    ) -> Result<Vec<Card>, AppErr> {
        let mut query = QueryBuilder::<Sqlite>::new(
            "SELECT id, title, description, deadline, completed, project_id, section_id FROM cards",
        );
        let mut has_where = false;

        match project_id {
            QueryFilter::Val(project_id) => {
                push_clause(&mut query, &mut has_where, "project_id = ");
                query.push_bind(project_id);
            }
            QueryFilter::None => push_clause(&mut query, &mut has_where, "project_id IS NULL"),
            QueryFilter::Ignore => {}
        }

        match section_id {
            QueryFilter::Val(section_id) => {
                push_clause(&mut query, &mut has_where, "section_id = ");
                query.push_bind(section_id);
            }
            QueryFilter::None => push_clause(&mut query, &mut has_where, "section_id IS NULL"),
            QueryFilter::Ignore => {}
        }

        match deadline {
            QueryFilter::Val(deadline) => {
                push_clause(&mut query, &mut has_where, "deadline = ");
                query.push_bind(deadline);
            }
            QueryFilter::None => push_clause(&mut query, &mut has_where, "deadline IS NULL"),
            QueryFilter::Ignore => {}
        }

        query
            .push(" ORDER BY rowid LIMIT ")
            .push_bind(i64::from(limit))
            .push(" OFFSET ")
            .push_bind(i64::from(offset));

        let rows = query.build().fetch_all(&self.conn).await?;
        Ok(rows.into_iter().map(map_card).collect())
    }
}

fn push_clause(query: &mut QueryBuilder<'_, Sqlite>, has_where: &mut bool, clause: &str) {
    if *has_where {
        query.push(" AND ");
    } else {
        query.push(" WHERE ");
        *has_where = true;
    }
    query.push(clause);
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
