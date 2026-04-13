use std::fmt;

/// Crate-level error type for all fallible operations in `autograph-core`.
#[derive(Debug)]
pub enum AppErr {
    Db(rusqlite::Error),
}

impl fmt::Display for AppErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppErr::Db(e) => write!(f, "database error: {e}"),
        }
    }
}

impl std::error::Error for AppErr {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppErr::Db(e) => Some(e),
        }
    }
}

impl From<rusqlite::Error> for AppErr {
    fn from(e: rusqlite::Error) -> Self {
        AppErr::Db(e)
    }
}
