use crate::url::Url;
use crate::uuid::Uuid;

pub struct Id(Uuid);

pub struct Document {
    id: Id,
    icon: Url,
    name: String,
}

pub struct Details {
    id: Id,
    desc: String,
    image: Url,
    file: Url,
}