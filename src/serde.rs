use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{Id, Identify};

impl<T> Serialize for Id<T>
where
    T: Identify,
    T::InnerId: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for Id<T>
where
    T: Identify,
    T::InnerId: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let internal_id = T::InnerId::deserialize(deserializer)?;
        Ok(Id(internal_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestStruct {
        id: Id<TestStruct>,
        some_field: String,
    }

    impl Identify for TestStruct {
        type InnerId = i32;
    }

    #[test]
    fn ser_de() {
        let base = TestStruct {
            id: 42.into(),
            some_field: "some value".into(),
        };
        let ser = serde_json::to_string(&base).unwrap();
        let expected = "{\"id\":42,\"some_field\":\"some value\"}";
        let de = serde_json::from_str(expected).unwrap();

        assert_eq!(ser, expected);
        assert_eq!(base, de);
    }
}
