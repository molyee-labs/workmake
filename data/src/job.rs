use crate::event::Log;
use crate::project;
use crate::contract;
use crate::uuid::Uuid;

pub struct Id(Uuid);

pub struct Job {
    id: Id,
    name: String,
    parent: Option<Id>,
    project: project::Id,
}

pub struct Details {
    id: Id,
    desc: String,
    contract: Option<contract::Id>,
    history: Log,
}
