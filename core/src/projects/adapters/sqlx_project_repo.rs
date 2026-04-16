use crate::projects::Project;
use crate::shared::adapters::database::sqlx_database::SqlxTx;
use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use sqlx::Row;
use uuid::Uuid;

pub struct SqliteProjectRepository<'a> {
    tx: &'a SqlxTx,
}

impl<'a> Repository<'a, Project> for SqliteProjectRepository<'a> {
    type Tx = SqlxTx;

    fn bind(tx: &'a Self::Tx) -> Self {
        Self { tx }
    }

    async fn get(&self, id: Uuid) -> Result<Option<Project>, AppErr> {
        let mut tx = self.tx.lock().await;
        let row = sqlx::query("SELECT id, title FROM projects WHERE id = ?1")
            .bind(id)
            .fetch_optional(&mut **tx)
            .await?;
        Ok(row.map(|row| Project {
            id: row.get(0),
            title: row.get(1),
        }))
    }

    async fn save(&self, project: &Project) -> Result<Uuid, AppErr> {
        let mut tx = self.tx.lock().await;
        if project.id.is_nil() {
            let id = Uuid::new_v4();
            sqlx::query("INSERT INTO projects (id, title) VALUES (?1, ?2)")
                .bind(id)
                .bind(project.title.as_str())
                .execute(&mut **tx)
                .await?;
            Ok(id)
        } else {
            sqlx::query("UPDATE projects SET title = ?1 WHERE id = ?2")
                .bind(project.title.as_str())
                .bind(project.id)
                .execute(&mut **tx)
                .await?;
            Ok(project.id)
        }
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppErr> {
        let mut tx = self.tx.lock().await;
        sqlx::query("DELETE FROM projects WHERE id = ?1")
            .bind(id)
            .execute(&mut **tx)
            .await?;
        Ok(())
    }
}
