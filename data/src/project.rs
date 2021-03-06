use crate::account::Accounts;
use crate::cost::Cost;
use core::ops::Range;
use uuid::Uuid;

pub struct Id(Uuid);

pub struct Project {
    id: Id,
    name: String,
    holders: Accounts,
}

pub struct Details {
    id: Id,
    desc: String,
    budget: Option<Range<Cost>>,
}
