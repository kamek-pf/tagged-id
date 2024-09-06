#![doc = include_str!("../README.md")]

use core::{
    cmp::Ordering,
    fmt::{self, Debug, Display, Formatter},
    hash::{Hash, Hasher},
};

#[cfg(feature = "derive")]
pub use tagged_id_derive::Id;

#[cfg(feature = "smartstring")]
mod smartstring;

#[cfg(feature = "serde")]
mod serde;

#[cfg(feature = "uuid")]
mod uuid;

mod sqlx;

/// A zero-cost newtype wrapper for your IDs.
pub struct Id<T: Identify>(pub(crate) T::InnerId);

impl<T: Identify> Id<T> {
    /// Construct a new Id from a base type.
    pub fn new(id: T::InnerId) -> Id<T> {
        Id(id)
    }

    /// Take the underlying ID out of the newtype wrapper.
    pub fn take(self) -> T::InnerId {
        self.0
    }
}

/// The Identify trait associates an Id with the resource it represents.
pub trait Identify {
    /// Raw ID of the resource.
    type InnerId;
}

impl<T> Clone for Id<T>
where
    T: Identify,
    T::InnerId: Clone,
{
    fn clone(&self) -> Self {
        Id(self.0.clone())
    }
}

impl<T> Copy for Id<T>
where
    T: Identify,
    T::InnerId: Copy,
{
}

impl<T> Debug for Id<T>
where
    T: Identify,
    T::InnerId: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<T> Display for Id<T>
where
    T: Identify,
    T::InnerId: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> PartialEq for Id<T>
where
    T: Identify,
    T::InnerId: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for Id<T>
where
    T: Identify,
    T::InnerId: Eq,
{
}

impl<T> PartialOrd for Id<T>
where
    T: Identify,
    T::InnerId: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T> Ord for Id<T>
where
    T: Identify,
    T::InnerId: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T> Hash for Id<T>
where
    T: Identify,
    T::InnerId: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> Default for Id<T>
where
    T: Identify,
    T::InnerId: Default,
{
    fn default() -> Self {
        Id(T::InnerId::default())
    }
}

impl<T> From<u8> for Id<T>
where
    T: Identify<InnerId = u8>,
{
    fn from(value: u8) -> Self {
        Id(value)
    }
}

impl<T> From<i32> for Id<T>
where
    T: Identify<InnerId = i32>,
{
    fn from(value: i32) -> Self {
        Id(value)
    }
}

impl<T> From<i64> for Id<T>
where
    T: Identify<InnerId = i64>,
{
    fn from(value: i64) -> Self {
        Id(value)
    }
}

impl<T> From<u32> for Id<T>
where
    T: Identify<InnerId = u32>,
{
    fn from(value: u32) -> Self {
        Id(value)
    }
}

impl<T> From<u64> for Id<T>
where
    T: Identify<InnerId = u64>,
{
    fn from(value: u64) -> Self {
        Id(value)
    }
}

#[cfg(not(feature = "smartstring"))]
impl<T> From<&str> for Id<T>
where
    T: Identify<InnerId = String>,
{
    fn from(value: &str) -> Self {
        Id(value.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate as tagged_id;

    #[test]
    fn copy() {
        struct TestStruct();

        impl Identify for TestStruct {
            type InnerId = i32;
        }

        let id1: Id<TestStruct> = 42.into();
        let id2 = id1;

        assert_eq!(id1, id2);

        let t = trybuild::TestCases::new();
        t.compile_fail("tests/fail/not_copy.rs");
    }

    #[test]
    fn take_twice() {
        struct TestStruct();

        impl Identify for TestStruct {
            type InnerId = i32;
        }

        let id1: Id<TestStruct> = 42.into();
        let id2 = id1.take();

        assert_eq!(id1.take(), id2);

        let t = trybuild::TestCases::new();
        t.compile_fail("tests/fail/not_take_twice.rs");
    }

    #[test]
    fn tagged_eq() {
        struct TestStruct();

        impl Identify for TestStruct {
            type InnerId = i32;
        }

        let id1: Id<TestStruct> = 42.into();
        let id2: Id<TestStruct> = 42.into();
        let id3: Id<TestStruct> = 101.into();

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);

        let t = trybuild::TestCases::new();
        t.compile_fail("tests/fail/not_tagged_eq.rs");
    }

    #[test]
    fn tagged_eq_derive_i32() {
        #[derive(Id)]
        #[tagged_id(i32)]
        struct TestStruct;

        let id1: Id<TestStruct> = 42.into();
        let id2: Id<TestStruct> = 42.into();
        let id3: Id<TestStruct> = 101.into();

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn tagged_eq_derive_string() {
        #[derive(Id)]
        #[tagged_id(String)]
        struct TestStruct;

        let id1: Id<TestStruct> = "42".into();
        let id2: Id<TestStruct> = "42".into();
        let id3: Id<TestStruct> = "101".into();

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn tagged_eq_derive_string_path() {
        #[derive(Id)]
        #[tagged_id(std::string::String)]
        struct TestStruct;

        let id1: Id<TestStruct> = "42".into();
        let id2: Id<TestStruct> = "42".into();
        let id3: Id<TestStruct> = "101".into();

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }
}
