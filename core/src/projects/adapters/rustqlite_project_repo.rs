use crate::projects::ports::project_repo::ProjectRepository;
use crate::projects::Project;
use crate::shared::adapters::rustqlite_database::RustqliteTransaction;
use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use futures::executor::block_on;
use sqlx::Row;
use uuid::Uuid;

pub struct SqliteProjectRepository {
    conn: RustqliteTransaction,
}

impl From<RustqliteTransaction> for SqliteProjectRepository {
    fn from(tx: RustqliteTransaction) -> Self {
        Self { conn: tx }
    }
}

impl Repository<Project> for SqliteProjectRepository {
    type Tx = RustqliteTransaction;

    fn get(&self, id: Uuid) -> Result<Option<Project>, AppErr> {
        let conn = self.conn.raw();
        let mut conn = conn.borrow_mut();
        let row = block_on(
            sqlx::query("SELECT id, title FROM projects WHERE id = ?1")
                .bind(id)
                .fetch_optional(&mut *conn),
        )?;
        Ok(row.map(|row| Project {
            id: row.get(0),
            title: row.get(1),
        }))
    }

    fn save(&self, project: &Project) -> Result<Uuid, AppErr> {
        let conn = self.conn.raw();
        let mut conn = conn.borrow_mut();
        if project.id.is_nil() {
            let id = Uuid::new_v4();
            block_on(
                sqlx::query("INSERT INTO projects (id, title) VALUES (?1, ?2)")
                    .bind(id)
                    .bind(project.title.as_str())
                    .execute(&mut *conn),
            )?;
            Ok(id)
        } else {
            block_on(
                sqlx::query("UPDATE projects SET title = ?1 WHERE id = ?2")
                    .bind(project.title.as_str())
                    .bind(project.id)
                    .execute(&mut *conn),
            )?;
            Ok(project.id)
        }
    }

    fn delete(&self, id: Uuid) -> Result<(), AppErr> {
        let conn = self.conn.raw();
        let mut conn = conn.borrow_mut();
        block_on(
            sqlx::query("DELETE FROM projects WHERE id = ?1")
                .bind(id)
                .execute(&mut *conn),
        )?;
        Ok(())
    }
}

impl ProjectRepository for SqliteProjectRepository {}
