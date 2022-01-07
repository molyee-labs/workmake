use async_trait::async_trait;
use domain::account;
use domain::user;

pub struct Service {
    repo: Rc<dyn account::Repo>,
    user: Rc<dyn user::Service>,
}

pub enum Error {
    Unknown,
}

type Result<T, E = Error> = core::result::Result<T, E>;

#[async_trait]
impl account::Service for Service {
    type Err = Error;

    async fn create(&self, owner: account::Owner) -> Result<account::Id> {
        self.repo.create(owner)
    }

    async fn get(&self, id: account::Id) -> Result<account::Account> {
        self.repo.get(id)
    }

    async fn own(&self, owner: account::Owner) -> Result<account::Accounts> {
        self.repo.own(owner)
    }

    async fn change_owner(&self, id: account::Id, new: account::Owner) -> Result<()> {
        self.repo.cas_owner()
    }
}