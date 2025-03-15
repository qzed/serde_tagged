//! Serialization of externally tagged values.
//!
//! Tagging values externally will apply the tag via a mapping from tag to
//! value, thus serializing a tagged value with the functionality provided in
//! this module will always result in a map-object with a single entry.
//!
//! This format is similar to the externally-tagged enum format provided by
//! serde, however allows for various tag types (not only `str` and `u32`).
//!
//! # Examples serializing to JSON
//!
//! ## A primitive
//!
//! Serializing a value
//!
//! ```
//! # extern crate serde_json;
//! # extern crate serde_tagged;
//! #
//! # fn main() {
//! let foo: i32 = 42;
//!
//! let mut serializer = serde_json::Serializer::new(std::io::stdout());
//! serde_tagged::ser::external::serialize(&mut serializer, "bar", &foo).unwrap();
//! # }
//! ```
//!
//! with a tag value of `"bar"` will produce
//!
//! ```json
//! { "bar": 42 }
//! ```
//!
//! ## A Simple struct
//!
//! Serializing a value `foo` with
//!
//! ```
//! # #[macro_use]
//! # extern crate serde_derive;
//! # extern crate serde_json;
//! # extern crate serde_tagged;
//! #
//! #[derive(Serialize)]
//! struct Foo {
//!     bar: &'static str,
//! }
//!
//! # fn main() {
//! let foo = Foo { bar: "baz" };
//!
//! let mut serializer = serde_json::Serializer::new(std::io::stdout());
//! serde_tagged::ser::external::serialize(&mut serializer, "my-tag", &foo).unwrap();
//! # }
//! ```
//!
//! with a tag-value of `"my-tag"` will produce the following JSON output:
//!
//! ```json
//! { "my-tag": { "bar": "baz" } }
//! ```
//!

use std::fmt::Display;

use serde;

use crate::ser::HasDelegate;
use crate::util::ser::content::{Content, ContentSerializer};
use crate::util::ser::forward;

/// Applies a tag externally to the specified value and serializes them using
/// the provided serializer.
///
/// The tag-value pair will be serialized as a map with one entry, where the tag
/// will be the key. The specified serializer performs the actual serialization
/// and thus controls the data format. For more information on this tag-format,
/// see the [module documentation](::ser::external).
///
/// # Note
///
/// You should prefer this method to the [`Serializer`](Serializer).
pub fn serialize<S, T, V>(serializer: S, tag: &T, value: &V) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
    T: serde::Serialize + ?Sized,
    V: serde::Serialize + ?Sized,
{
    use serde::Serialize;

    let tagged = Tagged { tag, value };
    tagged.serialize(serializer)
}

struct Tagged<'a, V, T>
where
    V: ?Sized + 'a,
    T: ?Sized + 'a,
{
    tag:   &'a T,
    value: &'a V,
}

impl<'a, V, T> serde::Serialize for Tagged<'a, V, T>
where
    T: serde::Serialize + ?Sized,
    V: serde::Serialize + ?Sized,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;

        let mut state = serializer.serialize_map(Some(1))?;
        state.serialize_entry(self.tag, self.value)?;
        state.end()
    }
}


/// A serializer that applies a tag externally to the provided value and
/// serializes them as key-value pair.
///
/// The tag-value pair will be serialized as a map with one entry, where the tag
/// will be the key. This serializer will simply call the appropriate functions
/// on the underlying `Serializer` that is responsible for the data format. For
/// more information on this tag-format, see the
/// [module documentation](::ser::external).
///
/// # Warning
///
/// You should prefer the [`serialize`](serialize) function over this serializer
/// implementation. To serialize key-value pairs, the serializer implementation
/// may need to allocate memory on the heap. This can be avoided in the
/// [`serialize`](serialize) function.
pub struct Serializer<'a, S, T>
where
    T: ?Sized + 'a,
{
    delegate: S,
    tag:      &'a T,
}

impl<'a, S, T> Serializer<'a, S, T>
where
    S: serde::Serializer,
    T: serde::Serialize + ?Sized + 'a,
{
    /// Creates a new Serializer with the specified tag and underlying
    /// serializer.
    pub fn new(delegate: S, tag: &'a T) -> Self {
        Serializer { delegate, tag }
    }

    fn serialize_as_map_value<V>(self, value: &V) -> Result<S::Ok, S::Error>
    where
        V: serde::Serialize + ?Sized,
    {
        use serde::ser::SerializeMap;

        let mut state = self.delegate.serialize_map(Some(1))?;
        state.serialize_entry(self.tag, value)?;
        state.end()
    }
}

impl<'a, S, T> HasDelegate for Serializer<'a, S, T>
where
    S: serde::Serializer,
    T: serde::Serialize + ?Sized + 'a,
{
    type Ok = S::Ok;
    type Error = S::Error;
    type Delegate = S;

    fn delegate(self) -> S {
        self.delegate
    }
}

impl<'a, S, T> serde::Serializer for Serializer<'a, S, T>
where
    S: serde::Serializer,
    T: serde::Serialize + ?Sized + 'a,
{
    type Ok = S::Ok;
    type Error = S::Error;

    type SerializeSeq = SerializeSeqAsMapValue<S::SerializeMap>;
    type SerializeTuple = SerializeTupleAsMapValue<S::SerializeMap>;
    type SerializeTupleStruct = SerializeTupleStructAsMapValue<S::SerializeMap>;
    type SerializeMap = SerializeMapAsMapValue<S::SerializeMap>;
    type SerializeStruct = SerializeStructAsMapValue<S::SerializeMap>;
    type SerializeTupleVariant = SerializeTupleVariantAsMapValue<S::SerializeMap>;
    type SerializeStructVariant = SerializeStructVariantAsMapValue<S::SerializeMap>;

    fn serialize_bool(self, value: bool) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_map_value(&value)
    }

    fn serialize_i8(self, value: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_map_value(&value)
    }

    fn serialize_i16(self, value: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_map_value(&value)
    }

    fn serialize_i32(self, value: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_map_value(&value)
    }

    fn serialize_i64(self, value: i64) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_map_value(&value)
    }

    fn serialize_u8(self, value: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_map_value(&value)
    }

    fn serialize_u16(self, value: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_map_value(&value)
    }

    fn serialize_u32(self, value: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_map_value(&value)
    }

    fn serialize_u64(self, value: u64) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_map_value(&value)
    }

    fn serialize_f32(self, value: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_map_value(&value)
    }

    fn serialize_f64(self, value: f64) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_map_value(&value)
    }

    fn serialize_char(self, value: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_map_value(&value)
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_map_value(value)
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_map_value(&forward::Bytes(value))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_map_value(&forward::None)
    }

    fn serialize_some<V>(self, value: &V) -> Result<Self::Ok, Self::Error>
    where
        V: serde::Serialize + ?Sized,
    {
        self.serialize_as_map_value(&forward::Some(value))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_map_value(&forward::Unit)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_map_value(&forward::UnitStruct(name))
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_map_value(&forward::UnitVariant(name, variant_index, variant))
    }

    fn serialize_newtype_struct<V>(
        self,
        name: &'static str,
        value: &V,
    ) -> Result<Self::Ok, Self::Error>
    where
        V: serde::Serialize + ?Sized,
    {
        self.serialize_as_map_value(&forward::NewtypeStruct(name, value))
    }

    fn serialize_newtype_variant<V>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &V,
    ) -> Result<Self::Ok, Self::Error>
    where
        V: serde::Serialize + ?Sized,
    {
        self.serialize_as_map_value(&forward::NewtypeVariant(
            name,
            variant_index,
            variant,
            value,
        ))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        use serde::ser::SerializeMap;

        let mut state = self.delegate.serialize_map(Some(1))?;
        state.serialize_key(self.tag)?;

        Ok(SerializeSeqAsMapValue::new(state, len))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        use serde::ser::SerializeMap;

        let mut state = self.delegate.serialize_map(Some(1))?;
        state.serialize_key(self.tag)?;

        Ok(SerializeTupleAsMapValue::new(state, len))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        use serde::ser::SerializeMap;

        let mut state = self.delegate.serialize_map(Some(1))?;
        state.serialize_key(self.tag)?;

        Ok(SerializeTupleStructAsMapValue::new(state, name, len))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        use serde::ser::SerializeMap;

        let mut state = self.delegate.serialize_map(Some(1))?;
        state.serialize_key(self.tag)?;

        Ok(SerializeTupleVariantAsMapValue::new(
            state,
            name,
            variant_index,
            variant,
            len,
        ))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        use serde::ser::SerializeMap;

        let mut state = self.delegate.serialize_map(Some(1))?;
        state.serialize_key(self.tag)?;

        Ok(SerializeMapAsMapValue::new(state, len))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        use serde::ser::SerializeMap;

        let mut state = self.delegate.serialize_map(Some(1))?;
        state.serialize_key(self.tag)?;

        Ok(SerializeStructAsMapValue::new(state, name, len))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        use serde::ser::SerializeMap;

        let mut state = self.delegate.serialize_map(Some(1))?;
        state.serialize_key(self.tag)?;

        Ok(SerializeStructVariantAsMapValue::new(
            state,
            name,
            variant_index,
            variant,
            len,
        ))
    }

    fn collect_seq<I>(self, iter: I) -> Result<Self::Ok, Self::Error>
    where
        I: IntoIterator,
        <I as IntoIterator>::Item: serde::Serialize,
    {
        self.serialize_as_map_value(&forward::CollectSeq::new(iter))
    }

    fn collect_map<K, V, I>(self, iter: I) -> Result<Self::Ok, Self::Error>
    where
        K: serde::Serialize,
        V: serde::Serialize,
        I: IntoIterator<Item = (K, V)>,
    {
        self.serialize_as_map_value(&forward::CollectMap::new(iter))
    }

    fn collect_str<V>(self, value: &V) -> Result<Self::Ok, Self::Error>
    where
        V: Display + ?Sized,
    {
        self.serialize_as_map_value(&forward::CollectStr(value))
    }

    fn is_human_readable(&self) -> bool {
        self.delegate.is_human_readable()
    }
}


/// Implementation of `SerializeSeq` to capture the sequence and then serialize
/// it as map value.
#[doc(hidden)]
pub struct SerializeSeqAsMapValue<S> {
    map:      S,
    elements: Vec<Content>,
}

impl<S> SerializeSeqAsMapValue<S> {
    fn new(map: S, len: Option<usize>) -> Self {
        let elements = match len {
            Some(len) => Vec::with_capacity(len),
            None => Vec::new(),
        };

        SerializeSeqAsMapValue { map, elements }
    }
}

impl<S> serde::ser::SerializeSeq for SerializeSeqAsMapValue<S>
where
    S: serde::ser::SerializeMap,
{
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        let value = value.serialize(ContentSerializer::<S::Error>::new())?;
        self.elements.push(value);
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.map.serialize_value(&Content::Seq(self.elements))?;
        self.map.end()
    }
}


/// Implementation of `SerializeTuple` to capture the tuple and then serialize
/// it as map value.
#[doc(hidden)]
pub struct SerializeTupleAsMapValue<S> {
    map:      S,
    elements: Vec<Content>,
}

impl<S> SerializeTupleAsMapValue<S> {
    fn new(map: S, len: usize) -> Self {
        SerializeTupleAsMapValue {
            map:      map,
            elements: Vec::with_capacity(len),
        }
    }
}

impl<S> serde::ser::SerializeTuple for SerializeTupleAsMapValue<S>
where
    S: serde::ser::SerializeMap,
{
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        let value = value.serialize(ContentSerializer::<S::Error>::new())?;
        self.elements.push(value);
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let value = Content::Tuple(self.elements);
        self.map.serialize_value(&value)?;
        self.map.end()
    }
}


/// Implementation of `SerializeTupleStruct` to capture the tuple-struct and
/// then serialize it as map value.
#[doc(hidden)]
pub struct SerializeTupleStructAsMapValue<S> {
    map:      S,
    name:     &'static str,
    elements: Vec<Content>,
}

impl<S> SerializeTupleStructAsMapValue<S> {
    fn new(map: S, name: &'static str, len: usize) -> Self {
        SerializeTupleStructAsMapValue {
            map:      map,
            name:     name,
            elements: Vec::with_capacity(len),
        }
    }
}

impl<S> serde::ser::SerializeTupleStruct for SerializeTupleStructAsMapValue<S>
where
    S: serde::ser::SerializeMap,
{
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        let value = value.serialize(ContentSerializer::<S::Error>::new())?;
        self.elements.push(value);
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let value = Content::TupleStruct(self.name, self.elements);
        self.map.serialize_value(&value)?;
        self.map.end()
    }
}


/// Implementation of `SerializeTupleVariant` to capture the tuple-variant and
/// then serialize it as map value.
#[doc(hidden)]
pub struct SerializeTupleVariantAsMapValue<S> {
    map:           S,
    name:          &'static str,
    variant_index: u32,
    variant:       &'static str,
    elements:      Vec<Content>,
}

impl<S> SerializeTupleVariantAsMapValue<S> {
    fn new(
        map: S,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Self {
        SerializeTupleVariantAsMapValue {
            map:           map,
            name:          name,
            variant_index: variant_index,
            variant:       variant,
            elements:      Vec::with_capacity(len),
        }
    }
}

impl<S> serde::ser::SerializeTupleVariant for SerializeTupleVariantAsMapValue<S>
where
    S: serde::ser::SerializeMap,
{
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        let value = value.serialize(ContentSerializer::<S::Error>::new())?;
        self.elements.push(value);
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let value =
            Content::TupleVariant(self.name, self.variant_index, self.variant, self.elements);

        self.map.serialize_value(&value)?;
        self.map.end()
    }
}


/// Implementation of `SerializeMap` to capture the map and then serialize it as
/// map value.
#[doc(hidden)]
pub struct SerializeMapAsMapValue<S> {
    map:      S,
    elements: Vec<(Content, Content)>,
}

impl<S> SerializeMapAsMapValue<S> {
    fn new(map: S, len: Option<usize>) -> Self {
        let elements = match len {
            Some(len) => Vec::with_capacity(len),
            None => Vec::new(),
        };

        SerializeMapAsMapValue { elements, map }
    }
}

impl<S> serde::ser::SerializeMap for SerializeMapAsMapValue<S>
where
    S: serde::ser::SerializeMap,
{
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize + ?Sized,
    {
        let key = key.serialize(ContentSerializer::<S::Error>::new())?;
        self.elements.push((key, Content::None));
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize + ?Sized,
    {
        let value = value.serialize(ContentSerializer::<S::Error>::new())?;
        self.elements.last_mut().unwrap().1 = value;
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.map.serialize_value(&Content::Map(self.elements))?;
        self.map.end()
    }
}


/// Implementation of `SerializeMap` to capture the struct and then serialize it
/// as map value.
#[doc(hidden)]
pub struct SerializeStructAsMapValue<S> {
    map:    S,
    name:   &'static str,
    fields: Vec<(&'static str, Content)>,
}

impl<S> SerializeStructAsMapValue<S> {
    fn new(map: S, name: &'static str, len: usize) -> Self {
        SerializeStructAsMapValue {
            map:    map,
            name:   name,
            fields: Vec::with_capacity(len),
        }
    }
}

impl<S> serde::ser::SerializeStruct for SerializeStructAsMapValue<S>
where
    S: serde::ser::SerializeMap,
{
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_field<T>(&mut self, name: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize + ?Sized,
    {
        let value = value.serialize(ContentSerializer::<S::Error>::new())?;
        self.fields.push((name, value));
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let value = Content::Struct(self.name, self.fields);
        self.map.serialize_value(&value)?;
        self.map.end()
    }
}


/// Implementation of `SerializeMap` to capture the struct-variant and then
/// serialize it as map value.
#[doc(hidden)]
pub struct SerializeStructVariantAsMapValue<S> {
    map:           S,
    name:          &'static str,
    variant_index: u32,
    variant:       &'static str,
    fields:        Vec<(&'static str, Content)>,
}

impl<S> SerializeStructVariantAsMapValue<S> {
    fn new(
        map: S,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Self {
        SerializeStructVariantAsMapValue {
            map:           map,
            name:          name,
            variant_index: variant_index,
            variant:       variant,
            fields:        Vec::with_capacity(len),
        }
    }
}

impl<S> serde::ser::SerializeStructVariant for SerializeStructVariantAsMapValue<S>
where
    S: serde::ser::SerializeMap,
{
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_field<T>(&mut self, name: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize + ?Sized,
    {
        let value = value.serialize(ContentSerializer::<S::Error>::new())?;
        self.fields.push((name, value));
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let value =
            Content::StructVariant(self.name, self.variant_index, self.variant, self.fields);
        self.map.serialize_value(&value)?;
        self.map.end()
    }
}
