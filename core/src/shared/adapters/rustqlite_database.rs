use crate::shared::error::AppErr;
use crate::shared::ports::{Connection, Database, Transaction};

/// A lightweight read-only handle wrapping a borrowed `rusqlite::Connection`.
/// Used for query-side operations; see [`SqliteTransaction`] for the write side.
#[derive(Copy, Clone)]
pub struct RustqliteConnection<'a>(&'a rusqlite::Connection);

impl<'a> RustqliteConnection<'a> {
    pub(crate) fn raw(&self) -> &'a rusqlite::Connection {
        self.0
    }
}

impl Connection for RustqliteConnection<'_> {}

/// A lightweight write-capable handle wrapping a borrowed `rusqlite::Connection`
/// that is already inside an open transaction.
/// Used for mutation operations; see [`SqliteConnection`] for the read side.
#[derive(Copy, Clone)]
pub struct RustqliteTransaction<'a>(&'a rusqlite::Connection);

impl<'a> RustqliteTransaction<'a> {
    pub(crate) fn raw(&self) -> &'a rusqlite::Connection {
        self.0
    }
}

impl<'a> Transaction for RustqliteTransaction<'a> {
    type Conn = RustqliteConnection<'a>;
}

pub struct RustqliteDatabase {
    conn: rusqlite::Connection,
}

impl RustqliteDatabase {
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

impl Database for RustqliteDatabase {
    type Conn<'a> = RustqliteConnection<'a>;
    type Tx<'a> = RustqliteTransaction<'a>;

    fn open(path: &str) -> Result<Self, AppErr> {
        let conn = rusqlite::Connection::open(path)?;
        Ok(Self { conn })
    }

    fn conn(&self) -> RustqliteConnection<'_> {
        RustqliteConnection(&self.conn)
    }

    fn transaction<T>(
        &self,
        f: impl FnOnce(RustqliteTransaction<'_>) -> Result<T, AppErr>,
    ) -> Result<T, AppErr> {
        self.conn.execute_batch("BEGIN")?;
        let tx = RustqliteTransaction(&self.conn);
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
