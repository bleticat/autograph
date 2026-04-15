use crate::projects::Project;
use crate::shared::ports::repository::Repository;
use uuid::Uuid;

pub trait ProjectRepository: Repository<Project, Uuid> {}

impl<T> ProjectRepository for T where T: Repository<Project, Uuid> {}
