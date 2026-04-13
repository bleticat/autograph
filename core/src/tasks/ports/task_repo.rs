use crate::shared::error::AppErr;
use crate::shared::ports::Transaction;

/// A repository for mutating todos.
///
/// Every implementor must also implement `From<Self::Tx>`, which serves as the
/// idiomatic Rust constructor: construction is expressed through the standard
/// [`From`] / [`Into`] traits rather than a bespoke `new` method in the trait.
/// This bound ensures that any future adapter cannot forget to provide a way to
/// create the repository from its associated transaction type.
pub trait TodoRepository: From<Self::Tx> {
    type Tx: Transaction;
    fn add(&self, title: &str) -> Result<i64, AppErr>;
    /// Toggles the `completed` flag of the todo with the given `id`.
    /// If no todo with that id exists this is a no-op (silently succeeds).
    fn toggle(&self, id: i64) -> Result<(), AppErr>;
    /// Deletes the todo with the given `id`.
    /// If no todo with that id exists this is a no-op (silently succeeds).
    fn delete(&self, id: i64) -> Result<(), AppErr>;
}
