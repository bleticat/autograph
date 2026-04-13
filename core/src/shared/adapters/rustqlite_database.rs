use crate::shared::error::AppErr;
use crate::shared::ports::database::{Connection, Database, Transaction};

#[derive(Copy, Clone)]
pub struct RustqliteConnection<'a>(&'a rusqlite::Connection);

impl<'a> RustqliteConnection<'a> {
    pub(crate) fn raw(&self) -> &'a rusqlite::Connection {
        self.0
    }
}

impl Connection for RustqliteConnection<'_> {}

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
