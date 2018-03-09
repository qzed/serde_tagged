//! Deserialization of adjacently tagged values using maps.
//!
//! See [`ser::adj::map`](::ser::adj::map) for a description of this tagging
//! format.
//!
//! # Warning
//! When the deserialization-process depends on the tag (i.e. with
//! [`deserialize`](`deserialize`) and/or [`Visitor`](`Visitor)),
//! deserialization of map-based adjacently tagged values is only supported for
//! self-describing formats.

use de::seed::SeedFactory;
use util::de::content::{Content, ContentDeserializer};

use std;
use std::marker::PhantomData;

use serde;


/// Deserialize a map-based adjacently tagged value.
///
/// The deserializer controls the underlying data format while the seed-factory
/// specifies the instructions (depending on the tag) on how the value should be
/// deserialized.
///
/// See [`de::seed`](::de::seed) for more information on
/// [`SeedFactory`](::de::seed::SeedFactory) and implementations thereof.
///
/// # Note
/// If you do not need to choose a specific deserialization-method based on the
/// tag, you should prefer [`deserialize_known`](deserialize_known) to this
/// method.
pub fn deserialize<'de, T, V, K, Kc, D, F>(
    deserializer: D,
    tag_key: Kc,
    value_key: Kc,
    seed_factory: F,
) -> Result<V, D::Error>
where
    T: serde::Deserialize<'de>,
    V: serde::Deserialize<'de>,
    K: serde::Deserialize<'de>,
    K: std::cmp::PartialEq<Kc>,
    D: serde::Deserializer<'de>,
    F: SeedFactory<'de, T, Value = V>,
{
    deserializer.deserialize_map(Visitor::<T, V, K, Kc, F>::new(
        tag_key,
        value_key,
        seed_factory,
    ))
}


/// A visitor that can be used to deserialize a map-based adjacently tagged
/// value.
///
/// This visitor handles a map-based adjacently tagged value, which is
/// represented by a map containing exactly two entries. One entry of this tuple
/// is a mapping from tag-key to tag, the other entry contains a mapping from
/// value-key to value. Thus this visitor will return an error if the visited
/// type is not a map with two entries.
///
/// The [`SeedFactory`](::de::seed::SeedFactory) provided to this visitor
/// provides a `serde::de::DeserializeSeed` implementation depending on the tag,
/// which then determines how the value is going to be deserialized.
///
/// See [`de::seed`](::de::seed) for more information on
/// [`SeedFactory`](::de::seed::SeedFactory) and implementations thereof.
/// 
/// # Note
/// If you do not need to choose a specific deserialization-method based on the
/// tag, you should prefer [`KnownVisitor`](KnownVisitor) to this visitor.
pub struct Visitor<T, V, K, Kc, F> {
    seed_factory: F,
    tag_key:      Kc,
    value_key:    Kc,
    _phantom_t:   PhantomData<T>,
    _phantom_v:   PhantomData<V>,
    _phantom_k:   PhantomData<K>,
}

impl<T, V, K, Kc, F> Visitor<T, V, K, Kc, F> {
    /// Creates a new visitor with the given
    /// [`SeedFactory`](::de::seed::SeedFactory), tag-key and value-key.
    pub fn new(tag_key: Kc, value_key: Kc, seed_factory: F) -> Self {
        Visitor {
            seed_factory,
            tag_key,
            value_key,
            _phantom_t: PhantomData,
            _phantom_v: PhantomData,
            _phantom_k: PhantomData,
        }
    }
}

impl<'de, T, V, K, Kc, F> serde::de::Visitor<'de> for Visitor<T, V, K, Kc, F>
where
    T: serde::Deserialize<'de>,
    V: serde::Deserialize<'de>,
    K: serde::Deserialize<'de>,
    K: std::cmp::PartialEq<Kc>,
    F: SeedFactory<'de, T, Value = V>,
{
    type Value = V;

    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "a map with exactly two entries")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        use serde::de::DeserializeSeed;
        use serde::de::Error;

        // try to validate the length
        match map.size_hint() {
            Some(n) if n != 2 => Err(Error::invalid_length(n, &self))?,
            _ => {},
        }

        let key_1: K = map.next_key()?
            .ok_or_else(|| Error::invalid_length(0, &self))?;

        // if first key is for tag: directly deserialize value
        if key_1 == self.tag_key {
            let tag: T = map.next_value()?;

            let value_key: K = map.next_key()?
                .ok_or_else(|| Error::invalid_length(1, &self))?;

            if value_key == self.value_key {
                map.next_value_seed(self.seed_factory.seed(tag)?)
            } else {
                Err(Error::custom(
                    "invalid entry key, expected the specified value-key",
                ))
            }

        // if first key is for value: cache value
        } else if key_1 == self.value_key {
            let value: Content = map.next_value()?;

            let tag_key: K = map.next_key()?
                .ok_or_else(|| Error::invalid_length(1, &self))?;

            let tag: T = if tag_key == self.tag_key {
                map.next_value()
            } else {
                Err(Error::custom(
                    "invalid entry key, expected the specified tag-key",
                ))
            }?;

            let de = ContentDeserializer::new(value);
            self.seed_factory.seed(tag)?.deserialize(de)
        } else {
            Err(Error::custom(
                "invalid entry key, expected either the specified tag- or value-key",
            ))
        }
    }
}


/// Deserialize a map-based adjacently tagged value of known type.
///
/// The deserializer controls the underlying data format while the seed-factory
/// specifies the instructions (depending on the tag) on how the value should be
/// deserialized.
///
/// See [`de::seed`](::de::seed) for more information on
/// [`SeedFactory`](::de::seed::SeedFactory) and implementations thereof.
///
/// # Note
/// If you do not need to choose a specific deserialization-method based on the
/// tag, you should prefer this method to [`deserialize`](deserialize).
pub fn deserialize_known<'de, T, V, K, Kc, D>(
    deserializer: D,
    tag_key: Kc,
    value_key: Kc,
) -> Result<(T, V), D::Error>
where
    T: serde::Deserialize<'de>,
    V: serde::Deserialize<'de>,
    K: serde::Deserialize<'de>,
    K: std::cmp::PartialEq<Kc>,
    D: serde::Deserializer<'de>,
{
    deserializer.deserialize_map(KnownVisitor::<T, V, K, Kc>::new(tag_key, value_key))
}


/// A visitor that can be used to deserialize a map-based adjacently tagged
/// value of known type.
///
/// This visitor handles a map-based adjacently tagged value, which is
/// represented by a map containing exactly two entries. One entry of this tuple
/// is a mapping from tag-key to tag, the other entry contains a mapping from
/// value-key to value. Thus this visitor will return an error if the visited
/// type is not a map with two entries.
///
/// This visitor is intended for use of known values, i.e. when no tag-specific
/// deserialization mehtod is required. Thus it does not need to cache values
/// which can improve the performance.
///
/// # Note
/// If you do not need to choose a specific deserialization-method based on the
/// tag, you should prefer this visitor to [`Visitor`](Visitor).
pub struct KnownVisitor<T, V, K, Kc> {
    tag_key:    Kc,
    value_key:  Kc,
    _phantom_t: PhantomData<T>,
    _phantom_v: PhantomData<V>,
    _phantom_k: PhantomData<K>,
}

impl<T, V, K, Kc> KnownVisitor<T, V, K, Kc> {
    /// Creates a new visitor with the given tag-key and value-key.
    pub fn new(tag_key: Kc, value_key: Kc) -> Self {
        KnownVisitor {
            tag_key,
            value_key,
            _phantom_t: PhantomData,
            _phantom_v: PhantomData,
            _phantom_k: PhantomData,
        }
    }
}

impl<'de, T, V, K, Kc> serde::de::Visitor<'de> for KnownVisitor<T, V, K, Kc>
where
    T: serde::Deserialize<'de>,
    V: serde::Deserialize<'de>,
    K: serde::Deserialize<'de>,
    K: std::cmp::PartialEq<Kc>,
{
    type Value = (T, V);

    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "a map with exactly two entries")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        use serde::de::Error;

        // try to validate the length
        match map.size_hint() {
            Some(n) if n != 2 => Err(Error::invalid_length(n, &self))?,
            _ => {},
        }

        let key_1: K = map.next_key()?
            .ok_or_else(|| Error::invalid_length(0, &self))?;

        if key_1 == self.tag_key {
            let tag: T = map.next_value()?;

            let value_key: K = map.next_key()?
                .ok_or_else(|| Error::invalid_length(1, &self))?;

            if value_key == self.value_key {
                Ok((tag, map.next_value()?))
            } else {
                Err(Error::custom(
                    "invalid entry key, expected the specified value-key",
                ))
            }
        } else if key_1 == self.value_key {
            let value: V = map.next_value()?;

            let tag_key: K = map.next_key()?
                .ok_or_else(|| Error::invalid_length(1, &self))?;

            if tag_key == self.tag_key {
                Ok((map.next_value()?, value))
            } else {
                Err(Error::custom(
                    "invalid entry key, expected the specified tag-key",
                ))
            }
        } else {
            Err(Error::custom(
                "invalid entry key, expected either the specified tag- or value-key",
            ))
        }
    }
}
