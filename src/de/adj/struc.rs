//! Deserialization of adjacently tagged values using structs.
//!
//! See [`ser::adj::struc`](::ser::adj::struc) for a description of this tagging
//! format.
//!
//! # Warning
//! If the deserialization-process depends on the tag (i.e. with
//! [`deserialize`](`deserialize`) and/or [`Visitor`](`Visitor)),
//! deserialization of struct-based adjacently tagged values is only supported
//! for self-describing formats.

use de::seed::SeedFactory;
use util::de::content::{Content, ContentDeserializer};

use std;
use std::marker::PhantomData;

use serde;


pub fn deserialize<'de, T, V, D, F>(
    deserializer: D,
    name: &'static str,
    tag_key: &'static str,
    value_key: &'static str,
    seed_factory: F,
) -> Result<V, D::Error>
where
    T: serde::Deserialize<'de>,
    V: serde::Deserialize<'de>,
    D: serde::Deserializer<'de>,
    F: SeedFactory<'de, T, Value = V>,
{
    deserializer.deserialize_struct(
        name,
        &["tag-key", "value-key"],
        Visitor::<T, V, F>::new(tag_key, value_key, seed_factory),
    )
}

pub struct Visitor<T, V, F> {
    seed_factory: F,
    tag_key:      &'static str,
    value_key:    &'static str,
    _phantom_t:   PhantomData<T>,
    _phantom_v:   PhantomData<V>,
}

impl<T, V, F> Visitor<T, V, F> {
    /// Creates a new visitor with the given
    /// [`SeedFactory`](::de::seed::SeedFactory), tag-key and value-key.
    pub fn new(tag_key: &'static str, value_key: &'static str, seed_factory: F) -> Self {
        Visitor {
            seed_factory,
            tag_key,
            value_key,
            _phantom_t: PhantomData,
            _phantom_v: PhantomData,
        }
    }
}

impl<'de, T, V, F> serde::de::Visitor<'de> for Visitor<T, V, F>
where
    T: serde::Deserialize<'de>,
    V: serde::Deserialize<'de>,
    F: SeedFactory<'de, T, Value = V>,
{
    type Value = V;

    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "a struct with exactly two fields")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        use serde::de::DeserializeSeed;
        use serde::de::Error;

        let key_1 = map.next_key_seed(KeySeed::new(self.tag_key, self.value_key))?
            .ok_or_else(|| Error::invalid_length(0, &self))?;

        match key_1 {
            Key::Tag => {
                let tag: T = map.next_value()?;

                let value_key = map.next_key_seed(KeySeed::new(self.tag_key, self.value_key))?
                    .ok_or_else(|| Error::missing_field(self.value_key))?;

                if value_key == Key::Value {
                    Ok(map.next_value_seed(self.seed_factory.seed(tag)?)?)
                } else {
                    Err(Error::duplicate_field(self.tag_key))
                }
            },
            Key::Value => {
                let value: Content = map.next_value()?;

                let tag_key = map.next_key_seed(KeySeed::new(self.tag_key, self.value_key))?
                    .ok_or_else(|| Error::missing_field(self.value_key))?;

                let tag: T = if tag_key == Key::Tag {
                    map.next_value()
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

        let tag: T = seq.next_element()?
            .ok_or_else(|| Error::invalid_length(0, &self))?;

        let value: V = seq.next_element_seed(self.seed_factory.seed(tag)?)?
            .ok_or_else(|| Error::invalid_length(1, &"a struct with exactly two fields"))?;

        Ok(value)
    }
}


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
    deserializer.deserialize_struct(
        name,
        &["tag-key", "value-key"],
        KnownVisitor::<T, V>::new(tag_key, value_key),
    )
}

pub struct KnownVisitor<T, V> {
    tag_key:    &'static str,
    value_key:  &'static str,
    _phantom_t: PhantomData<T>,
    _phantom_v: PhantomData<V>,
}

impl<T, V> KnownVisitor<T, V> {
    /// Creates a new visitor with the given
    /// [`SeedFactory`](::de::seed::SeedFactory), tag-key and value-key.
    pub fn new(tag_key: &'static str, value_key: &'static str) -> Self {
        KnownVisitor {
            tag_key,
            value_key,
            _phantom_t: PhantomData,
            _phantom_v: PhantomData,
        }
    }
}

impl<'de, T, V> serde::de::Visitor<'de> for KnownVisitor<T, V>
where
    T: serde::Deserialize<'de>,
    V: serde::Deserialize<'de>,
{
    type Value = (T, V);

    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "a struct with exactly two fields")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        use serde::de::Error;

        let key_1 = map.next_key_seed(KeySeed::new(self.tag_key, self.value_key))?
            .ok_or_else(|| Error::invalid_length(0, &self))?;

        match key_1 {
            Key::Tag => {
                let tag: T = map.next_value()?;

                let value_key = map.next_key_seed(KeySeed::new(self.tag_key, self.value_key))?
                    .ok_or_else(|| Error::missing_field(self.value_key))?;

                if value_key == Key::Value {
                    Ok((tag, map.next_value()?))
                } else {
                    Err(Error::duplicate_field(self.tag_key))
                }
            },
            Key::Value => {
                let value: V = map.next_value()?;

                let tag_key = map.next_key_seed(KeySeed::new(self.tag_key, self.value_key))?
                    .ok_or_else(|| Error::missing_field(self.tag_key))?;

                if tag_key == Key::Tag {
                    Ok((map.next_value()?, value))
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

        let tag: T = seq.next_element()?
            .ok_or_else(|| Error::invalid_length(0, &self))?;

        let value: V = seq.next_element()?
            .ok_or_else(|| Error::invalid_length(1, &self))?;

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
            Err(serde::de::Error::custom(format_args!("invalid field name `{}`", value)))
        }
    }
}
