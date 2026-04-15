use crate::shared::ports::repository::Repository;
use crate::tasks::Todo;
use uuid::Uuid;

pub trait TodoRepository: Repository<Todo, Uuid> {}

impl<T> TodoRepository for T where T: Repository<Todo, Uuid> {}
