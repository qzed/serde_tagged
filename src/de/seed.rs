//! `DeserializeSeed` implementations for tagged value deserialization and creation thereof.

use std;
use std::collections::{BTreeMap, HashMap};
use std::marker::PhantomData;

use crate::util::TagString;

use serde;


/// A factory that creates a `DeserializeSeed` implementation based on a given
/// tag.
///
/// Convenience implementations of this trait are provided for the standard map
/// types with `&'static str` and `String` as key, as well as
/// [`WithoutTag`](WithoutTag) and [`WithTag`](WithTag).
pub trait SeedFactory<'de, T> {
    /// The type of the value that will be produced by the `DeserializeSeed`
    /// implementation.
    type Value;

    /// The type of the `DeserializeSeed` implementation that will be returned
    /// by this factory.
    type Seed: serde::de::DeserializeSeed<'de, Value = Self::Value>;

    /// Returns the seed that should be used to deserialize a value tagged with
    /// the given tag.
    fn seed<E>(self, tag: T) -> Result<Self::Seed, E>
    where
        E: serde::de::Error;
}


impl<'de, T, V> SeedFactory<'de, T> for PhantomData<V>
where
    V: serde::Deserialize<'de>,
{
    type Value = V;
    type Seed = PhantomData<V>;

    fn seed<E>(self, _tag: T) -> Result<Self::Seed, E>
    where
        E: serde::de::Error,
    {
        Ok(PhantomData)
    }
}


/// A [`SeedFactory`](SeedFactory) implementation that can be used to discard
/// the tag during deserialization.
///
/// This implementation creates a seed which deserializes a tagged value with
/// known type and discards the tag.
///
/// This is equivalent to the [`SeedFactory`](SeedFactory) implementation
/// provided for `PhantomData` and has been added for consistency and
/// clarification.
pub struct WithoutTag<V> {
    _phantom: PhantomData<V>,
}

impl<V> WithoutTag<V> {
    /// Creates a new [`SeedFactory`](SeedFactory) implementation that
    /// deserializes a tagged value with known type and discards its tag.
    pub fn new() -> Self {
        WithoutTag {
            _phantom: PhantomData,
        }
    }
}

impl<V> Default for WithoutTag<V> {
    fn default() -> Self {
        WithoutTag::new()
    }
}

impl<'de, T, V> SeedFactory<'de, T> for WithoutTag<V>
where
    V: serde::Deserialize<'de>,
{
    type Value = V;
    type Seed = PhantomData<V>;

    fn seed<E>(self, _tag: T) -> Result<Self::Seed, E>
    where
        E: serde::de::Error,
    {
        Ok(PhantomData)
    }
}

/// A [`SeedFactory`](SeedFactory) implementation that can be used to retreive
/// a tag and value of known type.
///
/// This implementation creates a seed which deserializes a tagged value with
/// known type and returns a tuple containing tag and value.
pub struct WithTag<V> {
    _phantom: PhantomData<V>,
}

impl<V> WithTag<V> {
    /// Creates a new [`SeedFactory`](SeedFactory) implementation that
    /// deserializes a tagged value with known type and returns both tag and
    /// value as tuple.
    pub fn new() -> Self {
        WithTag {
            _phantom: PhantomData,
        }
    }
}

impl<V> Default for WithTag<V> {
    fn default() -> Self {
        WithTag::new()
    }
}

impl<'de, V, T> SeedFactory<'de, T> for WithTag<V>
where
    V: serde::Deserialize<'de>,
{
    type Value = (T, V);
    type Seed = DeserializeWithTag<T, V>;

    fn seed<E>(self, tag: T) -> Result<Self::Seed, E>
    where
        E: serde::de::Error,
    {
        Ok(DeserializeWithTag::new(tag))
    }
}


/// A `DeserializeSeed` implementation that returns the pair of wrapped tag and
/// deserialized value of known type.
pub struct DeserializeWithTag<T, V> {
    tag:      T,
    _phantom: PhantomData<V>,
}

impl<T, V> DeserializeWithTag<T, V> {
    /// Creates a new `DeserializeSeed` implementation that returns the pair of
    /// given tag and deserialized value.
    pub fn new(tag: T) -> Self {
        DeserializeWithTag {
            tag,
            _phantom: PhantomData,
        }
    }
}

impl<'de, T, V> serde::de::DeserializeSeed<'de> for DeserializeWithTag<T, V>
where
    V: serde::Deserialize<'de>,
{
    type Value = (T, V);

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok((self.tag, V::deserialize(deserializer)?))
    }
}


impl<'de, V, S> SeedFactory<'de, TagString<'de>> for BTreeMap<&'static str, S>
where
    S: serde::de::DeserializeSeed<'de, Value = V>,
{
    type Value = V;
    type Seed = S;

    fn seed<E>(mut self, tag: TagString<'de>) -> Result<Self::Seed, E>
    where
        E: serde::de::Error,
    {
        self.remove(tag.as_ref())
            .ok_or_else(|| serde::de::Error::custom("Unknown tag"))
    }
}

impl<'de, V, S, H> SeedFactory<'de, TagString<'de>> for HashMap<&'static str, S, H>
where
    S: serde::de::DeserializeSeed<'de, Value = V>,
    H: std::hash::BuildHasher,
{
    type Value = V;
    type Seed = S;

    fn seed<E>(mut self, tag: TagString<'de>) -> Result<Self::Seed, E>
    where
        E: serde::de::Error,
    {
        self.remove(tag.as_ref())
            .ok_or_else(|| serde::de::Error::custom("Unknown tag"))
    }
}


impl<'r, 'de, V, S> SeedFactory<'de, TagString<'de>> for &'r mut BTreeMap<&'static str, S>
where
    &'r mut S: serde::de::DeserializeSeed<'de, Value = V>,
{
    type Value = V;
    type Seed = &'r mut S;

    fn seed<E>(self, tag: TagString<'de>) -> Result<Self::Seed, E>
    where
        E: serde::de::Error,
    {
        self.get_mut(tag.as_ref())
            .ok_or_else(|| serde::de::Error::custom("Unknown tag"))
    }
}

impl<'r, 'de, V, S, H> SeedFactory<'de, TagString<'de>> for &'r mut HashMap<&'static str, S, H>
where
    &'r mut S: serde::de::DeserializeSeed<'de, Value = V>,
    H: std::hash::BuildHasher,
{
    type Value = V;
    type Seed = &'r mut S;

    fn seed<E>(self, tag: TagString<'de>) -> Result<Self::Seed, E>
    where
        E: serde::de::Error,
    {
        self.get_mut(tag.as_ref())
            .ok_or_else(|| serde::de::Error::custom("Unknown tag"))
    }
}


impl<'r, 'de, V, S> SeedFactory<'de, TagString<'de>> for &'r BTreeMap<&'static str, S>
where
    &'r S: serde::de::DeserializeSeed<'de, Value = V>,
{
    type Value = V;
    type Seed = &'r S;

    fn seed<E>(self, tag: TagString<'de>) -> Result<Self::Seed, E>
    where
        E: serde::de::Error,
    {
        self.get(tag.as_ref())
            .ok_or_else(|| serde::de::Error::custom("Unknown tag"))
    }
}

impl<'r, 'de, V, S, H> SeedFactory<'de, TagString<'de>> for &'r HashMap<&'static str, S, H>
where
    &'r S: serde::de::DeserializeSeed<'de, Value = V>,
    H: std::hash::BuildHasher,
{
    type Value = V;
    type Seed = &'r S;

    fn seed<E>(self, tag: TagString<'de>) -> Result<Self::Seed, E>
    where
        E: serde::de::Error,
    {
        self.get(tag.as_ref())
            .ok_or_else(|| serde::de::Error::custom("Unknown tag"))
    }
}


impl<'de, V, S> SeedFactory<'de, TagString<'de>> for BTreeMap<String, S>
where
    S: serde::de::DeserializeSeed<'de, Value = V>,
{
    type Value = V;
    type Seed = S;

    fn seed<E>(mut self, tag: TagString<'de>) -> Result<Self::Seed, E>
    where
        E: serde::de::Error,
    {
        self.remove(tag.as_ref())
            .ok_or_else(|| serde::de::Error::custom("Unknown tag"))
    }
}

impl<'de, V, S, H> SeedFactory<'de, TagString<'de>> for HashMap<String, S, H>
where
    S: serde::de::DeserializeSeed<'de, Value = V>,
    H: std::hash::BuildHasher,
{
    type Value = V;
    type Seed = S;

    fn seed<E>(mut self, tag: TagString<'de>) -> Result<Self::Seed, E>
    where
        E: serde::de::Error,
    {
        self.remove(tag.as_ref())
            .ok_or_else(|| serde::de::Error::custom("Unknown tag"))
    }
}


impl<'r, 'de, V, S> SeedFactory<'de, TagString<'de>> for &'r mut BTreeMap<String, S>
where
    &'r mut S: serde::de::DeserializeSeed<'de, Value = V>,
{
    type Value = V;
    type Seed = &'r mut S;

    fn seed<E>(self, tag: TagString<'de>) -> Result<Self::Seed, E>
    where
        E: serde::de::Error,
    {
        self.get_mut(tag.as_ref())
            .ok_or_else(|| serde::de::Error::custom("Unknown tag"))
    }
}

impl<'r, 'de, V, S, H> SeedFactory<'de, TagString<'de>> for &'r mut HashMap<String, S, H>
where
    &'r mut S: serde::de::DeserializeSeed<'de, Value = V>,
    H: std::hash::BuildHasher,
{
    type Value = V;
    type Seed = &'r mut S;

    fn seed<E>(self, tag: TagString<'de>) -> Result<Self::Seed, E>
    where
        E: serde::de::Error,
    {
        self.get_mut(tag.as_ref())
            .ok_or_else(|| serde::de::Error::custom("Unknown tag"))
    }
}


impl<'r, 'de, V, S> SeedFactory<'de, TagString<'de>> for &'r BTreeMap<String, S>
where
    &'r S: serde::de::DeserializeSeed<'de, Value = V>,
{
    type Value = V;
    type Seed = &'r S;

    fn seed<E>(self, tag: TagString<'de>) -> Result<Self::Seed, E>
    where
        E: serde::de::Error,
    {
        self.get(tag.as_ref())
            .ok_or_else(|| serde::de::Error::custom("Unknown tag"))
    }
}

impl<'r, 'de, V, S, H> SeedFactory<'de, TagString<'de>> for &'r HashMap<String, S, H>
where
    &'r S: serde::de::DeserializeSeed<'de, Value = V>,
    H: std::hash::BuildHasher,
{
    type Value = V;
    type Seed = &'r S;

    fn seed<E>(self, tag: TagString<'de>) -> Result<Self::Seed, E>
    where
        E: serde::de::Error,
    {
        self.get(tag.as_ref())
            .ok_or_else(|| serde::de::Error::custom("Unknown tag"))
    }
}


#[cfg(feature = "erased")]
mod erased {
    //! Utilities for trait-objects.

    use erased_serde;
    use serde;


    /// A trait alias for mutable closures that can be used as
    /// `DeserializeSeed` in combination with `BoxFnMutSeed`.
    pub trait FnMutSeed<V>:
        for<'de> FnMut(&mut dyn erased_serde::Deserializer<'de>) -> Result<V, erased_serde::Error>
    {
    }

    impl<V, F> FnMutSeed<V> for F where
        F: for<'de> FnMut(
            &mut dyn erased_serde::Deserializer<'de>,
        ) -> Result<V, erased_serde::Error>
    {
    }


    /// A boxed mutable closure that can be used as `DeserializeSeed`.
    ///
    /// It additionally requires the wrapped closure to implement `Sync` which
    /// allows for easy static type-registry creation, e.g. in combination with
    /// `BTreeMap<&'static str, _>`.
    pub struct BoxFnMutSeed<V>(
        Box<dyn FnMutSeed<V, Output = Result<V, erased_serde::Error>> + Sync>,
    );

    impl<V> BoxFnMutSeed<V> {
        /// Creates a new boxed closure from the given closure.
        pub fn new<F>(func: F) -> Self
        where
            F: FnMutSeed<V> + Sync + 'static,
        {
            BoxFnMutSeed(Box::new(func))
        }
    }

    impl<'de, V> serde::de::DeserializeSeed<'de> for BoxFnMutSeed<V> {
        type Value = V;

        fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let mut de = <dyn erased_serde::Deserializer>::erase(deserializer);
            (self.0)(&mut de).map_err(serde::de::Error::custom)
        }
    }

    impl<'de, V> serde::de::DeserializeSeed<'de> for &mut BoxFnMutSeed<V> {
        type Value = V;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let mut de = <dyn erased_serde::Deserializer>::erase(deserializer);
            (self.0)(&mut de).map_err(serde::de::Error::custom)
        }
    }


    /// A trait alias for (immutable) closures that can be used as
    /// `DeserializeSeed` in combination with `BoxFnSeed`.
    pub trait FnSeed<V>:
        for<'de> Fn(&mut dyn erased_serde::Deserializer<'de>) -> Result<V, erased_serde::Error>
    {
    }

    impl<V, F> FnSeed<V> for F where
        F: for<'de> Fn(&mut dyn erased_serde::Deserializer<'de>) -> Result<V, erased_serde::Error>
    {
    }


    /// A boxed (immutable) closure that can be used as `DeserializeSeed`.
    ///
    /// It additionally requires the wrapped closure to implement `Sync` which
    /// allows for easy static type-registry creation, e.g. in combination with
    /// `BTreeMap<&'static str, _>`.
    pub struct BoxFnSeed<V>(Box<dyn FnSeed<V, Output = Result<V, erased_serde::Error>> + Sync>);

    impl<V> BoxFnSeed<V> {
        /// Creates a new boxed closure from the given closure.
        pub fn new<F>(func: F) -> Self
        where
            F: FnSeed<V> + Sync + 'static,
        {
            BoxFnSeed(Box::new(func))
        }
    }

    impl<'de, V> serde::de::DeserializeSeed<'de> for BoxFnSeed<V> {
        type Value = V;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let mut de = <dyn erased_serde::Deserializer>::erase(deserializer);
            (self.0)(&mut de).map_err(serde::de::Error::custom)
        }
    }

    impl<'de, V> serde::de::DeserializeSeed<'de> for &BoxFnSeed<V> {
        type Value = V;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let mut de = <dyn erased_serde::Deserializer>::erase(deserializer);
            (self.0)(&mut de).map_err(serde::de::Error::custom)
        }
    }
}

#[cfg(feature = "erased")]
pub use self::erased::{BoxFnMutSeed, BoxFnSeed, FnMutSeed, FnSeed};
