use async_trait::async_trait;
use data::account::{self, Account, Accounts, Owner};
use data::user;
use data::company;

#[async_trait]
pub trait Service {
    type Err;
    async fn create(&self, owner: Owner) -> Result<account::Id, Self::Err>;
    async fn get(&self, id: account::Id) -> Result<Account, Self::Err>;
    async fn own(&self, owner: Owner) -> Result<Accounts, Self::Err>;
    async fn change_owner(&self, id: account::Id, new: Owner) -> Result<(), Self::Err>;
}

#[async_trait]
pub trait Repo {
    type Err;
    async fn get(&self, id: account::Id) -> Result<Account, Self::Err>;
    async fn user(&self, user: user::Id) -> Result<Accounts, Self::Err>;
    async fn company(&self, company: company::Id) -> Result<Accounts, Self::Err>;
    async fn partners(&self, id: account::Id) -> Result<Accounts, Self::Err>;
}
