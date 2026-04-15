use crate::shared::ports::repository::Repository;
use crate::tasks::Todo;

pub trait TodoRepository: Repository<Todo> {}
