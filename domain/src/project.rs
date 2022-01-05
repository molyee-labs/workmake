use async_trait::async_trait;
use data::account::Account;
use data::member::Members;
use data::project::{self, Project, Details};

#[async_trait]
pub trait Service {
    type Err;
    async fn create(&self, owner: Account, name: String) -> Result<Project, Self::Err>;
    async fn set_name(&self, id: project::Id, name: String) -> Result<(), Self::Err>;
    async fn details(&self, id: project::Id) -> Result<Details, Self::Err>;
    async fn set_details(&self, details: Details) -> Result<(), Self::Err>;
    async fn add_holder(&self, id: project::Id, holder: Account) -> Result<(), Self::Err>;
    async fn remove_holder(&self, id: project::Id, holder: Account) -> Result<(), Self::Err>;
    async fn members(&self, id: project::Id) -> Result<Members, Self::Err>;
    async fn add_members(&self, id: project::Id, members: Members) -> Result<(), Self::Err>;
    async fn remove_members(&self, id: project::Id, members: Members) -> Result<(), Self::Err>;
}

#[async_trait]
pub trait Repo {
    type Err;
    async fn get(&self, id: project::Id) -> Result<Project, Self::Err>;
    async fn details(&self, id: project::Id) -> Result<Details, Self::Err>;
}