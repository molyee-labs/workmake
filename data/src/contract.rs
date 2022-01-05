use crate::account;
use crate::document;
use crate::cost::Cost;
use crate::transaction::Tx;
use time::Date;
use uuid::Uuid;

pub struct Id(Uuid);

pub struct Contract {
    id: Id,
    parties: Vec<account::Id>,
    docs: Vec<document::Id>,
    scheme: Scheme,
}

pub struct State {
    id: Id,
    changes: Vec<Change>,
    payments: Vec<Payment>,
}

pub struct Change {
    contract: Contract,
    accepted: Acceptance,
}

pub struct Payment {
    docs: Vec<document::Id>,
    payor: account::Id,
    payee: account::Id,
    value: Cost,
    accepted: Acceptance,
    tx: Option<Tx>,
}

struct Acceptance(u8);

pub enum Scheme {
    Unknown, // TODO custom scheme
    Basic(Basic),
}

pub struct Basic {
    customer: account::Id,
    contractor: account::Id,
    value: Cost,
    begin: Date,
    end: Date,
}