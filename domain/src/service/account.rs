use async_trait::async_trait;
use data::account::{self, Account, Accounts, Owner};
use crate::error::Result;

#[async_trait]
pub trait AccountService {
    async fn create(&self, owner: Owner) -> Result<account::Id>;
    async fn get(&self, id: account::Id) -> Result<Account>;
    async fn own(&self, owner: Owner) -> Result<Accounts>;
    async fn change_owner(&self, id: account::Id, new: Owner) -> Result<()>;
}
