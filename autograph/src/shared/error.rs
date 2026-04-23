use std::fmt;

#[derive(Debug)]
pub enum AppErr {
    SeaOrm(sea_orm::DbErr),
    Sqlx(sqlx::Error),
    Migration(String),
    Parse(String),
    Validation(String),
}

impl fmt::Display for AppErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppErr::SeaOrm(e) => write!(f, "database error: {e}"),
            AppErr::Sqlx(e) => write!(f, "database error: {e}"),
            AppErr::Migration(e) => write!(f, "migration error: {e}"),
            AppErr::Parse(e) => write!(f, "parse error: {e}"),
            AppErr::Validation(e) => write!(f, "validation error: {e}"),
        }
    }
}

impl std::error::Error for AppErr {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppErr::SeaOrm(e) => Some(e),
            AppErr::Sqlx(e) => Some(e),
            AppErr::Parse(_) => None,
            AppErr::Migration(_) => None,
            AppErr::Validation(_) => None,
        }
    }
}

impl From<sea_orm::DbErr> for AppErr {
    fn from(e: sea_orm::DbErr) -> Self {
        AppErr::SeaOrm(e)
    }
}

impl From<sqlx::Error> for AppErr {
    fn from(e: sqlx::Error) -> Self {
        AppErr::Sqlx(e)
    }
}

impl From<sqlx::migrate::MigrateError> for AppErr {
    fn from(e: sqlx::migrate::MigrateError) -> Self {
        AppErr::Migration(e.to_string())
    }
}
