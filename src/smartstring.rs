use smartstring::alias::CompactString;

use crate::{Id, Identify};

impl<T> From<&str> for Id<T>
where
    T: Identify<InnerId = CompactString>,
{
    fn from(value: &str) -> Self {
        Id(value.into())
    }
}
