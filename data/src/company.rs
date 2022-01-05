use crate::member::Members;
use crate::phone::Phone;
use crate::address::Address;
use uuid::Uuid;
use email::Email;
use url::Url;

pub struct Id(Uuid);

pub struct Team {
    name: String,
    members: Members,
}

pub struct Company {
    id: Id,
    icon: Url,
    name: String,
}

pub struct Profile {
    id: Id,
    logo: Option<Url>,
    desc: Option<String>,
    phone: Option<Phone>,
    email: Option<Email>,
    address: Option<Address>,
}

pub struct Settings {
    id: Id,
    teams: Vec<Team>,
}