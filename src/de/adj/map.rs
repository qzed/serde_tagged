//! Deserialization of adjacently tagged values using maps.
//!
//! See [`ser::adj::map`](::ser::adj::map) for a description of this tagging
//! format.
//!
//! # Warning
//!
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
/// See [`de`](::de) for more information on
/// [`SeedFactory`](::de::SeedFactory) and implementations thereof.
///
/// See [`deserialize_seed`](deserialize_seed) for a version that allows you to
/// pass a `DeserializeSeed` implementation to deserialize the tag. This version
/// is equivalent to `deserialize_seed(deserializer, seed_factory,
/// PhantomData<T>)`
///
/// # Note
///
/// If you do not need to choose a specific deserialization-method based on the
/// tag, you should prefer [`deserialize_known`](deserialize_known) to this
/// method.
pub fn deserialize<'de, 'k, T, K, Kc: ?Sized, D, F>(
    deserializer: D,
    tag_key: &'k Kc,
    value_key: &'k Kc,
    seed_factory: F,
) -> Result<F::Value, D::Error>
where
    T: serde::Deserialize<'de>,
    D: serde::Deserializer<'de>,
    F: SeedFactory<'de, T>,
    K: serde::Deserialize<'de>,
    K: std::cmp::PartialEq<&'k Kc>,
{
    deserialize_seed::<K, _, _, _, _>(
        deserializer,
        tag_key,
        value_key,
        seed_factory,
        PhantomData::<T>,
    )
}


/// Deserialize a map-based adjacently tagged value with the given tag-seed.
///
/// The deserializer controls the underlying data format while the seed-factory
/// specifies the instructions (depending on the tag) on how the value should be
/// deserialized.
///
/// See [`de`](::de) for more information on
/// [`SeedFactory`](::de::SeedFactory) and implementations thereof.
///
/// # Note
///
/// If you do not need to choose a specific deserialization-method based on the
/// tag, you should prefer [`deserialize_known_seed`](deserialize_known_seed) to
/// this method.
pub fn deserialize_seed<'de, 'k, K, Kc: ?Sized, D, F, S>(
    deserializer: D,
    tag_key: &'k Kc,
    value_key: &'k Kc,
    seed_factory: F,
    tag_seed: S,
) -> Result<F::Value, D::Error>
where
    D: serde::Deserializer<'de>,
    F: SeedFactory<'de, S::Value>,
    S: serde::de::DeserializeSeed<'de>,
    K: serde::Deserialize<'de>,
    K: std::cmp::PartialEq<&'k Kc>,
{
    deserializer.deserialize_map(Visitor::<K, _, _, _>::new(
        tag_key,
        value_key,
        seed_factory,
        tag_seed,
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
/// The [`SeedFactory`](::de::SeedFactory) provided to this visitor
/// provides a `serde::de::DeserializeSeed` implementation depending on the tag,
/// which then determines how the value is going to be deserialized.
///
/// See [`de`](::de) for more information on
/// [`SeedFactory`](::de::SeedFactory) and implementations thereof.
///
/// # Note
///
/// If you do not need to choose a specific deserialization-method based on the
/// tag, you should prefer [`KnownVisitor`](KnownVisitor) to this visitor.
pub struct Visitor<'a, K, Kc: ?Sized + 'a, F, S> {
    seed_factory: F,
    tag_seed:     S,
    tag_key:      &'a Kc,
    value_key:    &'a Kc,
    _phantom_k:   PhantomData<K>,
}

impl<'a, K, Kc: ?Sized, F, S> Visitor<'a, K, Kc, F, S> {
    /// Creates a new visitor with the given
    /// [`SeedFactory`](::de::SeedFactory), tag-key and value-key.
    pub fn new(tag_key: &'a Kc, value_key: &'a Kc, seed_factory: F, tag_seed: S) -> Self {
        Visitor {
            seed_factory,
            tag_seed,
            tag_key,
            value_key,
            _phantom_k: PhantomData,
        }
    }
}

impl<'de, 'a, K, Kc: ?Sized, F, S> serde::de::Visitor<'de> for Visitor<'a, K, Kc, F, S>
where
    K: serde::Deserialize<'de>,
    K: std::cmp::PartialEq<&'a Kc>,
    F: SeedFactory<'de, S::Value>,
    S: serde::de::DeserializeSeed<'de>,
{
    type Value = F::Value;

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
            let tag = map.next_value_seed(self.tag_seed)?;

            let value_key = map.next_key_seed(KeySeed::<_, K>::new(self.tag_key, self.value_key))?
                .ok_or_else(|| Error::invalid_length(1, &"a map with exactly two entries"))?;

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

            let tag = if tag_key == Key::Tag {
                map.next_value_seed(self.tag_seed)
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
/// See [`de`](::de) for more information on
/// [`SeedFactory`](::de::SeedFactory) and implementations thereof.
///
/// See [`deserialize_known_seed`](deserialize_known_seed) for a version that
/// allows you to pass `DeserializeSeed` implementations to deserialize the tag
/// and value. This version is equivalent to
/// `deserialize_known_seed(deserializer, seed_factory, PhantomData<T>,
/// PhantomData<V>)`
///
/// # Note
///
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
    D: serde::Deserializer<'de>,
    K: serde::Deserialize<'de>,
    K: std::cmp::PartialEq<&'a Kc>,
{
    deserialize_known_seed::<K, _, _, _, _>(
        deserializer,
        tag_key,
        value_key,
        PhantomData::<T>,
        PhantomData::<V>,
    )
}


/// Deserialize a map-based adjacently tagged value of known type with the given seeds.
///
/// The deserializer controls the underlying data format while the seed-factory
/// specifies the instructions (depending on the tag) on how the value should be
/// deserialized.
///
/// See [`de`](::de) for more information on
/// [`SeedFactory`](::de::SeedFactory) and implementations thereof.
///
/// # Note
///
/// If you do not need to choose a specific deserialization-method based on the
/// tag, you should prefer this method to
/// [`deserialize_known`](deserialize_known).
pub fn deserialize_known_seed<'de, 'a, K, Kc: ?Sized, D, T, V>(
    deserializer: D,
    tag_key: &'a Kc,
    value_key: &'a Kc,
    tag_seed: T,
    value_seed: V,
) -> Result<(T::Value, V::Value), D::Error>
where
    T: serde::de::DeserializeSeed<'de>,
    V: serde::de::DeserializeSeed<'de>,
    D: serde::Deserializer<'de>,
    K: serde::Deserialize<'de>,
    K: std::cmp::PartialEq<&'a Kc>,
{
    deserializer.deserialize_map(KnownVisitor::<K, _, _, _>::new(
        tag_seed,
        value_seed,
        tag_key,
        value_key,
    ))
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
///
/// If you do not need to choose a specific deserialization-method based on the
/// tag, you should prefer this visitor to [`Visitor`](Visitor).
pub struct KnownVisitor<'a, K, Kc: ?Sized + 'a, T, V> {
    tag_seed:   T,
    value_seed: V,
    tag_key:    &'a Kc,
    value_key:  &'a Kc,
    _phantom_k: PhantomData<K>,
}

impl<'a, K, Kc: ?Sized, T, V> KnownVisitor<'a, K, Kc, T, V> {
    /// Creates a new visitor with the given tag-key and value-key.
    pub fn new(tag_seed: T, value_seed: V, tag_key: &'a Kc, value_key: &'a Kc) -> Self {
        KnownVisitor {
            tag_seed,
            value_seed,
            tag_key,
            value_key,
            _phantom_k: PhantomData,
        }
    }
}

impl<'de, 'a, K, Kc: ?Sized, T, V> serde::de::Visitor<'de> for KnownVisitor<'a, K, Kc, T, V>
where
    T: serde::de::DeserializeSeed<'de>,
    V: serde::de::DeserializeSeed<'de>,
    K: serde::Deserialize<'de>,
    K: std::cmp::PartialEq<&'a Kc>,
{
    type Value = (T::Value, V::Value);

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
            let tag = map.next_value_seed(self.tag_seed)?;

            let value_key = map.next_key_seed(KeySeed::<_, K>::new(self.tag_key, self.value_key))?
                .ok_or_else(|| Error::invalid_length(1, &"a map with exactly two entries"))?;

            if value_key == Key::Value {
                Ok((tag, map.next_value_seed(self.value_seed)?))
            } else {
                Err(Error::custom("duplicate tag-key"))
            }
        } else {
            let value = map.next_value_seed(self.value_seed)?;

            let tag_key = map.next_key_seed(KeySeed::<_, K>::new(self.tag_key, self.value_key))?
                .ok_or_else(|| Error::invalid_length(1, &"a map with exactly two entries"))?;

            if tag_key == Key::Tag {
                Ok((map.next_value_seed(self.tag_seed)?, value))
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
