use crate::projects::Project;
use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use crate::shared::ports::unit_of_work::UnitOfWork;
use sqlx::Row;
use uuid::Uuid;

impl Repository for Project {
    type Tx = sqlx::Transaction<'static, sqlx::Sqlite>;

    async fn get<U>(uow: &mut U, id: Uuid) -> Result<Option<Project>, AppErr>
    where
        U: UnitOfWork<Tx = Self::Tx>,
    {
        let row = sqlx::query("SELECT id, title FROM projects WHERE id = ?1")
            .bind(id)
            .fetch_optional(&mut **uow.tx())
            .await?;
        Ok(row.map(|row| Project {
            id: row.get(0),
            title: row.get(1),
        }))
    }

    async fn save<U>(self, uow: &mut U) -> Result<Project, AppErr>
    where
        U: UnitOfWork<Tx = Self::Tx>,
    {
        let project = self;
        if project.id.is_nil() {
            let id = Uuid::new_v4();
            sqlx::query("INSERT INTO projects (id, title) VALUES (?1, ?2)")
                .bind(id)
                .bind(project.title.as_str())
                .execute(&mut **uow.tx())
                .await?;
            Ok(Project { id, ..project })
        } else {
            sqlx::query("UPDATE projects SET title = ?1 WHERE id = ?2")
                .bind(project.title.as_str())
                .bind(project.id)
                .execute(&mut **uow.tx())
                .await?;
            Ok(project)
        }
    }

    async fn delete<U>(uow: &mut U, id: Uuid) -> Result<(), AppErr>
    where
        U: UnitOfWork<Tx = Self::Tx>,
    {
        sqlx::query("DELETE FROM projects WHERE id = ?1")
            .bind(id)
            .execute(&mut **uow.tx())
            .await?;
        Ok(())
    }
}
