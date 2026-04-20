use crate::projects::Project;
use crate::shared::adapters::database::sqlx_unit_of_work::SqlxUnitOfWork;
use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use sqlx::Row;
use uuid::Uuid;

impl Repository<Project> for SqlxUnitOfWork {
    async fn get(&mut self, id: Uuid) -> Result<Option<Project>, AppErr> {
        let tx = self.tx();
        let row = sqlx::query("SELECT id, title FROM projects WHERE id = ?1")
            .bind(id)
            .fetch_optional(&mut **tx)
            .await?;
        Ok(row.map(|row| Project {
            id: row.get(0),
            title: row.get(1),
        }))
    }

    async fn save(&mut self, project: Project) -> Result<Project, AppErr> {
        if project.id.is_nil() {
            let id = Uuid::new_v4();
            let tx = self.tx();
            sqlx::query("INSERT INTO projects (id, title) VALUES (?1, ?2)")
                .bind(id)
                .bind(project.title.as_str())
                .execute(&mut **tx)
                .await?;
            Ok(Project { id, ..project })
        } else {
            let tx = self.tx();
            sqlx::query("UPDATE projects SET title = ?1 WHERE id = ?2")
                .bind(project.title.as_str())
                .bind(project.id)
                .execute(&mut **tx)
                .await?;
            Ok(project)
        }
    }

    async fn delete(&mut self, id: Uuid) -> Result<(), AppErr> {
        let tx = self.tx();
        sqlx::query("DELETE FROM projects WHERE id = ?1")
            .bind(id)
            .execute(&mut **tx)
            .await?;
        Ok(())
    }
}
