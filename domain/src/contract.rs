use async_trait::async_trait;
use data::account;

pub use data::contract::{self, *};

#[async_trait]
pub trait Repo {
    type Err;
    async fn get(&self, id: contract::Id) -> Result<Contract, Self::Err>;
    async fn own(&self, account: account::Id) -> Result<Contracts, Self::Err>;
    async fn listed(&self, accounts: &[account::Id]) -> Result<Contracts, Self::Err>;
    async fn schemes(&self) -> Result<Vec<Scheme>, Self::Err>;
    async fn state(&self, id: contract::Id) -> Result<State, Self::Err>;
}