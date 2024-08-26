use sqlx::{
    encode::IsNull,
    error::BoxDynError,
    sqlite::{SqliteArgumentValue, SqliteTypeInfo, SqliteValueRef},
    Decode, Encode, Sqlite, Type,
};

use crate::{Id, Identify};

impl<'a, T> Encode<'a, Sqlite> for Id<T>
where
    T: Identify,
    T::InnerId: Encode<'a, Sqlite>,
{
    fn encode_by_ref(&self, buf: &mut Vec<SqliteArgumentValue<'a>>) -> Result<IsNull, BoxDynError> {
        self.0.encode_by_ref(buf)
    }

    fn produces(&self) -> Option<<Sqlite as sqlx::Database>::TypeInfo> {
        self.0.produces()
    }

    fn size_hint(&self) -> usize {
        self.0.size_hint()
    }
}

impl<'a, T> Decode<'a, Sqlite> for Id<T>
where
    T: Identify,
    T::InnerId: Decode<'a, Sqlite>,
{
    fn decode(value: SqliteValueRef<'a>) -> Result<Self, BoxDynError> {
        let decoded = T::InnerId::decode(value)?;
        Ok(Id::new(decoded))
    }
}

impl<T> Type<Sqlite> for Id<T>
where
    T: Identify,
    T::InnerId: Type<Sqlite>,
{
    fn type_info() -> SqliteTypeInfo {
        T::InnerId::type_info()
    }

    fn compatible(ty: &SqliteTypeInfo) -> bool {
        T::InnerId::compatible(ty)
    }
}
