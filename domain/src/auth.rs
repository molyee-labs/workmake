use crate::user::{Login, Password};
use async_trait::async_trait;
use data::user::User;

#[async_trait]
pub trait Service {
    type Err;
    async fn register(&self, login: Login, passw: Password) -> Result<(), Self::Err>;
    async fn authentificate(&self, login: Login, passw: Password) -> Result<User, Self::Err>;
}