use async_trait::async_trait;
use email::Email;
use data::phone::Phone;
use data::user::{self, User, Profile};

pub enum Login {
    Name(String),
    Phone(Phone),
    Email(Email),
}

pub struct Password {
    inner: String,
}

#[async_trait]
pub trait Service {
    type Err;
    async fn create(&self, login: Login, passw: Password) -> Result<User, Self::Err>;
    async fn get(&self, id: user::Id) -> Result<User, Self::Err>;
    async fn profile(&self, id: user::Id) -> Result<Profile, Self::Err>;
    async fn set_profile(&self, profile: Profile) -> Result<(), Self::Err>;
    async fn set_name(&self, id: user::Id, name: String) -> Result<(), Self::Err>;
}

#[async_trait]
pub trait Repo {
    type Err;
    async fn get(&self, id: user::Id) -> Result<User, Self::Err>;
    async fn find(&self, name: &str) -> Result<Option<User>, Self::Err>;
    async fn profile(&self, id: user::Id) -> Result<Profile, Self::Err>;
    async fn contacts(&self, id: user::Id) -> Result<Vec<User>, Self::Err>;
}