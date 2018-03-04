//! Types that can be used to indirectly call a function of a serializer using
//! `Serialize`.
//!
//! These types are required to assure that the right serializer function is
//! being called (e.g. in case of `serialize_bytes`) and can also be used to
//! forward a serializer function-call to another serializer.

use std::cell::Cell;
use std::fmt::Display;

use serde;


/// A type that serializes the enclosed `u8` slice using `serialize_bytes`.
pub struct Bytes<'a>(pub &'a [u8]);

impl<'a> serde::Serialize for Bytes<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(self.0)
    }
}


/// A type that serializes using `serialize_none`.
pub struct None;

impl serde::Serialize for None {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_none()
    }
}


/// A type that serializes the enclosed value using `serialize_some`.
pub struct Some<'a, V: ?Sized + 'a>(pub &'a V);

impl<'a, V: serde::Serialize + ?Sized + 'a> serde::Serialize for Some<'a, V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_some(self.0)
    }
}


/// A type that serializes using `serialize_unit`.
pub struct Unit;

impl serde::Serialize for Unit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_unit()
    }
}


/// A type that serializes using `serialize_unit_struct`.
pub struct UnitStruct(pub &'static str);

impl serde::Serialize for UnitStruct {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_unit_struct(self.0)
    }
}


/// A type that serializes using `serialize_unit_variant`.
pub struct UnitVariant(pub &'static str, pub u32, pub &'static str);

impl serde::Serialize for UnitVariant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_unit_variant(self.0, self.1, self.2)
    }
}


/// A type that serializes using `serialize_newtype_struct`.
pub struct NewtypeStruct<'a, V: ?Sized + 'a>(pub &'static str, pub &'a V);

impl<'a, V: serde::Serialize + ?Sized + 'a> serde::Serialize for NewtypeStruct<'a, V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_newtype_struct(self.0, self.1)
    }
}


/// A type that serializes using `serialize_newtype_variant`.
pub struct NewtypeVariant<'a, V: ?Sized + 'a>(
    pub &'static str,
    pub u32,
    pub &'static str,
    pub &'a V,
);

impl<'a, V: serde::Serialize + ?Sized + 'a> serde::Serialize for NewtypeVariant<'a, V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_newtype_variant(self.0, self.1, self.2, self.3)
    }
}


/// A type that serializes using `collect_seq`.
///
/// Forwards an owned sequence collected during serialization.
///
/// This struct will take ownership of the sequence and pass it on to the
/// serializer in the first call to serialize. Thus calling serialize on an
/// object of this type more than once is illegal and will result in a panic.
pub struct CollectSeq<I>(Cell<Option<I>>);

impl<I> CollectSeq<I> {
    pub fn new(iter: I) -> Self {
        CollectSeq(Cell::new(Option::Some(iter)))
    }
}

impl<I> serde::Serialize for CollectSeq<I>
where
    I: IntoIterator,
    <I as IntoIterator>::Item: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_seq(self.0.take().unwrap())
    }
}


/// A type that serializes using `collect_map`.
///
/// Forwards an owned key-value sequence collected during serialization.
///
/// This struct will take ownership of the sequence and pass it on to the
/// serializer in the first call to serialize. Thus calling serialize on an
/// object of this type more than once is illegal and will result in a panic.
pub struct CollectMap<I>(Cell<Option<I>>);

impl<I> CollectMap<I> {
    pub fn new(iter: I) -> Self {
        CollectMap(Cell::new(Option::Some(iter)))
    }
}

impl<I, K, V> serde::Serialize for CollectMap<I>
where
    K: serde::Serialize,
    V: serde::Serialize,
    I: IntoIterator<Item = (K, V)>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_map(self.0.take().unwrap())
    }
}


/// A type that serializes using `collect_str`.
pub struct CollectStr<'a, D: ?Sized + 'a>(pub &'a D);

impl<'a, D: ?Sized> serde::Serialize for CollectStr<'a, D>
where
    D: Display,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self.0)
    }
}
