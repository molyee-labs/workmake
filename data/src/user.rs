use crate::phone::Phone;
use crate::address::Address;
use email::Email;
use uuid::Uuid;
use url::Url;

pub struct Id(Uuid); // unique name

pub struct User {
    id: Id,
    icon: Url,
    name: String,
}

pub struct Profile {
    id: Id,
    name: Option<String>,
    midname: Option<String>,
    surname: Option<String>,
    email: Option<Email>,
    phone: Option<Phone>,
    photo: Option<Url>,
    address: Option<Address>,
}

pub struct Settings {
    id: Id,
}