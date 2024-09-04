use uuid::Uuid;

use crate::{Id, Identify};

impl<T> From<Uuid> for Id<T>
where
    T: Identify<InnerId = Uuid>,
{
    fn from(uuid: Uuid) -> Self {
        Id(uuid)
    }
}
