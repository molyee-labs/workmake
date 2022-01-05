use crate::uuid::Uuid;
use crate::url::Url;

pub enum Tx {
    Url(Url),
    Uuid(Uuid),
}
