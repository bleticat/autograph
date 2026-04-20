use crate::project::entity::Project;
use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use sqlx::Row;
use uuid::Uuid;

pub struct SqlxProjectRepository<'a> {
    tx: &'a mut sqlx::Transaction<'static, sqlx::Sqlite>,
}

impl<'a> SqlxProjectRepository<'a> {
    pub fn new(tx: &'a mut sqlx::Transaction<'static, sqlx::Sqlite>) -> Self {
        Self { tx }
    }
}

impl<'a> Repository<Project> for SqlxProjectRepository<'a> {
    async fn get(&mut self, id: Uuid) -> Result<Option<Project>, AppErr> {
        let row = sqlx::query("SELECT id, title FROM projects WHERE id = ?1")
            .bind(id)
            .fetch_optional(&mut **self.tx)
            .await?;
        Ok(row.map(|row| Project {
            id: row.get(0),
            title: row.get(1),
        }))
    }

    async fn save(&mut self, project: Project) -> Result<Project, AppErr> {
        if project.id.is_nil() {
            let id = Uuid::new_v4();
            sqlx::query("INSERT INTO projects (id, title) VALUES (?1, ?2)")
                .bind(id)
                .bind(project.title.as_str())
                .execute(&mut **self.tx)
                .await?;
            Ok(Project { id, ..project })
        } else {
            sqlx::query("UPDATE projects SET title = ?1 WHERE id = ?2")
                .bind(project.title.as_str())
                .bind(project.id)
                .execute(&mut **self.tx)
                .await?;
            Ok(project)
        }
    }

    async fn delete(&mut self, id: Uuid) -> Result<(), AppErr> {
        sqlx::query("DELETE FROM projects WHERE id = ?1")
            .bind(id)
            .execute(&mut **self.tx)
            .await?;
        Ok(())
    }
}
