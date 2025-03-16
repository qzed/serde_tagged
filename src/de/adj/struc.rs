//! Deserialization of adjacently tagged values using structs.
//!
//! See [`ser::adj::struc`](::ser::adj::struc) for a description of this tagging
//! format.
//!
//! # Warning
//!
//! If the deserialization-process depends on the tag (i.e. with
//! [`deserialize`](deserialize) and/or [`Visitor`](Visitor)),
//! deserialization of struct-based adjacently tagged values is only supported
//! for self-describing formats.

use crate::de::seed::SeedFactory;
use crate::util::de::content::{Content, ContentDeserializer};

use std;
use std::marker::PhantomData;

use serde;


/// Deserialize a struct-based adjacently tagged value.
///
/// The deserializer controls the underlying data format while the seed-factory
/// specifies the instructions (depending on the tag) on how the value should be
/// deserialized.
///
/// `name` is the name with which the struct that will be serialized.
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
pub fn deserialize<'de, T, D, F>(
    deserializer: D,
    name: &'static str,
    tag_key: &'static str,
    value_key: &'static str,
    seed_factory: F,
) -> Result<F::Value, D::Error>
where
    T: serde::Deserialize<'de>,
    D: serde::Deserializer<'de>,
    F: SeedFactory<'de, T>,
{
    deserialize_seed(
        deserializer,
        name,
        tag_key,
        value_key,
        seed_factory,
        PhantomData::<T>,
    )
}


/// Deserialize a struct-based adjacently tagged value using the given tag-seed.
///
/// The deserializer controls the underlying data format while the seed-factory
/// specifies the instructions (depending on the tag) on how the value should be
/// deserialized.
///
/// `name` is the name with which the struct that will be serialized.
///
/// See [`de`](::de) for more information on
/// [`SeedFactory`](::de::SeedFactory) and implementations thereof.
///
/// # Note
///
/// If you do not need to choose a specific deserialization-method based on the
/// tag, you should prefer [`deserialize_known_seed`](deserialize_known_seed) to
/// this method.
pub fn deserialize_seed<'de, D, F, S>(
    deserializer: D,
    name: &'static str,
    tag_key: &'static str,
    value_key: &'static str,
    seed_factory: F,
    tag_seed: S,
) -> Result<F::Value, D::Error>
where
    D: serde::Deserializer<'de>,
    F: SeedFactory<'de, S::Value>,
    S: serde::de::DeserializeSeed<'de>,
{
    deserializer.deserialize_struct(
        name,
        &["tag-key", "value-key"],
        Visitor::new(tag_key, value_key, seed_factory, tag_seed),
    )
}


/// A visitor that can be used to deserialize a struct-based adjacently tagged
/// value.
///
/// This visitor handles a struct-based adjacently tagged value, which is
/// represented by a struct containing exactly two fields. The first field of
/// this tuple is named according to tag-key and contains tag, the second field
/// is named according to value-key and contains the value. Thus this visitor
/// will return an error if the visited type is not a map or sequence with two
/// entries.
///
/// `name` is the name with which the struct that will be serialized.
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
pub struct Visitor<F, S> {
    seed_factory: F,
    tag_seed:     S,
    tag_key:      &'static str,
    value_key:    &'static str,
}

impl<F, S> Visitor<F, S> {
    /// Creates a new visitor with the given
    /// [`SeedFactory`](::de::SeedFactory), tag-key and value-key.
    pub fn new(
        tag_key: &'static str,
        value_key: &'static str,
        seed_factory: F,
        tag_seed: S,
    ) -> Self {
        Visitor {
            seed_factory,
            tag_seed,
            tag_key,
            value_key,
        }
    }
}

impl<'de, F, S> serde::de::Visitor<'de> for Visitor<F, S>
where
    F: SeedFactory<'de, S::Value>,
    S: serde::de::DeserializeSeed<'de>,
{
    type Value = F::Value;

    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "a struct with exactly two fields")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        use serde::de::DeserializeSeed;
        use serde::de::Error;

        let key_1 = map
            .next_key_seed(KeySeed::new(self.tag_key, self.value_key))?
            .ok_or_else(|| Error::invalid_length(0, &self))?;

        match key_1 {
            Key::Tag => {
                let tag = map.next_value_seed(self.tag_seed)?;

                let value_key = map
                    .next_key_seed(KeySeed::new(self.tag_key, self.value_key))?
                    .ok_or_else(|| Error::custom("missing value"))?;

                if value_key == Key::Value {
                    Ok(map.next_value_seed(self.seed_factory.seed(tag)?)?)
                } else {
                    Err(Error::duplicate_field(self.tag_key))
                }
            },
            Key::Value => {
                let value: Content = map.next_value()?;

                let tag_key = map
                    .next_key_seed(KeySeed::new(self.tag_key, self.value_key))?
                    .ok_or_else(|| Error::missing_field(self.value_key))?;

                let tag = if tag_key == Key::Tag {
                    map.next_value_seed(self.tag_seed)
                } else {
                    Err(Error::custom(
                        "invalid entry key, expected the specified tag-key",
                    ))
                }?;

                let de = ContentDeserializer::new(value);
                self.seed_factory.seed(tag)?.deserialize(de)
            },
        }
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        use serde::de::Error;

        let tag = seq
            .next_element_seed(self.tag_seed)?
            .ok_or_else(|| Error::invalid_length(0, &"a struct with exactly two fields"))?;

        let value = seq
            .next_element_seed(self.seed_factory.seed(tag)?)?
            .ok_or_else(|| Error::invalid_length(1, &"a struct with exactly two fields"))?;

        Ok(value)
    }
}


/// Deserialize a struct-based adjacently tagged value of known type.
///
/// The deserializer controls the underlying data format while the seed-factory
/// specifies the instructions (depending on the tag) on how the value should be
/// deserialized.
///
/// `name` is the name with which the struct that will be serialized.
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
pub fn deserialize_known<'de, T, V, D>(
    deserializer: D,
    name: &'static str,
    tag_key: &'static str,
    value_key: &'static str,
) -> Result<(T, V), D::Error>
where
    T: serde::Deserialize<'de>,
    V: serde::Deserialize<'de>,
    D: serde::Deserializer<'de>,
{
    deserialize_known_seed(
        deserializer,
        name,
        tag_key,
        value_key,
        PhantomData::<T>,
        PhantomData::<V>,
    )
}


/// Deserialize a struct-based adjacently tagged value of known type with the given seeds.
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
pub fn deserialize_known_seed<'de, T, V, D>(
    deserializer: D,
    name: &'static str,
    tag_key: &'static str,
    value_key: &'static str,
    tag_seed: T,
    value_seed: V,
) -> Result<(T::Value, V::Value), D::Error>
where
    T: serde::de::DeserializeSeed<'de>,
    V: serde::de::DeserializeSeed<'de>,
    D: serde::Deserializer<'de>,
{
    deserializer.deserialize_struct(
        name,
        &["tag-key", "value-key"],
        KnownVisitor::<T, V>::new(tag_seed, value_seed, tag_key, value_key),
    )
}


/// A visitor that can be used to deserialize a struct-based adjacently tagged
/// value of known type.
///
/// This visitor handles a struct-based adjacently tagged value, which is
/// represented by a struct containing exactly two fields. The first field of
/// this tuple is named according to tag-key and contains tag, the second field
/// is named according to value-key and contains the value. Thus this visitor
/// will return an error if the visited type is not a map or sequence with two
/// entries.
///
/// `name` is the name with which the struct that will be serialized.
///
/// This visitor is intended for use of known values, i.e. when no tag-specific
/// deserialization method is required. Thus it does not need to cache values
/// which can improve the performance.
///
/// # Note
///
/// If you do not need to choose a specific deserialization-method based on the
/// tag, you should prefer this visitor to [`Visitor`](Visitor).
pub struct KnownVisitor<T, V> {
    tag_seed:   T,
    value_seed: V,
    tag_key:    &'static str,
    value_key:  &'static str,
}

impl<T, V> KnownVisitor<T, V> {
    /// Creates a new visitor with the given
    /// [`SeedFactory`](::de::SeedFactory), tag-key and value-key.
    pub fn new(tag_seed: T, value_seed: V, tag_key: &'static str, value_key: &'static str) -> Self {
        KnownVisitor {
            tag_seed,
            value_seed,
            tag_key,
            value_key,
        }
    }
}

impl<'de, T, V> serde::de::Visitor<'de> for KnownVisitor<T, V>
where
    T: serde::de::DeserializeSeed<'de>,
    V: serde::de::DeserializeSeed<'de>,
{
    type Value = (T::Value, V::Value);

    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "a struct with exactly two fields")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        use serde::de::Error;

        let key_1 = map
            .next_key_seed(KeySeed::new(self.tag_key, self.value_key))?
            .ok_or_else(|| Error::invalid_length(0, &self))?;

        match key_1 {
            Key::Tag => {
                let tag = map.next_value_seed(self.tag_seed)?;

                let value_key = map
                    .next_key_seed(KeySeed::new(self.tag_key, self.value_key))?
                    .ok_or_else(|| Error::custom("missing value field"))?;

                if value_key == Key::Value {
                    Ok((tag, map.next_value_seed(self.value_seed)?))
                } else {
                    Err(Error::duplicate_field(self.tag_key))
                }
            },
            Key::Value => {
                let value = map.next_value_seed(self.value_seed)?;

                let tag_key = map
                    .next_key_seed(KeySeed::new(self.tag_key, self.value_key))?
                    .ok_or_else(|| Error::custom("missing tag field"))?;

                if tag_key == Key::Tag {
                    Ok((map.next_value_seed(self.tag_seed)?, value))
                } else {
                    Err(Error::duplicate_field(self.value_key))
                }
            },
        }
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        use serde::de::Error;

        let tag = seq
            .next_element_seed(self.tag_seed)?
            .ok_or_else(|| Error::invalid_length(0, &"a struct with exactly two fields"))?;

        let value = seq
            .next_element_seed(self.value_seed)?
            .ok_or_else(|| Error::invalid_length(1, &"a struct with exactly two fields"))?;

        Ok((tag, value))
    }
}


#[derive(PartialEq)]
enum Key {
    Tag,
    Value,
}


struct KeySeed {
    tag_key:   &'static str,
    value_key: &'static str,
}

impl KeySeed {
    fn new(tag_key: &'static str, value_key: &'static str) -> Self {
        KeySeed { tag_key, value_key }
    }
}

impl<'de> serde::de::DeserializeSeed<'de> for KeySeed {
    type Value = Key;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_identifier(self)
    }
}

impl<'de> serde::de::Visitor<'de> for KeySeed {
    type Value = Key;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("either the specified tag- or value-key")
    }

    fn visit_str<E>(self, value: &str) -> Result<Key, E>
    where
        E: serde::de::Error,
    {
        if value == self.tag_key {
            Ok(Key::Tag)
        } else if value == self.value_key {
            Ok(Key::Value)
        } else {
            Err(serde::de::Error::custom(format_args!(
                "invalid field name `{}`",
                value
            )))
        }
    }

    fn visit_i64<E>(self, value: i64) -> Result<Key, E>
    where
        E: serde::de::Error,
    {
        if value == 0 {
            Ok(Key::Tag)
        } else if value == 1 {
            Ok(Key::Value)
        } else {
            Err(serde::de::Error::custom(format_args!(
                "invalid field id `{}`",
                value
            )))
        }
    }

    fn visit_u64<E>(self, value: u64) -> Result<Key, E>
    where
        E: serde::de::Error,
    {
        if value == 0 {
            Ok(Key::Tag)
        } else if value == 1 {
            Ok(Key::Value)
        } else {
            Err(serde::de::Error::custom(format_args!(
                "invalid field id `{}`",
                value
            )))
        }
    }
}
