use crate::company;
use crate::user;
use crate::uuid::Uuid;

pub struct Id(Uuid);

pub enum Owner {
    User(user::Id),
    Company(company::Id),
}

pub struct Account {
    id: Id,
    owner: Owner,
}

pub struct Accounts(Vec<Account>);
