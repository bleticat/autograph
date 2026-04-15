use crate::projects::Project;
use crate::shared::ports::repository::Repository;

pub trait ProjectRepository: Repository<Project> {}
