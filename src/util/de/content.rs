// This file is adapted from `serde`, specifically the modules `content` and
// `size_hint` in
// https://github.com/serde-rs/serde/blob/v1.0.29/serde/src/private/de.rs
//
// The original copyright notice:
// > Copyright 2017 Serde Developers
// >
// > Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// > http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// > <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// > option. This file may not be copied, modified, or distributed
// > except according to those terms.
//
// The mentioned license files for the original can be found at
// https://github.com/serde-rs/serde/blob/v1.0.27/LICENSE-APACHE
// https://github.com/serde-rs/serde/blob/v1.0.27/LICENSE-MIT
//
// The original MIT license file:
// > Copyright (c) 2014 The Rust Project Developers
// >
// > Permission is hereby granted, free of charge, to any
// > person obtaining a copy of this software and associated
// > documentation files (the "Software"), to deal in the
// > Software without restriction, including without
// > limitation the rights to use, copy, modify, merge,
// > publish, distribute, sublicense, and/or sell copies of
// > the Software, and to permit persons to whom the Software
// > is furnished to do so, subject to the following
// > conditions:
// >
// > The above copyright notice and this permission notice
// > shall be included in all copies or substantial portions
// > of the Software.
// >
// > THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// > ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// > TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// > PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// > SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// > CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// > OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// > IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// > DEALINGS IN THE SOFTWARE.
//
// Changes made to the original module:
// - re-formatted code using rustfmt (appearance, replaced try! with ?)
// - changed import, absolute and relative paths for compatibility
// - removed some comments
// - removed types and respective impls for:
//   - `TagOrContent`
//   - `TagOrContentVisitor`
//   - `TaggedContent`
//   - `TaggedContentField`
//   - `TaggedContentFieldVisitor`
//   - `TaggedContentOtherField`
//   - `TaggedContentOtherFieldVisitor`
//   - `InternallyTaggedUnitVisitor`
//   - `UntaggedUnitVisitor`
//   - `IdentifierDeserializer`
//   - `StrDeserializer`
//   - `BytesDeserializer`
//   - `InPlaceSeed`
// - removed unused imports
// - fixed clippy warnings
// - add special case for empty `Content::Seq` to `ContentDeserializer::deserialize_unit_struct`
// - changed visibility of `ContentVisitor` and its `new` function to public
//


pub mod size_hint {
    use std::cmp;

    pub fn from_bounds<I>(iter: &I) -> Option<usize>
    where
        I: Iterator,
    {
        helper(iter.size_hint())
    }

    #[inline]
    pub fn cautious(hint: Option<usize>) -> usize {
        cmp::min(hint.unwrap_or(0), 4096)
    }

    fn helper(bounds: (usize, Option<usize>)) -> Option<usize> {
        match bounds {
            (lower, Some(upper)) if lower == upper => Some(upper),
            _ => None,
        }
    }
}


use std::fmt;
use std::marker::PhantomData;

use serde::de::{
    self,
    Deserialize,
    Deserializer,
    EnumAccess,
    MapAccess,
    SeqAccess,
    Unexpected,
    Visitor,
};

#[derive(Debug)]
pub enum Content<'de> {
    Bool(bool),

    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),

    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),

    F32(f32),
    F64(f64),

    Char(char),
    String(String),
    Str(&'de str),
    ByteBuf(Vec<u8>),
    Bytes(&'de [u8]),

    None,
    Some(Box<Content<'de>>),

    Unit,
    Newtype(Box<Content<'de>>),
    Seq(Vec<Content<'de>>),
    Map(Vec<(Content<'de>, Content<'de>)>),
}

impl<'de> Content<'de> {
    fn unexpected(&self) -> Unexpected {
        match *self {
            Content::Bool(b) => Unexpected::Bool(b),
            Content::U8(n) => Unexpected::Unsigned(u64::from(n)),
            Content::U16(n) => Unexpected::Unsigned(u64::from(n)),
            Content::U32(n) => Unexpected::Unsigned(u64::from(n)),
            Content::U64(n) => Unexpected::Unsigned(n),
            Content::I8(n) => Unexpected::Signed(i64::from(n)),
            Content::I16(n) => Unexpected::Signed(i64::from(n)),
            Content::I32(n) => Unexpected::Signed(i64::from(n)),
            Content::I64(n) => Unexpected::Signed(n),
            Content::F32(f) => Unexpected::Float(f64::from(f)),
            Content::F64(f) => Unexpected::Float(f),
            Content::Char(c) => Unexpected::Char(c),
            Content::String(ref s) => Unexpected::Str(s),
            Content::Str(s) => Unexpected::Str(s),
            Content::ByteBuf(ref b) => Unexpected::Bytes(b),
            Content::Bytes(b) => Unexpected::Bytes(b),
            Content::None | Content::Some(_) => Unexpected::Option,
            Content::Unit => Unexpected::Unit,
            Content::Newtype(_) => Unexpected::NewtypeStruct,
            Content::Seq(_) => Unexpected::Seq,
            Content::Map(_) => Unexpected::Map,
        }
    }
}

impl<'de> Deserialize<'de> for Content<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let visitor = ContentVisitor::new();
        deserializer.deserialize_any(visitor)
    }
}

pub struct ContentVisitor<'de> {
    value: PhantomData<Content<'de>>,
}

impl<'de> ContentVisitor<'de> {
    pub fn new() -> Self {
        ContentVisitor { value: PhantomData }
    }
}

impl<'de> Visitor<'de> for ContentVisitor<'de> {
    type Value = Content<'de>;

    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("any value")
    }

    fn visit_bool<F>(self, value: bool) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::Bool(value))
    }

    fn visit_i8<F>(self, value: i8) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::I8(value))
    }

    fn visit_i16<F>(self, value: i16) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::I16(value))
    }

    fn visit_i32<F>(self, value: i32) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::I32(value))
    }

    fn visit_i64<F>(self, value: i64) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::I64(value))
    }

    fn visit_u8<F>(self, value: u8) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::U8(value))
    }

    fn visit_u16<F>(self, value: u16) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::U16(value))
    }

    fn visit_u32<F>(self, value: u32) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::U32(value))
    }

    fn visit_u64<F>(self, value: u64) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::U64(value))
    }

    fn visit_f32<F>(self, value: f32) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::F32(value))
    }

    fn visit_f64<F>(self, value: f64) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::F64(value))
    }

    fn visit_char<F>(self, value: char) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::Char(value))
    }

    fn visit_str<F>(self, value: &str) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::String(value.into()))
    }

    fn visit_borrowed_str<F>(self, value: &'de str) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::Str(value))
    }

    fn visit_string<F>(self, value: String) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::String(value))
    }

    fn visit_bytes<F>(self, value: &[u8]) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::ByteBuf(value.into()))
    }

    fn visit_borrowed_bytes<F>(self, value: &'de [u8]) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::Bytes(value))
    }

    fn visit_byte_buf<F>(self, value: Vec<u8>) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::ByteBuf(value))
    }

    fn visit_unit<F>(self) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::Unit)
    }

    fn visit_none<F>(self) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::None)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(|v| Content::Some(Box::new(v)))
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(|v| Content::Newtype(Box::new(v)))
    }

    fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let mut vec = Vec::with_capacity(size_hint::cautious(visitor.size_hint()));
        while let Some(e) = visitor.next_element()? {
            vec.push(e);
        }
        Ok(Content::Seq(vec))
    }

    fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut vec = Vec::with_capacity(size_hint::cautious(visitor.size_hint()));
        while let Some(kv) = visitor.next_entry()? {
            vec.push(kv);
        }
        Ok(Content::Map(vec))
    }

    fn visit_enum<V>(self, _visitor: V) -> Result<Self::Value, V::Error>
    where
        V: EnumAccess<'de>,
    {
        Err(de::Error::custom(
            "untagged and internally tagged enums do not support enum input",
        ))
    }
}

pub struct ContentDeserializer<'de, E> {
    content: Content<'de>,
    err:     PhantomData<E>,
}

impl<'de, E> Deserializer<'de> for ContentDeserializer<'de, E>
where
    E: de::Error,
{
    type Error = E;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::Bool(v) => visitor.visit_bool(v),
            Content::U8(v) => visitor.visit_u8(v),
            Content::U16(v) => visitor.visit_u16(v),
            Content::U32(v) => visitor.visit_u32(v),
            Content::U64(v) => visitor.visit_u64(v),
            Content::I8(v) => visitor.visit_i8(v),
            Content::I16(v) => visitor.visit_i16(v),
            Content::I32(v) => visitor.visit_i32(v),
            Content::I64(v) => visitor.visit_i64(v),
            Content::F32(v) => visitor.visit_f32(v),
            Content::F64(v) => visitor.visit_f64(v),
            Content::Char(v) => visitor.visit_char(v),
            Content::String(v) => visitor.visit_string(v),
            Content::Str(v) => visitor.visit_borrowed_str(v),
            Content::ByteBuf(v) => visitor.visit_byte_buf(v),
            Content::Bytes(v) => visitor.visit_borrowed_bytes(v),
            Content::Unit => visitor.visit_unit(),
            Content::None => visitor.visit_none(),
            Content::Some(v) => visitor.visit_some(ContentDeserializer::new(*v)),
            Content::Newtype(v) => visitor.visit_newtype_struct(ContentDeserializer::new(*v)),
            Content::Seq(v) => {
                let seq = v.into_iter().map(ContentDeserializer::new);
                let mut seq_visitor = de::value::SeqDeserializer::new(seq);
                let value = visitor.visit_seq(&mut seq_visitor)?;
                seq_visitor.end()?;
                Ok(value)
            },
            Content::Map(v) => {
                let map = v
                    .into_iter()
                    .map(|(k, v)| (ContentDeserializer::new(k), ContentDeserializer::new(v)));
                let mut map_visitor = de::value::MapDeserializer::new(map);
                let value = visitor.visit_map(&mut map_visitor)?;
                map_visitor.end()?;
                Ok(value)
            },
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::None => visitor.visit_none(),
            Content::Some(v) => visitor.visit_some(ContentDeserializer::new(*v)),
            Content::Unit => visitor.visit_unit(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_newtype_struct<V>(self, _name: &str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_enum<V>(
        self,
        _name: &str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let (variant, value) = match self.content {
            Content::Map(value) => {
                let mut iter = value.into_iter();
                let (variant, value) = match iter.next() {
                    Some(v) => v,
                    None => {
                        return Err(de::Error::invalid_value(
                            de::Unexpected::Map,
                            &"map with a single key",
                        ));
                    },
                };
                // enums are encoded in json as maps with a single key:value pair
                if iter.next().is_some() {
                    return Err(de::Error::invalid_value(
                        de::Unexpected::Map,
                        &"map with a single key",
                    ));
                }
                (variant, Some(value))
            },
            s @ Content::String(_) | s @ Content::Str(_) => (s, None),
            other => {
                return Err(de::Error::invalid_type(
                    other.unexpected(),
                    &"string or map",
                ));
            },
        };

        visitor.visit_enum(EnumDeserializer {
            variant: variant,
            value:   value,
            err:     PhantomData,
        })
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            // As a special case, allow deserializing untagged newtype
            // variant containing unit struct.
            //
            //     #[derive(Deserialize)]
            //     struct Info;
            //
            //     #[derive(Deserialize)]
            //     #[serde(tag = "topic")]
            //     enum Message {
            //         Info(Info),
            //     }
            //
            // We want {"topic":"Info"} to deserialize even though
            // ordinarily unit structs do not deserialize from empty map.
            Content::Map(ref v) if v.is_empty() => visitor.visit_unit(),
            Content::Seq(ref v) if v.is_empty() => visitor.visit_unit(),
            _ => self.deserialize_any(visitor),
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
        byte_buf unit seq tuple tuple_struct map struct identifier
        ignored_any
    }
}

impl<'de, E> ContentDeserializer<'de, E> {
    /// private API, don't use
    pub fn new(content: Content<'de>) -> Self {
        ContentDeserializer {
            content: content,
            err:     PhantomData,
        }
    }
}

struct EnumDeserializer<'de, E>
where
    E: de::Error,
{
    variant: Content<'de>,
    value:   Option<Content<'de>>,
    err:     PhantomData<E>,
}
impl<'de, E> de::EnumAccess<'de> for EnumDeserializer<'de, E>
where
    E: de::Error,
{
    type Error = E;
    type Variant = VariantDeserializer<'de, Self::Error>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), E>
    where
        V: de::DeserializeSeed<'de>,
    {
        let visitor = VariantDeserializer {
            value: self.value,
            err:   PhantomData,
        };
        seed.deserialize(ContentDeserializer::new(self.variant))
            .map(|v| (v, visitor))
    }
}

struct VariantDeserializer<'de, E>
where
    E: de::Error,
{
    value: Option<Content<'de>>,
    err:   PhantomData<E>,
}

impl<'de, E> de::VariantAccess<'de> for VariantDeserializer<'de, E>
where
    E: de::Error,
{
    type Error = E;

    fn unit_variant(self) -> Result<(), E> {
        match self.value {
            Some(value) => de::Deserialize::deserialize(ContentDeserializer::new(value)),
            None => Ok(()),
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, E>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.value {
            Some(value) => seed.deserialize(ContentDeserializer::new(value)),
            None => Err(de::Error::invalid_type(
                de::Unexpected::UnitVariant,
                &"newtype variant",
            )),
        }
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            Some(Content::Seq(v)) => {
                de::Deserializer::deserialize_any(SeqDeserializer::new(v), visitor)
            },
            Some(other) => Err(de::Error::invalid_type(
                other.unexpected(),
                &"tuple variant",
            )),
            None => Err(de::Error::invalid_type(
                de::Unexpected::UnitVariant,
                &"tuple variant",
            )),
        }
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            Some(Content::Map(v)) => {
                de::Deserializer::deserialize_any(MapDeserializer::new(v), visitor)
            },
            Some(Content::Seq(v)) => {
                de::Deserializer::deserialize_any(SeqDeserializer::new(v), visitor)
            },
            Some(other) => Err(de::Error::invalid_type(
                other.unexpected(),
                &"struct variant",
            )),
            _ => Err(de::Error::invalid_type(
                de::Unexpected::UnitVariant,
                &"struct variant",
            )),
        }
    }
}

struct SeqDeserializer<'de, E>
where
    E: de::Error,
{
    iter: <Vec<Content<'de>> as IntoIterator>::IntoIter,
    err:  PhantomData<E>,
}

impl<'de, E> SeqDeserializer<'de, E>
where
    E: de::Error,
{
    fn new(vec: Vec<Content<'de>>) -> Self {
        SeqDeserializer {
            iter: vec.into_iter(),
            err:  PhantomData,
        }
    }
}

impl<'de, E> de::Deserializer<'de> for SeqDeserializer<'de, E>
where
    E: de::Error,
{
    type Error = E;

    #[inline]
    fn deserialize_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let len = self.iter.len();
        if len == 0 {
            visitor.visit_unit()
        } else {
            let ret = visitor.visit_seq(&mut self)?;
            let remaining = self.iter.len();
            if remaining == 0 {
                Ok(ret)
            } else {
                Err(de::Error::invalid_length(len, &"fewer elements in array"))
            }
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
        byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

impl<'de, E> de::SeqAccess<'de> for SeqDeserializer<'de, E>
where
    E: de::Error,
{
    type Error = E;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some(value) => seed.deserialize(ContentDeserializer::new(value)).map(Some),
            None => Ok(None),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        size_hint::from_bounds(&self.iter)
    }
}

struct MapDeserializer<'de, E>
where
    E: de::Error,
{
    iter:  <Vec<(Content<'de>, Content<'de>)> as IntoIterator>::IntoIter,
    value: Option<Content<'de>>,
    err:   PhantomData<E>,
}

impl<'de, E> MapDeserializer<'de, E>
where
    E: de::Error,
{
    fn new(map: Vec<(Content<'de>, Content<'de>)>) -> Self {
        MapDeserializer {
            iter:  map.into_iter(),
            value: None,
            err:   PhantomData,
        }
    }
}

impl<'de, E> de::MapAccess<'de> for MapDeserializer<'de, E>
where
    E: de::Error,
{
    type Error = E;

    fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some((key, value)) => {
                self.value = Some(value);
                seed.deserialize(ContentDeserializer::new(key)).map(Some)
            },
            None => Ok(None),
        }
    }

    fn next_value_seed<T>(&mut self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.value.take() {
            Some(value) => seed.deserialize(ContentDeserializer::new(value)),
            None => Err(de::Error::custom("value is missing")),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        size_hint::from_bounds(&self.iter)
    }
}

impl<'de, E> de::Deserializer<'de> for MapDeserializer<'de, E>
where
    E: de::Error,
{
    type Error = E;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_map(self)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
        byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}


pub struct ContentRefDeserializer<'a, 'de: 'a, E> {
    content: &'a Content<'de>,
    err:     PhantomData<E>,
}

impl<'de, 'a, E> Deserializer<'de> for ContentRefDeserializer<'a, 'de, E>
where
    E: de::Error,
{
    type Error = E;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, E>
    where
        V: Visitor<'de>,
    {
        match *self.content {
            Content::Bool(v) => visitor.visit_bool(v),
            Content::U8(v) => visitor.visit_u8(v),
            Content::U16(v) => visitor.visit_u16(v),
            Content::U32(v) => visitor.visit_u32(v),
            Content::U64(v) => visitor.visit_u64(v),
            Content::I8(v) => visitor.visit_i8(v),
            Content::I16(v) => visitor.visit_i16(v),
            Content::I32(v) => visitor.visit_i32(v),
            Content::I64(v) => visitor.visit_i64(v),
            Content::F32(v) => visitor.visit_f32(v),
            Content::F64(v) => visitor.visit_f64(v),
            Content::Char(v) => visitor.visit_char(v),
            Content::String(ref v) => visitor.visit_str(v),
            Content::Str(v) => visitor.visit_borrowed_str(v),
            Content::ByteBuf(ref v) => visitor.visit_bytes(v),
            Content::Bytes(v) => visitor.visit_borrowed_bytes(v),
            Content::Unit => visitor.visit_unit(),
            Content::None => visitor.visit_none(),
            Content::Some(ref v) => visitor.visit_some(ContentRefDeserializer::new(v)),
            Content::Newtype(ref v) => visitor.visit_newtype_struct(ContentRefDeserializer::new(v)),
            Content::Seq(ref v) => {
                let seq = v.iter().map(ContentRefDeserializer::new);
                let mut seq_visitor = de::value::SeqDeserializer::new(seq);
                let value = visitor.visit_seq(&mut seq_visitor)?;
                seq_visitor.end()?;
                Ok(value)
            },
            Content::Map(ref v) => {
                let map = v.iter().map(|(k, v)| {
                    (
                        ContentRefDeserializer::new(k),
                        ContentRefDeserializer::new(v),
                    )
                });
                let mut map_visitor = de::value::MapDeserializer::new(map);
                let value = visitor.visit_map(&mut map_visitor)?;
                map_visitor.end()?;
                Ok(value)
            },
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, E>
    where
        V: Visitor<'de>,
    {
        match *self.content {
            Content::None => visitor.visit_none(),
            Content::Some(ref v) => visitor.visit_some(ContentRefDeserializer::new(v)),
            Content::Unit => visitor.visit_unit(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_newtype_struct<V>(self, _name: &str, visitor: V) -> Result<V::Value, E>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_enum<V>(
        self,
        _name: &str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let (variant, value) = match *self.content {
            Content::Map(ref value) => {
                let mut iter = value.iter();
                let (variant, value) = match iter.next() {
                    Some(v) => v,
                    None => {
                        return Err(de::Error::invalid_value(
                            de::Unexpected::Map,
                            &"map with a single key",
                        ));
                    },
                };
                // enums are encoded in json as maps with a single key:value pair
                if iter.next().is_some() {
                    return Err(de::Error::invalid_value(
                        de::Unexpected::Map,
                        &"map with a single key",
                    ));
                }
                (variant, Some(value))
            },
            ref s @ Content::String(_) | ref s @ Content::Str(_) => (s, None),
            ref other => {
                return Err(de::Error::invalid_type(
                    other.unexpected(),
                    &"string or map",
                ));
            },
        };

        visitor.visit_enum(EnumRefDeserializer {
            variant: variant,
            value:   value,
            err:     PhantomData,
        })
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
        byte_buf unit unit_struct seq tuple tuple_struct map struct
        identifier ignored_any
    }
}

impl<'a, 'de, E> ContentRefDeserializer<'a, 'de, E> {
    pub fn new(content: &'a Content<'de>) -> Self {
        ContentRefDeserializer {
            content: content,
            err:     PhantomData,
        }
    }
}

struct EnumRefDeserializer<'a, 'de: 'a, E>
where
    E: de::Error,
{
    variant: &'a Content<'de>,
    value:   Option<&'a Content<'de>>,
    err:     PhantomData<E>,
}

impl<'de, 'a, E> de::EnumAccess<'de> for EnumRefDeserializer<'a, 'de, E>
where
    E: de::Error,
{
    type Error = E;
    type Variant = VariantRefDeserializer<'a, 'de, Self::Error>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let visitor = VariantRefDeserializer {
            value: self.value,
            err:   PhantomData,
        };
        seed.deserialize(ContentRefDeserializer::new(self.variant))
            .map(|v| (v, visitor))
    }
}

struct VariantRefDeserializer<'a, 'de: 'a, E>
where
    E: de::Error,
{
    value: Option<&'a Content<'de>>,
    err:   PhantomData<E>,
}

impl<'de, 'a, E> de::VariantAccess<'de> for VariantRefDeserializer<'a, 'de, E>
where
    E: de::Error,
{
    type Error = E;

    fn unit_variant(self) -> Result<(), E> {
        match self.value {
            Some(value) => de::Deserialize::deserialize(ContentRefDeserializer::new(value)),
            None => Ok(()),
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, E>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.value {
            Some(value) => seed.deserialize(ContentRefDeserializer::new(value)),
            None => Err(de::Error::invalid_type(
                de::Unexpected::UnitVariant,
                &"newtype variant",
            )),
        }
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            Some(Content::Seq(v)) => {
                de::Deserializer::deserialize_any(SeqRefDeserializer::new(v), visitor)
            },
            Some(other) => Err(de::Error::invalid_type(
                other.unexpected(),
                &"tuple variant",
            )),
            None => Err(de::Error::invalid_type(
                de::Unexpected::UnitVariant,
                &"tuple variant",
            )),
        }
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            Some(Content::Map(v)) => {
                de::Deserializer::deserialize_any(MapRefDeserializer::new(v), visitor)
            },
            Some(Content::Seq(v)) => {
                de::Deserializer::deserialize_any(SeqRefDeserializer::new(v), visitor)
            },
            Some(other) => Err(de::Error::invalid_type(
                other.unexpected(),
                &"struct variant",
            )),
            _ => Err(de::Error::invalid_type(
                de::Unexpected::UnitVariant,
                &"struct variant",
            )),
        }
    }
}

struct SeqRefDeserializer<'a, 'de: 'a, E>
where
    E: de::Error,
{
    iter: <&'a [Content<'de>] as IntoIterator>::IntoIter,
    err:  PhantomData<E>,
}

impl<'a, 'de, E> SeqRefDeserializer<'a, 'de, E>
where
    E: de::Error,
{
    fn new(vec: &'a [Content<'de>]) -> Self {
        SeqRefDeserializer {
            iter: vec.iter(),
            err:  PhantomData,
        }
    }
}

impl<'de, 'a, E> de::Deserializer<'de> for SeqRefDeserializer<'a, 'de, E>
where
    E: de::Error,
{
    type Error = E;

    #[inline]
    fn deserialize_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let len = self.iter.len();
        if len == 0 {
            visitor.visit_unit()
        } else {
            let ret = visitor.visit_seq(&mut self)?;
            let remaining = self.iter.len();
            if remaining == 0 {
                Ok(ret)
            } else {
                Err(de::Error::invalid_length(len, &"fewer elements in array"))
            }
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
        byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

impl<'de, 'a, E> de::SeqAccess<'de> for SeqRefDeserializer<'a, 'de, E>
where
    E: de::Error,
{
    type Error = E;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some(value) => seed
                .deserialize(ContentRefDeserializer::new(value))
                .map(Some),
            None => Ok(None),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        size_hint::from_bounds(&self.iter)
    }
}

struct MapRefDeserializer<'a, 'de: 'a, E>
where
    E: de::Error,
{
    iter:  <&'a [(Content<'de>, Content<'de>)] as IntoIterator>::IntoIter,
    value: Option<&'a Content<'de>>,
    err:   PhantomData<E>,
}

impl<'a, 'de, E> MapRefDeserializer<'a, 'de, E>
where
    E: de::Error,
{
    fn new(map: &'a [(Content<'de>, Content<'de>)]) -> Self {
        MapRefDeserializer {
            iter:  map.iter(),
            value: None,
            err:   PhantomData,
        }
    }
}

impl<'de, 'a, E> de::MapAccess<'de> for MapRefDeserializer<'a, 'de, E>
where
    E: de::Error,
{
    type Error = E;

    fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some((key, value)) => {
                self.value = Some(value);
                seed.deserialize(ContentRefDeserializer::new(key)).map(Some)
            },
            None => Ok(None),
        }
    }

    fn next_value_seed<T>(&mut self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.value.take() {
            Some(value) => seed.deserialize(ContentRefDeserializer::new(value)),
            None => Err(de::Error::custom("value is missing")),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        size_hint::from_bounds(&self.iter)
    }
}

impl<'de, 'a, E> de::Deserializer<'de> for MapRefDeserializer<'a, 'de, E>
where
    E: de::Error,
{
    type Error = E;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_map(self)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
        byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

impl<'de, E> de::IntoDeserializer<'de, E> for ContentDeserializer<'de, E>
where
    E: de::Error,
{
    type Deserializer = Self;

    fn into_deserializer(self) -> Self {
        self
    }
}

impl<'de, 'a, E> de::IntoDeserializer<'de, E> for ContentRefDeserializer<'a, 'de, E>
where
    E: de::Error,
{
    type Deserializer = Self;

    fn into_deserializer(self) -> Self {
        self
    }
}
