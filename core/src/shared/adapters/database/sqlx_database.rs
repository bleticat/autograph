use crate::shared::error::AppErr;
use crate::shared::ports::database::{Connection, Database, Transaction};
use futures::executor::block_on;
use sqlx::Connection as _;
use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

#[derive(Clone)]
pub struct SqlxConnection(Rc<RefCell<sqlx::SqliteConnection>>);

impl SqlxConnection {
    pub(crate) fn raw(&self) -> Rc<RefCell<sqlx::SqliteConnection>> {
        Rc::clone(&self.0)
    }
}

impl Connection for SqlxConnection {}

#[derive(Clone)]
pub struct SqlxTransaction(Rc<RefCell<sqlx::SqliteConnection>>);

impl SqlxTransaction {
    pub(crate) fn raw(&self) -> Rc<RefCell<sqlx::SqliteConnection>> {
        Rc::clone(&self.0)
    }
}

impl Transaction for SqlxTransaction {
    type Conn = SqlxConnection;
}

pub struct SqlxDatabase {
    conn: Rc<RefCell<sqlx::SqliteConnection>>,
}

impl SqlxDatabase {
    pub fn migrate(&self) -> Result<(), AppErr> {
        let mut conn = self.conn.borrow_mut();
        block_on(sqlx::migrate!("./src/shared/adapters/database/migrations").run(&mut *conn))?;
        Ok(())
    }
}

impl Database for SqlxDatabase {
    type Conn<'a> = SqlxConnection;
    type Tx<'a> = SqlxTransaction;

    fn open(path: &str) -> Result<Self, AppErr> {
        let is_memory = path == ":memory:";
        let conn_str = if is_memory {
            "sqlite::memory:".to_owned()
        } else {
            format!("sqlite://{path}")
        };

        let conn_options = sqlx::sqlite::SqliteConnectOptions::from_str(&conn_str)?
            .foreign_keys(true)
            .create_if_missing(!is_memory);
        let conn = block_on(sqlx::SqliteConnection::connect_with(&conn_options))?;

        Ok(Self {
            conn: Rc::new(RefCell::new(conn)),
        })
    }

    fn conn(&self) -> SqlxConnection {
        SqlxConnection(Rc::clone(&self.conn))
    }

    fn transaction<T>(
        &self,
        f: impl FnOnce(SqlxTransaction) -> Result<T, AppErr>,
    ) -> Result<T, AppErr> {
        {
            let mut conn = self.conn.borrow_mut();
            block_on(sqlx::query("BEGIN").execute(&mut *conn))?;
        }

        let tx = SqlxTransaction(Rc::clone(&self.conn));
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
