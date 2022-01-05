use uuid::Uuid;
use url::Url;

pub enum Tx {
    Url(Url),
    Uuid(Uuid),
}
