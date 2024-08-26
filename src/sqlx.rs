use sqlx::{encode::IsNull, error::BoxDynError, Database, Decode, Encode, Type};

use crate::{Id, Identify};

impl<'a, DB, T> Encode<'a, DB> for Id<T>
where
    DB: Database,
    T: Identify,
    T::InnerId: Encode<'a, DB> + Type<DB>,
{
    fn encode_by_ref(&self, buf: &mut DB::ArgumentBuffer<'a>) -> Result<IsNull, BoxDynError> {
        self.0.encode_by_ref(buf)
    }

    fn produces(&self) -> Option<<DB as Database>::TypeInfo> {
        self.0.produces()
    }

    fn size_hint(&self) -> usize {
        self.0.size_hint()
    }
}

impl<'a, DB, T> Decode<'a, DB> for Id<T>
where
    DB: Database,
    T: Identify,
    T::InnerId: Decode<'a, DB> + Type<DB>,
{
    fn decode(value: DB::ValueRef<'a>) -> Result<Self, BoxDynError> {
        let decoded = T::InnerId::decode(value)?;
        Ok(Id::new(decoded))
    }
}
