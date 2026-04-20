use crate::events::Event;
use crate::shared::adapters::database::sqlx_unit_of_work::SqlxUnitOfWork;
use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use sqlx::Row;
use uuid::Uuid;

impl Repository<Event> for SqlxUnitOfWork {
    async fn get(&mut self, id: Uuid) -> Result<Option<Event>, AppErr> {
        let tx = self.tx();
        let row = sqlx::query(
            "SELECT id, date, title, description, project_id FROM events WHERE id = ?1",
        )
        .bind(id)
        .fetch_optional(&mut **tx)
        .await?;
        Ok(row.map(|row| Event {
            id: row.get(0),
            date: row.get(1),
            title: row.get(2),
            description: row.get(3),
            project_id: row.get(4),
        }))
    }

    async fn save(&mut self, event: Event) -> Result<Event, AppErr> {
        if event.id.is_nil() {
            let id = Uuid::new_v4();
            let tx = self.tx();
            sqlx::query(
                "INSERT INTO events (id, date, title, description, project_id) VALUES (?1, ?2, ?3, ?4, ?5)",
            )
            .bind(id)
            .bind(event.date)
            .bind(&event.title)
            .bind(&event.description)
            .bind(event.project_id)
            .execute(&mut **tx)
            .await?;
            Ok(Event { id, ..event })
        } else {
            let tx = self.tx();
            let updated = sqlx::query(
                "UPDATE events SET date = ?1, title = ?2, description = ?3, project_id = ?4 WHERE id = ?5",
            )
            .bind(event.date)
            .bind(&event.title)
            .bind(&event.description)
            .bind(event.project_id)
            .bind(event.id)
            .execute(&mut **tx)
            .await?
            .rows_affected();
            if updated == 0 {
                return Err(sqlx::Error::RowNotFound.into());
            }
            Ok(event)
        }
    }

    async fn delete(&mut self, id: Uuid) -> Result<(), AppErr> {
        let tx = self.tx();
        sqlx::query("DELETE FROM events WHERE id = ?1")
            .bind(id)
            .execute(&mut **tx)
            .await?;
        Ok(())
    }
}
