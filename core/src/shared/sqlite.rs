use super::ports::{Connection, Database, Transaction};
use crate::shared::error::AppErr;

/// A lightweight read-only handle wrapping a borrowed `rusqlite::Connection`.
/// Used for query-side operations; see [`SqliteTransaction`] for the write side.
#[derive(Copy, Clone)]
pub struct SqliteConnection<'a>(&'a rusqlite::Connection);

impl<'a> SqliteConnection<'a> {
    pub(crate) fn raw(&self) -> &'a rusqlite::Connection {
        self.0
    }
}

impl Connection for SqliteConnection<'_> {}

/// A lightweight write-capable handle wrapping a borrowed `rusqlite::Connection`
/// that is already inside an open transaction.
/// Used for mutation operations; see [`SqliteConnection`] for the read side.
#[derive(Copy, Clone)]
pub struct SqliteTransaction<'a>(&'a rusqlite::Connection);

impl<'a> SqliteTransaction<'a> {
    pub(crate) fn raw(&self) -> &'a rusqlite::Connection {
        self.0
    }
}

impl<'a> Transaction for SqliteTransaction<'a> {
    type Conn = SqliteConnection<'a>;
}

pub struct SqliteDatabase {
    conn: rusqlite::Connection,
}

impl SqliteDatabase {
    /// Create all required tables if they do not already exist.
    /// Call this once after [`SqliteDatabase::open`] before using the database.
    pub fn migrate(&self) -> Result<(), AppErr> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS todos (
                id    INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                completed INTEGER NOT NULL DEFAULT 0
            );",
        )?;
        Ok(())
    }
}

impl Database for SqliteDatabase {
    type Conn<'a> = SqliteConnection<'a>;
    type Tx<'a> = SqliteTransaction<'a>;

    fn open(path: &str) -> Result<Self, AppErr> {
        let conn = rusqlite::Connection::open(path)?;
        Ok(Self { conn })
    }

    fn conn(&self) -> SqliteConnection<'_> {
        SqliteConnection(&self.conn)
    }

    fn transaction<T>(
        &self,
        f: impl FnOnce(SqliteTransaction<'_>) -> Result<T, AppErr>,
    ) -> Result<T, AppErr> {
        self.conn.execute_batch("BEGIN")?;
        let tx = SqliteTransaction(&self.conn);
        match f(tx) {
            Ok(val) => match self.conn.execute_batch("COMMIT") {
                Ok(()) => Ok(val),
                Err(e) => {
                    let _ = self.conn.execute_batch("ROLLBACK");
                    Err(e.into())
                }
            },
            Err(e) => {
                let _ = self.conn.execute_batch("ROLLBACK");
                Err(e)
            }
        }
    }
}
