//! Types used for testing.

// FIXME: The tests using `serde_value` currently don't use all types due to an
// issue with this crate (https://github.com/arcnmx/serde-value/issues/18).
#![allow(unused)]
// Yes, we use `foo` and `bar` here... please don't complain...
#![allow(clippy::disallowed_names)]

use std::collections::BTreeMap;
use std::fmt::Display;

use serde::{Serialize, Serializer};


/// A simple unit-struct.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct UnitStruct;

/// A simple newtype-struct.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct NewtypeStruct<V>(pub V);

/// A simple tuple-struct.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TupleStruct<A, B, C, D>(pub A, pub B, pub C, pub D);

/// A simple struct with one field.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Struct<V> {
    pub foo: V,
}


/// An externally tagged enum.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum EnumTaggedExternal {
    Unit,
    NewtypeP(i32),
    NewtypeC(Struct<String>),
    Tuple(i32, i32, i32),
    Struct { foo: String },
}

/// An internally tagged enum.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "t")]
pub enum EnumTaggedInternal {
    Unit,
    NewtypeC(Struct<String>),
    Struct { foo: String },
}

/// An adjacently tagged enum.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "t", content = "c")]
pub enum EnumTaggedAdjacent {
    Unit,
    NewtypeP(i32),
    NewtypeC(Struct<String>),
    Tuple(i32, i32, i32),
    Struct { foo: String },
}

/// An untagged tagged enum.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum EnumUntagged {
    Unit,
    NewtypeP(i32),
    NewtypeC(Struct<String>),
    Tuple(i32, i32, i32),
    Struct { baz: String },
}


/// A type that serializes its contents using `serialize_seq`.
///
/// Note: `Vec` and other types use `collect_seq`.
#[derive(Debug, PartialEq)]
pub struct SerializeSeq<'a, V: 'a>(pub &'a V);

impl<'a, V> Serialize for SerializeSeq<'a, V>
where
    &'a V: IntoIterator,
    <&'a V as IntoIterator>::Item: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeSeq;

        let iter = self.0.into_iter();

        let mut state = serializer.serialize_seq(iter.size_hint().1)?;
        for val in iter {
            state.serialize_element(&val)?;
        }
        state.end()
    }
}

/// A type that serializes its contents using `serialize_seq` and does not
/// provide a length estimate.
///
/// Note: `Vec` and other types use `collect_seq`.
#[derive(Debug, PartialEq)]
pub struct SerializeSeqLenHidden<'a, V: 'a>(pub &'a V);

impl<'a, V> Serialize for SerializeSeqLenHidden<'a, V>
where
    &'a V: IntoIterator,
    <&'a V as IntoIterator>::Item: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeSeq;

        let iter = self.0.into_iter();

        let mut state = serializer.serialize_seq(None)?;
        for val in iter {
            state.serialize_element(&val)?;
        }
        state.end()
    }
}


/// A type that serializes its contents using `serialize_map`.
///
/// Note: `BTreeMap` and other types use `collect_map`.
#[derive(Debug, PartialEq)]
pub struct SerializeMap<'a, V: 'a>(pub &'a V);

impl<'a, V, A, B> Serialize for SerializeMap<'a, V>
where
    &'a V: IntoIterator<Item = (A, B)>,
    A: Serialize,
    B: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeMap;

        let iter = self.0.into_iter();

        let mut state = serializer.serialize_map(iter.size_hint().1)?;
        for (key, val) in iter {
            state.serialize_entry(&key, &val)?;
        }
        state.end()
    }
}

/// A type that serializes its contents using `serialize_map` and does not
/// provide a length estimate.
///
/// Note: `BTreeMap` and other types use `collect_map`.
#[derive(Debug, PartialEq)]
pub struct SerializeMapLenHidden<'a, V: 'a>(pub &'a V);

impl<'a, V, A, B> Serialize for SerializeMapLenHidden<'a, V>
where
    &'a V: IntoIterator<Item = (A, B)>,
    A: Serialize,
    B: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeMap;

        let iter = self.0.into_iter();

        let mut state = serializer.serialize_map(None)?;
        for (key, val) in iter {
            state.serialize_entry(&key, &val)?;
        }
        state.end()
    }
}


/// A type that serializes its contents using `collect_seq`.
#[derive(Debug, PartialEq)]
pub struct CollectSeq<V>(pub Vec<V>);

impl<V> Serialize for CollectSeq<V>
where
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self.0.iter())
    }
}


/// A type that serializes its contents using `collect_map`.
#[derive(Debug, PartialEq)]
pub struct CollectMap<K, V>(pub BTreeMap<K, V>);

impl<K, V> Serialize for CollectMap<K, V>
where
    K: Serialize,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_map(self.0.iter())
    }
}


/// A type that serializes its contents using `collect_str`.
#[derive(Debug, PartialEq)]
pub struct CollectStr<'a, D: ?Sized + 'a>(pub &'a D);

impl<'a, D: Display + ?Sized> Serialize for CollectStr<'a, D> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self.0)
    }
}
