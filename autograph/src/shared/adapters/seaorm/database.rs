use super::unit_of_work::SeaOrmUnitOfWork;
use crate::shared::error::AppErr;
use crate::shared::ports::database::{Database, DatabaseBuilder};
use crate::shared::ports::unit_of_work::UnitOfWork;
use sea_orm::{
    ConnectOptions, ConnectionTrait, Database as SeaOrmDriver, DatabaseConnection, TransactionTrait,
};

pub type SeaOrmConnection = DatabaseConnection;

pub struct SeaOrmDatabase {
    conn: SeaOrmConnection,
}

pub struct SeaOrmDatabaseBuilder {
    path: String,
    run_migrations: bool,
}

impl DatabaseBuilder for SeaOrmDatabaseBuilder {
    type Db = SeaOrmDatabase;

    fn open(path: &str) -> Self {
        Self {
            path: path.to_owned(),
            run_migrations: false,
        }
    }

    fn migrate(self) -> Self {
        Self {
            run_migrations: true,
            ..self
        }
    }

    async fn finish(self) -> Result<SeaOrmDatabase, AppErr> {
        let is_memory = self.path == ":memory:";
        let conn_str = if is_memory {
            "sqlite::memory:".to_owned()
        } else {
            format!("sqlite://{}?mode=rwc", self.path)
        };

        let mut options = ConnectOptions::new(conn_str);
        options.max_connections(if is_memory { 1 } else { 4 });
        options.sqlx_logging(false);

        let conn = SeaOrmDriver::connect(options).await?;
        conn.execute_unprepared("PRAGMA foreign_keys = ON;").await?;

        if !is_memory {
            conn.execute_unprepared("PRAGMA journal_mode = WAL;")
                .await?;
        }

        if self.run_migrations {
            run_migrations(&conn).await?;
        }

        Ok(SeaOrmDatabase { conn })
    }
}

impl Database for SeaOrmDatabase {
    type Conn = SeaOrmConnection;
    type Uow = SeaOrmUnitOfWork;

    fn conn(&self) -> SeaOrmConnection {
        self.conn.clone()
    }

    async fn begin<'a, T: Send + 'a>(
        &'a self,
        f: impl AsyncFnOnce(&mut SeaOrmUnitOfWork) -> Result<T, AppErr> + Send + 'a,
    ) -> Result<T, AppErr> {
        let tx = self.conn.begin().await?;
        let mut uow = SeaOrmUnitOfWork::new(tx);
        let val = f(&mut uow).await?;
        uow.commit().await?;
        Ok(val)
    }
}

async fn run_migrations(conn: &DatabaseConnection) -> Result<(), AppErr> {
    for migration in MIGRATIONS {
        conn.execute_unprepared(migration).await?;
    }

    Ok(())
}

const MIGRATIONS: &[&str] = &[
    include_str!("../sqlx/migrations/0001_init.sql"),
    include_str!("../sqlx/migrations/0002_card_details.sql"),
    include_str!("../sqlx/migrations/0003_events.sql"),
    include_str!("../sqlx/migrations/0004_sections.sql"),
    include_str!("../sqlx/migrations/0005_remove_events.sql"),
];
