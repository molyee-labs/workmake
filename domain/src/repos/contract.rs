use async_trait::async_trait;
use data::contract::{self, Contract, Scheme, State};
use data::account;
use crate::error::Result;

#[async_trait]
pub trait Repo {
    async fn get(&self, id: contract::Id) -> Result<Contract>;
    async fn own(&self, account: account::Id) -> Result<Vec<Contract>>;
    async fn listed(&self, accounts: &[account::Id]) -> Result<Vec<Contract>>;
    async fn schemes(&self) -> Result<Vec<Scheme>>;
    async fn state(&self, id: contract::Id) -> Result<State>;
}