/// Crate-level error type. All fallible operations in `autograph-core` return this.
#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("database error: {0}")]
    Db(#[from] rusqlite::Error),
}
