use data::project::{self, Project, Details};
use crate::error::Result;

pub trait Repo {
    async fn get(&self, id: project::Id) -> Result<Project>;
    async fn details(&self, id: project::Id) -> Result<Details>;
}