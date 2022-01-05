use crate::user;
use crate::action::Action;
use time::DateTime;

pub struct Event {
    dtime: DateTime,
    action: Box<dyn Action>,
    reason: Reason,
}

pub struct Log(Vec<Event>);

pub enum Reason {
    User(user::Id),
    // TODO auto, custom or etc.
}
