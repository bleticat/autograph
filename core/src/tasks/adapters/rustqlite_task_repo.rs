use crate::shared::adapters::rustqlite_database::RustqliteTransaction;
use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use crate::tasks::ports::task_repo::TodoRepository;
use crate::tasks::Todo;
use uuid::Uuid;

pub struct SqliteTodoRepository<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> From<RustqliteTransaction<'a>> for SqliteTodoRepository<'a> {
    fn from(tx: RustqliteTransaction<'a>) -> Self {
        Self { conn: tx.raw() }
    }
}

impl<'a> Repository<Todo> for SqliteTodoRepository<'a> {
    type Tx = RustqliteTransaction<'a>;

    fn get(&self, id: Uuid) -> Result<Option<Todo>, AppErr> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, title, completed, project_id FROM todos WHERE id = ?1")?;
        let mut rows = stmt.query([id])?;
        match rows.next()? {
            Some(row) => Ok(Some(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                completed: row.get::<_, i32>(2)? != 0,
                project_id: row.get(3)?,
            })),
            None => Ok(None),
        }
    }

    fn save(&self, todo: &Todo) -> Result<Uuid, AppErr> {
        if todo.id.is_nil() {
            let id = Uuid::new_v4();
            self.conn.execute(
                "INSERT INTO todos (id, title, completed, project_id) VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params![id, todo.title, todo.completed as i32, todo.project_id],
            )?;
            Ok(id)
        } else {
            self.conn.execute(
                "UPDATE todos SET title = ?1, completed = ?2, project_id = ?3 WHERE id = ?4",
                rusqlite::params![todo.title, todo.completed as i32, todo.project_id, todo.id],
            )?;
            Ok(todo.id)
        }
    }

    fn delete(&self, id: Uuid) -> Result<(), AppErr> {
        self.conn.execute("DELETE FROM todos WHERE id = ?1", [id])?;
        Ok(())
    }
}

impl<'a> TodoRepository for SqliteTodoRepository<'a> {}
