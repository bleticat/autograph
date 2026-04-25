use std::fmt;

#[derive(Debug)]
pub enum AppErr {
    SeaOrm(sea_orm::DbErr),
    Parse(String),
    Validation(String),
}

impl fmt::Display for AppErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppErr::SeaOrm(e) => write!(f, "database error: {e}"),
            AppErr::Parse(e) => write!(f, "parse error: {e}"),
            AppErr::Validation(e) => write!(f, "validation error: {e}"),
        }
    }
}

impl std::error::Error for AppErr {
    // 'static is required by the std::error::Error trait signature; it means
    // the returned source error must own all its data (or be a 'static reference).
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppErr::SeaOrm(e) => Some(e),
            AppErr::Parse(_) => None,
            AppErr::Validation(_) => None,
        }
    }
}

impl From<sea_orm::DbErr> for AppErr {
    fn from(e: sea_orm::DbErr) -> Self {
        AppErr::SeaOrm(e)
    }
}
