use crate::card::entity::Card;
use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use sqlx::Row;
use uuid::Uuid;

pub struct SqlxCardRepository<'a> {
    tx: &'a mut sqlx::Transaction<'static, sqlx::Sqlite>,
}

impl<'a> SqlxCardRepository<'a> {
    pub fn new(tx: &'a mut sqlx::Transaction<'static, sqlx::Sqlite>) -> Self {
        Self { tx }
    }
}

impl<'a> Repository<Card> for SqlxCardRepository<'a> {
    async fn get(&mut self, id: Uuid) -> Result<Option<Card>, AppErr> {
        let row = sqlx::query(
            "SELECT id, title, description, deadline, completed, project_id, section_id FROM cards WHERE id = ?1",
        )
            .bind(id)
            .fetch_optional(&mut **self.tx)
            .await?;
        Ok(row.map(|row| Card {
            id: row.get(0),
            title: row.get(1),
            description: row.get(2),
            deadline: row.get(3),
            completed: row.get::<bool, _>(4),
            project_id: row.get(5),
            section_id: row.get(6),
        }))
    }

    async fn save(&mut self, card: Card) -> Result<Card, AppErr> {
        if card.id.is_nil() {
            let id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO cards (id, title, description, deadline, completed, project_id, section_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            )
            .bind(id)
            .bind(&card.title)
            .bind(&card.description)
            .bind(&card.deadline)
            .bind(card.completed)
            .bind(card.project_id)
            .bind(card.section_id)
            .execute(&mut **self.tx)
            .await?;
            Ok(Card { id, ..card })
        } else {
            let updated = sqlx::query(
                "UPDATE cards SET title = ?1, description = ?2, deadline = ?3, completed = ?4, project_id = ?5, section_id = ?6 WHERE id = ?7",
            )
            .bind(&card.title)
            .bind(&card.description)
            .bind(&card.deadline)
            .bind(card.completed)
            .bind(card.project_id)
            .bind(card.section_id)
            .bind(card.id)
            .execute(&mut **self.tx)
            .await?
            .rows_affected();
            if updated == 0 {
                return Err(sqlx::Error::RowNotFound.into());
            }
            Ok(card)
        }
    }

    async fn delete(&mut self, id: Uuid) -> Result<(), AppErr> {
        sqlx::query("DELETE FROM cards WHERE id = ?1")
            .bind(id)
            .execute(&mut **self.tx)
            .await?;
        Ok(())
    }
}
