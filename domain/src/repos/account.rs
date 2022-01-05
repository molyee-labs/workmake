use data::account::{self, Account, Accounts};
use data::user;
use crate::error::Result;

pub trait Repo {
    async fn get(&self, id: account::Id) -> Result<Account>;
    async fn user(&self, user: user::Id) -> Result<Accounts>;
    async fn company(&self, company: company::Id) -> Result<Accounts>;
    async fn partners(&self, id: account::Id) -> Result<Accounts>;
}