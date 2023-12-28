//! Deserialization of internally tagged values.
//!
//! See [`ser::internal`](::ser::internal) for a description of this tagging
//! format.
//!
//! # Warning
//!
//! Deserialization of internally tagged values requires a self-describing
//! data format.

use de::seed::SeedFactory;
use util::de::content::{Content, ContentDeserializer, ContentVisitor};

use std;
use std::marker::PhantomData;

use serde;


/// Deserialize an internally tagged value.
///
/// The deserializer controls the underlying data format while the seed-factory
/// specifies the instructions (depending on the tag) on how the value should be
/// deserialized.
///
/// See [`de`](::de) for more information on
/// [`SeedFactory`](::de::SeedFactory) and implementations thereof.
///
/// See [`deserialize_seed`](deserialize_seed) for a version that allows you to
/// pass a `DeserializeSeed` to deserialize the tag. This version is equivalent
/// to `deserialize_seed(deserializer, tag_key, seed_factory, PhantomData<T>)`
pub fn deserialize<'de, T, D, F>(
    deserializer: D,
    tag_key: &'static str,
    seed_factory: F,
) -> Result<F::Value, D::Error>
where
    T: serde::Deserialize<'de>,
    D: serde::Deserializer<'de>,
    F: SeedFactory<'de, T>,
{
    deserialize_seed(deserializer, tag_key, seed_factory, PhantomData::<T>)
}


/// Deserialize an internally tagged value with the given tag-seed.
///
/// The deserializer controls the underlying data format while the seed-factory
/// specifies the instructions (depending on the tag) on how the value should be
/// deserialized.
///
/// See [`de`](::de) for more information on
/// [`SeedFactory`](::de::SeedFactory) and implementations thereof.
pub fn deserialize_seed<'de, D, F, S>(
    deserializer: D,
    tag_key: &'static str,
    seed_factory: F,
    tag_seed: S,
) -> Result<F::Value, D::Error>
where
    D: serde::Deserializer<'de>,
    F: SeedFactory<'de, S::Value>,
    S: serde::de::DeserializeSeed<'de>,
{
    deserializer.deserialize_any(Visitor::new(tag_key, seed_factory, tag_seed))
}


/// A visitor that can be used to deserialize an externally tagged value.
///
/// This visitor handles an externally tagged value, which is represented by a
/// map containing a single entry, where the key is the tag and the value is the
/// value that should be deserialized. Thus it will return an error if the
/// visited type is not a map.
///
/// The [`SeedFactory`](::de::SeedFactory) provided to this visitor
/// provides a `serde::de::DeserializeSeed` implementation depending on the tag,
/// which then determines how the value is going to be deserialized.
///
/// See [`de`](::de) for more information on
/// [`SeedFactory`](::de::SeedFactory) and implementations thereof.
pub struct Visitor<F, S> {
    seed_factory: F,
    tag_seed:     S,
    tag_key:      &'static str,
}

impl<F, S> Visitor<F, S> {
    /// Creates a new visitor with the given tag-key and
    /// [`SeedFactory`](::de::SeedFactory).
    pub fn new(tag_key: &'static str, seed_factory: F, tag_seed: S) -> Self {
        Visitor {
            seed_factory,
            tag_seed,
            tag_key,
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
        write!(fmt, "a tagged value")
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        use serde::de::DeserializeSeed;

        let (tag, val) = TaggedValueVisitor::new(self.tag_key).visit_seq(seq)?;

        self.seed_factory
            .seed(self.tag_seed.deserialize(ContentDeserializer::new(tag))?)?
            .deserialize(ContentDeserializer::new(val))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        use serde::de::DeserializeSeed;

        let (tag, val) = TaggedValueVisitor::new(self.tag_key).visit_map(map)?;

        self.seed_factory
            .seed(self.tag_seed.deserialize(ContentDeserializer::new(tag))?)?
            .deserialize(ContentDeserializer::new(val))
    }
}


struct TaggedValueVisitor {
    tag_key: &'static str,
}

impl TaggedValueVisitor {
    fn new(tag_key: &'static str) -> Self {
        TaggedValueVisitor { tag_key }
    }
}

impl<'de> serde::de::Visitor<'de> for TaggedValueVisitor {
    type Value = (Content<'de>, Content<'de>);

    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "a tagged value")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        use serde::de::value::SeqAccessDeserializer;
        use serde::de::{Deserialize, Error};

        let tag: Content = seq
            .next_element()?
            .ok_or_else(|| Error::missing_field(self.tag_key))?;

        let val = Content::deserialize(SeqAccessDeserializer::new(seq))?;

        Ok((tag, val))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        use serde::de::Error;

        let mut tag = None;
        let mut val = Vec::with_capacity(map.size_hint().unwrap_or(128));

        while let Some(key) = map.next_key_seed(TagOrValueSeed::new(self.tag_key))? {
            match key {
                TagOrValue::Tag => {
                    if tag.is_some() {
                        return Err(Error::duplicate_field(self.tag_key));
                    }
                    tag = Some(map.next_value()?);
                },
                TagOrValue::Value(key) => {
                    val.push((key, map.next_value()?));
                },
            }
        }

        let tag = tag.ok_or_else(|| Error::missing_field(self.tag_key))?;

        Ok((tag, Content::Map(val)))
    }
}


enum TagOrValue<'de> {
    Tag,
    Value(Content<'de>),
}


struct TagOrValueSeed {
    tag_key: &'static str,
}

impl TagOrValueSeed {
    fn new(tag_key: &'static str) -> Self {
        TagOrValueSeed { tag_key }
    }
}

impl<'de> serde::de::DeserializeSeed<'de> for TagOrValueSeed {
    type Value = TagOrValue<'de>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }
}

impl<'de> serde::de::Visitor<'de> for TagOrValueSeed {
    type Value = TagOrValue<'de>;

    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "a tag `{}` or any other value", self.tag_key)
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        ContentVisitor::new().visit_bool(v).map(TagOrValue::Value)
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        ContentVisitor::new().visit_i8(v).map(TagOrValue::Value)
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        ContentVisitor::new().visit_i16(v).map(TagOrValue::Value)
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        ContentVisitor::new().visit_i32(v).map(TagOrValue::Value)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        ContentVisitor::new().visit_i64(v).map(TagOrValue::Value)
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        ContentVisitor::new().visit_u8(v).map(TagOrValue::Value)
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        ContentVisitor::new().visit_u16(v).map(TagOrValue::Value)
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        ContentVisitor::new().visit_u32(v).map(TagOrValue::Value)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        ContentVisitor::new().visit_u64(v).map(TagOrValue::Value)
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        ContentVisitor::new().visit_f32(v).map(TagOrValue::Value)
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        ContentVisitor::new().visit_f64(v).map(TagOrValue::Value)
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        ContentVisitor::new().visit_char(v).map(TagOrValue::Value)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v == self.tag_key {
            Ok(TagOrValue::Tag)
        } else {
            ContentVisitor::new().visit_str(v).map(TagOrValue::Value)
        }
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v == self.tag_key {
            Ok(TagOrValue::Tag)
        } else {
            ContentVisitor::new()
                .visit_borrowed_str(v)
                .map(TagOrValue::Value)
        }
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v == self.tag_key {
            Ok(TagOrValue::Tag)
        } else {
            ContentVisitor::new().visit_string(v).map(TagOrValue::Value)
        }
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v == self.tag_key.as_bytes() {
            Ok(TagOrValue::Tag)
        } else {
            ContentVisitor::new().visit_bytes(v).map(TagOrValue::Value)
        }
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v == self.tag_key.as_bytes() {
            Ok(TagOrValue::Tag)
        } else {
            ContentVisitor::new()
                .visit_borrowed_bytes(v)
                .map(TagOrValue::Value)
        }
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v == self.tag_key.as_bytes() {
            Ok(TagOrValue::Tag)
        } else {
            ContentVisitor::new()
                .visit_byte_buf(v)
                .map(TagOrValue::Value)
        }
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        ContentVisitor::new().visit_none().map(TagOrValue::Value)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        ContentVisitor::new()
            .visit_some(deserializer)
            .map(TagOrValue::Value)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        ContentVisitor::new().visit_unit().map(TagOrValue::Value)
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        ContentVisitor::new()
            .visit_newtype_struct(deserializer)
            .map(TagOrValue::Value)
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        ContentVisitor::new().visit_seq(seq).map(TagOrValue::Value)
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        ContentVisitor::new().visit_map(map).map(TagOrValue::Value)
    }

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::EnumAccess<'de>,
    {
        ContentVisitor::new()
            .visit_enum(data)
            .map(TagOrValue::Value)
    }
}
