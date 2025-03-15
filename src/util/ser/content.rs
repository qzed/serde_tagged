// This file is adapted from `serde`, specifically the module `content` in
// https://github.com/serde-rs/serde/blob/v1.0.27/serde/src/private/ser.rs
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
// - removed types and respecitve impls for:
//   - `SerializeTupleVariantAsMapValue`
//   - `SerializeStructVariantAsMapValue`
// - changed visibility of types and constructor-functions to `pub`


use std::marker::PhantomData;

use serde::ser::{self, Serialize, Serializer};

#[derive(Debug)]
pub enum Content {
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
    Bytes(Vec<u8>),

    None,
    Some(Box<Content>),

    Unit,
    UnitStruct(&'static str),
    UnitVariant(&'static str, u32, &'static str),
    NewtypeStruct(&'static str, Box<Content>),
    NewtypeVariant(&'static str, u32, &'static str, Box<Content>),

    Seq(Vec<Content>),
    Tuple(Vec<Content>),
    TupleStruct(&'static str, Vec<Content>),
    TupleVariant(&'static str, u32, &'static str, Vec<Content>),
    Map(Vec<(Content, Content)>),
    Struct(&'static str, Vec<(&'static str, Content)>),
    StructVariant(
        &'static str,
        u32,
        &'static str,
        Vec<(&'static str, Content)>,
    ),
}

impl Serialize for Content {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Content::Bool(b) => serializer.serialize_bool(b),
            Content::U8(u) => serializer.serialize_u8(u),
            Content::U16(u) => serializer.serialize_u16(u),
            Content::U32(u) => serializer.serialize_u32(u),
            Content::U64(u) => serializer.serialize_u64(u),
            Content::I8(i) => serializer.serialize_i8(i),
            Content::I16(i) => serializer.serialize_i16(i),
            Content::I32(i) => serializer.serialize_i32(i),
            Content::I64(i) => serializer.serialize_i64(i),
            Content::F32(f) => serializer.serialize_f32(f),
            Content::F64(f) => serializer.serialize_f64(f),
            Content::Char(c) => serializer.serialize_char(c),
            Content::String(ref s) => serializer.serialize_str(s),
            Content::Bytes(ref b) => serializer.serialize_bytes(b),
            Content::None => serializer.serialize_none(),
            Content::Some(ref c) => serializer.serialize_some(&**c),
            Content::Unit => serializer.serialize_unit(),
            Content::UnitStruct(n) => serializer.serialize_unit_struct(n),
            Content::UnitVariant(n, i, v) => serializer.serialize_unit_variant(n, i, v),
            Content::NewtypeStruct(n, ref c) => serializer.serialize_newtype_struct(n, &**c),
            Content::NewtypeVariant(n, i, v, ref c) => {
                serializer.serialize_newtype_variant(n, i, v, &**c)
            },
            Content::Seq(ref elements) => elements.serialize(serializer),
            Content::Tuple(ref elements) => {
                use serde::ser::SerializeTuple;
                let mut tuple = serializer.serialize_tuple(elements.len())?;
                for e in elements {
                    tuple.serialize_element(e)?;
                }
                tuple.end()
            },
            Content::TupleStruct(n, ref fields) => {
                use serde::ser::SerializeTupleStruct;
                let mut ts = serializer.serialize_tuple_struct(n, fields.len())?;
                for f in fields {
                    ts.serialize_field(f)?;
                }
                ts.end()
            },
            Content::TupleVariant(n, i, v, ref fields) => {
                use serde::ser::SerializeTupleVariant;
                let mut tv = serializer.serialize_tuple_variant(n, i, v, fields.len())?;
                for f in fields {
                    tv.serialize_field(f)?;
                }
                tv.end()
            },
            Content::Map(ref entries) => {
                use serde::ser::SerializeMap;
                let mut map = serializer.serialize_map(Some(entries.len()))?;
                for (k, v) in entries {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            },
            Content::Struct(n, ref fields) => {
                use serde::ser::SerializeStruct;
                let mut s = serializer.serialize_struct(n, fields.len())?;
                for (k, v) in fields {
                    s.serialize_field(k, v)?;
                }
                s.end()
            },
            Content::StructVariant(n, i, v, ref fields) => {
                use serde::ser::SerializeStructVariant;
                let mut sv = serializer.serialize_struct_variant(n, i, v, fields.len())?;
                for (k, v) in fields {
                    sv.serialize_field(k, v)?;
                }
                sv.end()
            },
        }
    }
}

pub struct ContentSerializer<E> {
    error: PhantomData<E>,
}

impl<E> ContentSerializer<E> {
    pub fn new() -> Self {
        ContentSerializer { error: PhantomData }
    }
}

impl<E> Serializer for ContentSerializer<E>
where
    E: ser::Error,
{
    type Ok = Content;
    type Error = E;

    type SerializeSeq = SerializeSeq<E>;
    type SerializeTuple = SerializeTuple<E>;
    type SerializeTupleStruct = SerializeTupleStruct<E>;
    type SerializeTupleVariant = SerializeTupleVariant<E>;
    type SerializeMap = SerializeMap<E>;
    type SerializeStruct = SerializeStruct<E>;
    type SerializeStructVariant = SerializeStructVariant<E>;

    fn serialize_bool(self, v: bool) -> Result<Content, E> {
        Ok(Content::Bool(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Content, E> {
        Ok(Content::I8(v))
    }

    fn serialize_i16(self, v: i16) -> Result<Content, E> {
        Ok(Content::I16(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Content, E> {
        Ok(Content::I32(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Content, E> {
        Ok(Content::I64(v))
    }

    fn serialize_u8(self, v: u8) -> Result<Content, E> {
        Ok(Content::U8(v))
    }

    fn serialize_u16(self, v: u16) -> Result<Content, E> {
        Ok(Content::U16(v))
    }

    fn serialize_u32(self, v: u32) -> Result<Content, E> {
        Ok(Content::U32(v))
    }

    fn serialize_u64(self, v: u64) -> Result<Content, E> {
        Ok(Content::U64(v))
    }

    fn serialize_f32(self, v: f32) -> Result<Content, E> {
        Ok(Content::F32(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Content, E> {
        Ok(Content::F64(v))
    }

    fn serialize_char(self, v: char) -> Result<Content, E> {
        Ok(Content::Char(v))
    }

    fn serialize_str(self, value: &str) -> Result<Content, E> {
        Ok(Content::String(value.to_owned()))
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Content, E> {
        Ok(Content::Bytes(value.to_owned()))
    }

    fn serialize_none(self) -> Result<Content, E> {
        Ok(Content::None)
    }

    fn serialize_some<T>(self, value: &T) -> Result<Content, E>
    where
        T: Serialize + ?Sized,
    {
        Ok(Content::Some(Box::new(value.serialize(self)?)))
    }

    fn serialize_unit(self) -> Result<Content, E> {
        Ok(Content::Unit)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Content, E> {
        Ok(Content::UnitStruct(name))
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Content, E> {
        Ok(Content::UnitVariant(name, variant_index, variant))
    }

    fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<Content, E>
    where
        T: Serialize + ?Sized,
    {
        Ok(Content::NewtypeStruct(
            name,
            Box::new(value.serialize(self)?),
        ))
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Content, E>
    where
        T: Serialize + ?Sized,
    {
        Ok(Content::NewtypeVariant(
            name,
            variant_index,
            variant,
            Box::new(value.serialize(self)?),
        ))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, E> {
        Ok(SerializeSeq {
            elements: Vec::with_capacity(len.unwrap_or(0)),
            error:    PhantomData,
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, E> {
        Ok(SerializeTuple {
            elements: Vec::with_capacity(len),
            error:    PhantomData,
        })
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, E> {
        Ok(SerializeTupleStruct {
            name:   name,
            fields: Vec::with_capacity(len),
            error:  PhantomData,
        })
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, E> {
        Ok(SerializeTupleVariant {
            name:          name,
            variant_index: variant_index,
            variant:       variant,
            fields:        Vec::with_capacity(len),
            error:         PhantomData,
        })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, E> {
        Ok(SerializeMap {
            entries: Vec::with_capacity(len.unwrap_or(0)),
            key:     None,
            error:   PhantomData,
        })
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct, E> {
        Ok(SerializeStruct {
            name:   name,
            fields: Vec::with_capacity(len),
            error:  PhantomData,
        })
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, E> {
        Ok(SerializeStructVariant {
            name:          name,
            variant_index: variant_index,
            variant:       variant,
            fields:        Vec::with_capacity(len),
            error:         PhantomData,
        })
    }
}

pub struct SerializeSeq<E> {
    elements: Vec<Content>,
    error:    PhantomData<E>,
}

impl<E> ser::SerializeSeq for SerializeSeq<E>
where
    E: ser::Error,
{
    type Ok = Content;
    type Error = E;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), E>
    where
        T: Serialize + ?Sized,
    {
        let value = value.serialize(ContentSerializer::<E>::new())?;
        self.elements.push(value);
        Ok(())
    }

    fn end(self) -> Result<Content, E> {
        Ok(Content::Seq(self.elements))
    }
}

pub struct SerializeTuple<E> {
    elements: Vec<Content>,
    error:    PhantomData<E>,
}

impl<E> ser::SerializeTuple for SerializeTuple<E>
where
    E: ser::Error,
{
    type Ok = Content;
    type Error = E;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), E>
    where
        T: Serialize + ?Sized,
    {
        let value = value.serialize(ContentSerializer::<E>::new())?;
        self.elements.push(value);
        Ok(())
    }

    fn end(self) -> Result<Content, E> {
        Ok(Content::Tuple(self.elements))
    }
}

pub struct SerializeTupleStruct<E> {
    name:   &'static str,
    fields: Vec<Content>,
    error:  PhantomData<E>,
}

impl<E> ser::SerializeTupleStruct for SerializeTupleStruct<E>
where
    E: ser::Error,
{
    type Ok = Content;
    type Error = E;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), E>
    where
        T: Serialize + ?Sized,
    {
        let value = value.serialize(ContentSerializer::<E>::new())?;
        self.fields.push(value);
        Ok(())
    }

    fn end(self) -> Result<Content, E> {
        Ok(Content::TupleStruct(self.name, self.fields))
    }
}

pub struct SerializeTupleVariant<E> {
    name:          &'static str,
    variant_index: u32,
    variant:       &'static str,
    fields:        Vec<Content>,
    error:         PhantomData<E>,
}

impl<E> ser::SerializeTupleVariant for SerializeTupleVariant<E>
where
    E: ser::Error,
{
    type Ok = Content;
    type Error = E;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), E>
    where
        T: Serialize + ?Sized,
    {
        let value = value.serialize(ContentSerializer::<E>::new())?;
        self.fields.push(value);
        Ok(())
    }

    fn end(self) -> Result<Content, E> {
        Ok(Content::TupleVariant(
            self.name,
            self.variant_index,
            self.variant,
            self.fields,
        ))
    }
}

pub struct SerializeMap<E> {
    entries: Vec<(Content, Content)>,
    key:     Option<Content>,
    error:   PhantomData<E>,
}

impl<E> ser::SerializeMap for SerializeMap<E>
where
    E: ser::Error,
{
    type Ok = Content;
    type Error = E;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), E>
    where
        T: Serialize + ?Sized,
    {
        let key = key.serialize(ContentSerializer::<E>::new())?;
        self.key = Some(key);
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), E>
    where
        T: Serialize + ?Sized,
    {
        let key = self
            .key
            .take()
            .expect("serialize_value called before serialize_key");
        let value = value.serialize(ContentSerializer::<E>::new())?;
        self.entries.push((key, value));
        Ok(())
    }

    fn end(self) -> Result<Content, E> {
        Ok(Content::Map(self.entries))
    }

    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), E>
    where
        K: Serialize + ?Sized,
        V: Serialize + ?Sized,
    {
        let key = key.serialize(ContentSerializer::<E>::new())?;
        let value = value.serialize(ContentSerializer::<E>::new())?;
        self.entries.push((key, value));
        Ok(())
    }
}

pub struct SerializeStruct<E> {
    name:   &'static str,
    fields: Vec<(&'static str, Content)>,
    error:  PhantomData<E>,
}

impl<E> ser::SerializeStruct for SerializeStruct<E>
where
    E: ser::Error,
{
    type Ok = Content;
    type Error = E;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), E>
    where
        T: Serialize + ?Sized,
    {
        let value = value.serialize(ContentSerializer::<E>::new())?;
        self.fields.push((key, value));
        Ok(())
    }

    fn end(self) -> Result<Content, E> {
        Ok(Content::Struct(self.name, self.fields))
    }
}

pub struct SerializeStructVariant<E> {
    name:          &'static str,
    variant_index: u32,
    variant:       &'static str,
    fields:        Vec<(&'static str, Content)>,
    error:         PhantomData<E>,
}

impl<E> ser::SerializeStructVariant for SerializeStructVariant<E>
where
    E: ser::Error,
{
    type Ok = Content;
    type Error = E;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), E>
    where
        T: Serialize + ?Sized,
    {
        let value = value.serialize(ContentSerializer::<E>::new())?;
        self.fields.push((key, value));
        Ok(())
    }

    fn end(self) -> Result<Content, E> {
        Ok(Content::StructVariant(
            self.name,
            self.variant_index,
            self.variant,
            self.fields,
        ))
    }
}
