use async_trait::async_trait;
use email::Email;
use data::phone::Phone;
use data::user::{self, User, Profile};
use crate::error::Result;

pub enum Login {
    Name(String),
    Phone(Phone),
    Email(Email),
}

pub struct Password {
    inner: String,
}

#[async_trait]
pub trait UserService {
    async fn create(&self, login: Login, passw: Password) -> Result<User>;
    async fn get(&self, id: user::Id) -> Result<User>;
    async fn profile(&self, id: user::Id) -> Result<Profile>;
    async fn set_profile(&self, profile: Profile) -> Result<()>;
    async fn set_name(&self, id: user::Id, name: String) -> Result<()>;
}

