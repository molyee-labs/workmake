use async_trait::async_trait;
use data::user::{self, User, Profile};
use crate::error::Result;

#[async_trait]
pub trait Repo {
    async fn get(&self, id: user::Id) -> Result<User>;
    async fn find(&self, name: &str) -> Result<Option<User>>;
    async fn profile(&self, id: user::Id) -> Result<Profile>;
    async fn contacts(&self, id: user::Id) -> Result<Vec<User>>;
}