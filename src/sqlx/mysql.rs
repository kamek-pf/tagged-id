use sqlx::{
    encode::IsNull,
    error::BoxDynError,
    mysql::{MySqlTypeInfo, MySqlValueRef},
    Decode, Encode, MySql, Type,
};

use crate::{Id, Identify};

impl<'a, T> Encode<'a, MySql> for Id<T>
where
    T: Identify,
    T::InnerId: Encode<'a, MySql>,
{
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        self.0.encode_by_ref(buf)
    }

    fn produces(&self) -> Option<<MySql as sqlx::Database>::TypeInfo> {
        self.0.produces()
    }

    fn size_hint(&self) -> usize {
        self.0.size_hint()
    }
}

impl<'a, T> Decode<'a, MySql> for Id<T>
where
    T: Identify,
    T::InnerId: Decode<'a, MySql>,
{
    fn decode(value: MySqlValueRef<'a>) -> Result<Self, BoxDynError> {
        let decoded = T::InnerId::decode(value)?;
        Ok(Id::new(decoded))
    }
}

impl<T> Type<MySql> for Id<T>
where
    T: Identify,
    T::InnerId: Type<MySql>,
{
    fn type_info() -> MySqlTypeInfo {
        T::InnerId::type_info()
    }

    fn compatible(ty: &MySqlTypeInfo) -> bool {
        T::InnerId::compatible(ty)
    }
}
