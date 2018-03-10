//! Deserialization of adjacently tagged values using maps.
//!
//! See [`ser::adj::map`](::ser::adj::map) for a description of this tagging
//! format.
//!
//! # Warning
//! If the deserialization-process depends on the tag (i.e. with
//! [`deserialize`](deserialize) and/or [`Visitor`](Visitor)),
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
pub fn deserialize<'de, 'a, T, V, K, Kc: ?Sized, D, F>(
    deserializer: D,
    tag_key: &'a Kc,
    value_key: &'a Kc,
    seed_factory: F,
) -> Result<V, D::Error>
where
    T: serde::Deserialize<'de>,
    V: serde::Deserialize<'de>,
    K: serde::Deserialize<'de>,
    K: std::cmp::PartialEq<&'a Kc>,
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
pub struct Visitor<'a, T, V, K, Kc: ?Sized + 'a, F> {
    seed_factory: F,
    tag_key:      &'a Kc,
    value_key:    &'a Kc,
    _phantom_t:   PhantomData<T>,
    _phantom_v:   PhantomData<V>,
    _phantom_k:   PhantomData<K>,
}

impl<'a, T, V, K, Kc: ?Sized, F> Visitor<'a, T, V, K, Kc, F> {
    /// Creates a new visitor with the given
    /// [`SeedFactory`](::de::seed::SeedFactory), tag-key and value-key.
    pub fn new(tag_key: &'a Kc, value_key: &'a Kc, seed_factory: F) -> Self {
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

impl<'de, 'a, T, V, K, Kc: ?Sized, F> serde::de::Visitor<'de> for Visitor<'a, T, V, K, Kc, F>
where
    T: serde::Deserialize<'de>,
    V: serde::Deserialize<'de>,
    K: serde::Deserialize<'de>,
    K: std::cmp::PartialEq<&'a Kc>,
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

        let key_1 = map.next_key_seed(KeySeed::<_, K>::new(self.tag_key, self.value_key))?
            .ok_or_else(|| Error::invalid_length(0, &self))?;

        // if first key is for tag: directly deserialize value
        if key_1 == Key::Tag {
            let tag: T = map.next_value()?;

            let value_key = map.next_key_seed(KeySeed::<_, K>::new(self.tag_key, self.value_key))?
                .ok_or_else(|| Error::invalid_length(1, &self))?;

            if value_key == Key::Value {
                map.next_value_seed(self.seed_factory.seed(tag)?)
            } else {
                Err(Error::custom("duplicate tag-key"))
            }

        // if first key is for value: cache value
        } else {
            let value: Content = map.next_value()?;

            let tag_key = map.next_key_seed(KeySeed::<_, K>::new(self.tag_key, self.value_key))?
                .ok_or_else(|| Error::invalid_length(1, &self))?;

            let tag: T = if tag_key == Key::Tag {
                map.next_value()
            } else {
                Err(Error::custom("duplicate value-key"))
            }?;

            let de = ContentDeserializer::new(value);
            self.seed_factory.seed(tag)?.deserialize(de)
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
pub fn deserialize_known<'de, 'a, T, V, K, Kc: ?Sized, D>(
    deserializer: D,
    tag_key: &'a Kc,
    value_key: &'a Kc,
) -> Result<(T, V), D::Error>
where
    T: serde::Deserialize<'de>,
    V: serde::Deserialize<'de>,
    K: serde::Deserialize<'de>,
    K: std::cmp::PartialEq<&'a Kc>,
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
pub struct KnownVisitor<'a, T, V, K, Kc: ?Sized + 'a> {
    tag_key:    &'a Kc,
    value_key:  &'a Kc,
    _phantom_t: PhantomData<T>,
    _phantom_v: PhantomData<V>,
    _phantom_k: PhantomData<K>,
}

impl<'a, T, V, K, Kc: ?Sized> KnownVisitor<'a, T, V, K, Kc> {
    /// Creates a new visitor with the given tag-key and value-key.
    pub fn new(tag_key: &'a Kc, value_key: &'a Kc) -> Self {
        KnownVisitor {
            tag_key,
            value_key,
            _phantom_t: PhantomData,
            _phantom_v: PhantomData,
            _phantom_k: PhantomData,
        }
    }
}

impl<'de, 'a, T, V, K, Kc: ?Sized> serde::de::Visitor<'de> for KnownVisitor<'a, T, V, K, Kc>
where
    T: serde::Deserialize<'de>,
    V: serde::Deserialize<'de>,
    K: serde::Deserialize<'de>,
    K: std::cmp::PartialEq<&'a Kc>,
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

        let key_1 = map.next_key_seed(KeySeed::<_, K>::new(self.tag_key, self.value_key))?
            .ok_or_else(|| Error::invalid_length(0, &self))?;

        if key_1 == Key::Tag {
            let tag: T = map.next_value()?;

            let value_key = map.next_key_seed(KeySeed::<_, K>::new(self.tag_key, self.value_key))?
                .ok_or_else(|| Error::invalid_length(1, &self))?;

            if value_key == Key::Value {
                Ok((tag, map.next_value()?))
            } else {
                Err(Error::custom("duplicate tag-key"))
            }
        } else {
            let value: V = map.next_value()?;

            let tag_key = map.next_key_seed(KeySeed::<_, K>::new(self.tag_key, self.value_key))?
                .ok_or_else(|| Error::invalid_length(1, &self))?;

            if tag_key == Key::Tag {
                Ok((map.next_value()?, value))
            } else {
                Err(Error::custom("duplicate value-key"))
            }
        }
    }
}


#[derive(PartialEq)]
enum Key {
    Tag,
    Value,
}


struct KeySeed<'a, Kc: ?Sized + 'a, Kd> {
    tag_key:     &'a Kc,
    value_key:   &'a Kc,
    _phantom_kd: std::marker::PhantomData<Kd>,
}

impl<'a, Kc: ?Sized, Kd> KeySeed<'a, Kc, Kd> {
    fn new(tag_key: &'a Kc, value_key: &'a Kc) -> Self {
        KeySeed {
            tag_key:     tag_key,
            value_key:   value_key,
            _phantom_kd: std::marker::PhantomData,
        }
    }
}

impl<'de, 'a, Kc: ?Sized, Kd> serde::de::DeserializeSeed<'de> for KeySeed<'a, Kc, Kd>
where
    Kd: serde::de::Deserialize<'de>,
    Kd: std::cmp::PartialEq<&'a Kc>,
{
    type Value = Key;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let key = Kd::deserialize(deserializer)?;

        if key == self.tag_key {
            Ok(Key::Tag)
        } else if key == self.value_key {
            Ok(Key::Value)
        } else {
            Err(serde::de::Error::custom(
                &"invalid entry key, expected either the specified tag- or value-key",
            ))
        }
    }
}
