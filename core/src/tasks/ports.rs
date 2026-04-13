use super::Todo;
use crate::shared::ports::{Connection, Transaction};

pub trait TodoRepository {
    type Tx: Transaction;
    fn new(tx: &Self::Tx) -> Self
    where
        Self: Sized;
    fn add(&self, title: &str) -> Result<i64, String>;
    fn toggle(&self, id: i64) -> Result<(), String>;
    fn delete(&self, id: i64) -> Result<(), String>;
}

pub trait TaskQueries {
    type Conn: Connection;
    fn new(conn: Self::Conn) -> Self
    where
        Self: Sized;
    fn get_all_todos(&self) -> Result<Vec<Todo>, String>;
}
