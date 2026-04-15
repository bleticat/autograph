use crate::shared::error::AppErr;
use crate::shared::ports::database::{Connection, Database, Transaction};
use futures::executor::block_on;
use sqlx::Connection as _;
use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

#[derive(Clone)]
pub struct RustqliteConnection(Rc<RefCell<sqlx::SqliteConnection>>);

impl RustqliteConnection {
    pub(crate) fn raw(&self) -> Rc<RefCell<sqlx::SqliteConnection>> {
        Rc::clone(&self.0)
    }
}

impl Connection for RustqliteConnection {}

#[derive(Clone)]
pub struct RustqliteTransaction(Rc<RefCell<sqlx::SqliteConnection>>);

impl RustqliteTransaction {
    pub(crate) fn raw(&self) -> Rc<RefCell<sqlx::SqliteConnection>> {
        Rc::clone(&self.0)
    }
}

impl Transaction for RustqliteTransaction {
    type Conn = RustqliteConnection;
}

pub struct RustqliteDatabase {
    conn: Rc<RefCell<sqlx::SqliteConnection>>,
}

impl RustqliteDatabase {
    pub fn migrate(&self) -> Result<(), AppErr> {
        let mut conn = self.conn.borrow_mut();
        block_on(sqlx::migrate!("./migrations").run(&mut *conn))?;
        Ok(())
    }
}

impl Database for RustqliteDatabase {
    type Conn<'a> = RustqliteConnection;
    type Tx<'a> = RustqliteTransaction;

    fn open(path: &str) -> Result<Self, AppErr> {
        let conn_str = if path == ":memory:" {
            "sqlite::memory:".to_owned()
        } else {
            format!("sqlite://{path}")
        };

        let conn_options = sqlx::sqlite::SqliteConnectOptions::from_str(&conn_str)?
            .foreign_keys(true)
            .create_if_missing(path != ":memory:");
        let conn = block_on(sqlx::SqliteConnection::connect_with(&conn_options))?;

        Ok(Self {
            conn: Rc::new(RefCell::new(conn)),
        })
    }

    fn conn(&self) -> RustqliteConnection {
        RustqliteConnection(Rc::clone(&self.conn))
    }

    fn transaction<T>(
        &self,
        f: impl FnOnce(RustqliteTransaction) -> Result<T, AppErr>,
    ) -> Result<T, AppErr> {
        let mut conn = self.conn.borrow_mut();
        block_on(sqlx::query("BEGIN").execute(&mut *conn))?;
        drop(conn);

        let tx = RustqliteTransaction(Rc::clone(&self.conn));
        match f(tx) {
            Ok(val) => {
                let mut conn = self.conn.borrow_mut();
                match block_on(sqlx::query("COMMIT").execute(&mut *conn)) {
                    Ok(_) => Ok(val),
                    Err(e) => {
                        let _ = block_on(sqlx::query("ROLLBACK").execute(&mut *conn));
                        Err(e.into())
                    }
                }
            }
            Err(e) => {
                let mut conn = self.conn.borrow_mut();
                let _ = block_on(sqlx::query("ROLLBACK").execute(&mut *conn));
                Err(e)
            }
        }
    }
}
