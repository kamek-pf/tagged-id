use sqlx::{
    encode::IsNull,
    error::BoxDynError,
    postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef},
    Decode, Encode, Postgres, Type,
};

use crate::{Id, Identify};

impl<'a, T> Encode<'a, Postgres> for Id<T>
where
    T: Identify,
    T::InnerId: Encode<'a, Postgres>,
{
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        self.0.encode_by_ref(buf)
    }

    fn produces(&self) -> Option<<Postgres as sqlx::Database>::TypeInfo> {
        self.0.produces()
    }

    fn size_hint(&self) -> usize {
        self.0.size_hint()
    }
}

impl<'a, T> Decode<'a, Postgres> for Id<T>
where
    T: Identify,
    T::InnerId: Decode<'a, Postgres>,
{
    fn decode(value: PgValueRef<'a>) -> Result<Self, BoxDynError> {
        let decoded = T::InnerId::decode(value)?;
        Ok(Id::new(decoded))
    }
}

impl<T> Type<Postgres> for Id<T>
where
    T: Identify,
    T::InnerId: Type<Postgres>,
{
    fn type_info() -> PgTypeInfo {
        T::InnerId::type_info()
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        T::InnerId::compatible(ty)
    }
}
