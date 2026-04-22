use crate::section::entity::Section;
use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use sqlx::Row;
use uuid::Uuid;

pub struct SqlxSectionRepository<'a> {
    tx: &'a mut sqlx::Transaction<'static, sqlx::Sqlite>,
}

impl<'a> SqlxSectionRepository<'a> {
    pub fn new(tx: &'a mut sqlx::Transaction<'static, sqlx::Sqlite>) -> Self {
        Self { tx }
    }
}

impl<'a> Repository<Section> for SqlxSectionRepository<'a> {
    async fn get(&mut self, id: Uuid) -> Result<Option<Section>, AppErr> {
        let row = sqlx::query("SELECT id, title, project_id FROM sections WHERE id = ?1")
            .bind(id)
            .fetch_optional(&mut **self.tx)
            .await?;

        Ok(row.map(|row| Section {
            id: row.get(0),
            title: row.get(1),
            project_id: row.get(2),
        }))
    }

    async fn save(&mut self, section: Section) -> Result<Section, AppErr> {
        if section.id.is_nil() {
            let id = Uuid::new_v4();
            sqlx::query("INSERT INTO sections (id, title, project_id) VALUES (?1, ?2, ?3)")
                .bind(id)
                .bind(section.title.as_str())
                .bind(section.project_id)
                .execute(&mut **self.tx)
                .await?;

            Ok(Section { id, ..section })
        } else {
            let updated =
                sqlx::query("UPDATE sections SET title = ?1, project_id = ?2 WHERE id = ?3")
                    .bind(section.title.as_str())
                    .bind(section.project_id)
                    .bind(section.id)
                    .execute(&mut **self.tx)
                    .await?
                    .rows_affected();

            if updated == 0 {
                return Err(sqlx::Error::RowNotFound.into());
            }

            Ok(section)
        }
    }

    async fn delete(&mut self, id: Uuid) -> Result<(), AppErr> {
        sqlx::query("DELETE FROM sections WHERE id = ?1")
            .bind(id)
            .execute(&mut **self.tx)
            .await?;
        Ok(())
    }
}
