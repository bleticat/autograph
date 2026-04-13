use super::ports::{Database, Transaction};

pub struct SqliteConnection<'a>(pub(crate) &'a rusqlite::Connection);

pub struct SqliteTransaction<'a>(pub(crate) &'a rusqlite::Connection);

impl Transaction for SqliteTransaction<'_> {}

pub struct SqliteDatabase {
    conn: rusqlite::Connection,
}

impl Database for SqliteDatabase {}

impl SqliteDatabase {
    pub fn open(path: &str) -> Result<Self, String> {
        let conn = rusqlite::Connection::open(path).map_err(|e| e.to_string())?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS todos (
                id    INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                completed INTEGER NOT NULL DEFAULT 0
            );",
        )
        .map_err(|e| e.to_string())?;
        Ok(Self { conn })
    }

    pub fn conn(&self) -> SqliteConnection<'_> {
        SqliteConnection(&self.conn)
    }

    pub fn transaction<T>(
        &self,
        f: impl FnOnce(&SqliteTransaction) -> Result<T, String>,
    ) -> Result<T, String> {
        self.conn
            .execute_batch("BEGIN")
            .map_err(|e| e.to_string())?;
        let tx = SqliteTransaction(&self.conn);
        match f(&tx) {
            Ok(val) => {
                self.conn
                    .execute_batch("COMMIT")
                    .map_err(|e| e.to_string())?;
                Ok(val)
            }
            Err(e) => {
                let _ = self.conn.execute_batch("ROLLBACK");
                Err(e)
            }
        }
    }
}
