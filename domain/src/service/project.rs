use data::account::Account;
use data::member::Members;
use data::project::{self, Project, Details};
use crate::error::Result;

pub trait ProjectService {
    async fn create(&self, owner: Account, name: String) -> Result<Project>;
    async fn set_name(&self, id: project::Id, name: String) -> Result<()>;
    async fn details(&self, id: project::Id) -> Result<Details>;
    async fn set_details(&self, details: Details) -> Result<()>;
    async fn add_holder(&self, id: project::Id, holder: Account) -> Result<()>;
    async fn remove_holder(&self, id: project::Id, holder: Account) -> Result<()>;
    async fn members(&self, id: project::Id) -> Result<Members>;
    async fn add_members(&self, id: project::Id, members: Members) -> Result<()>;
    async fn remove_members(&self, id: project::Id, members: Members) -> Result<()>;
}
