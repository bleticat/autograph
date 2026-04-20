use std::fmt;

#[derive(Debug)]
pub enum AppErr {
    Db(sqlx::Error),
    Migration(sqlx::migrate::MigrateError),
}

impl fmt::Display for AppErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppErr::Db(e) => write!(f, "database error: {e}"),
            AppErr::Migration(e) => write!(f, "migration error: {e}"),
        }
    }
}

impl std::error::Error for AppErr {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppErr::Db(e) => Some(e),
            AppErr::Migration(e) => Some(e),
        }
    }
}

impl From<sqlx::Error> for AppErr {
    fn from(e: sqlx::Error) -> Self {
        AppErr::Db(e)
    }
}

impl From<sqlx::migrate::MigrateError> for AppErr {
    fn from(e: sqlx::migrate::MigrateError) -> Self {
        AppErr::Migration(e)
    }
}
