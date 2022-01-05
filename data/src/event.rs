use crate::user;
use crate::time::DateTime;
use crate::action::Action;

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
