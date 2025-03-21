//! Serialization of adjacently tagged values using structs.
//!
//! Tagging a value adjacently using this strategy will create a struct with two
//! fields, where the first field will be the tag and the second field the
//! value.
//!
//! The representation of this tagging format depends largely on the
//! underlying data format, e.g. in JSON, this is equivalent to the map-based
//! adjacent tagging format, whereas with msgpack, it would be similar to the
//! tuple-based format.
//!
//! # Warning
//!
//! If the deserialization-process depends on the tag (i.e. with
//! [`deserialize`](crate::de::adj::struc::deserialize) and/or
//! [`Visitor`](crate::de::adj::struc::Visitor)), deserialization of
//! struct-based adjacently tagged values is only supported for self-describing
//! formats.
//!
//! # Examples serializing to JSON
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
//! serde_tagged::ser::adj::struc::serialize(
//!     &mut serializer,
//!     "Tagged",
//!     "t", "bar",
//!     "c", &foo,
//! ).unwrap();
//! # }
//! ```
//!
//! with a struct-name of `"Tagged"`, a tag-key of `"t"`, a tag value of
//! `"bar"`, and a value-key of `"c"` will produce
//!
//! ```json
//! {
//!     "t": "bar",
//!     "c": 42
//! }
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
//! serde_tagged::ser::adj::struc::serialize(
//!     &mut serializer,
//!     "Tagged",
//!     "t", "my-tag",
//!     "c", &foo,
//! ).unwrap();
//! # }
//! ```
//!
//! with a struct-name of `"Tagged"`, a tag-key of `"t"`, a tag value of
//! `"my-tag"`, and a value-key of `"c"` will produce
//!
//! ```json
//! {
//!     "t": "my-tag",
//!     "c": { "bar": "baz" }
//! }
//! ```

use std::fmt::Display;

use serde;

use crate::ser::HasDelegate;
use crate::util::ser::content::{Content, ContentSerializer};
use crate::util::ser::forward;


/// Serializes the specified tag-key, tag, value-key and value as struct.
///
/// The specified parameters will be serialized as a struct with with the given
/// name containing two fields. The first field is named according to the given
/// tag-key and contains the tag, the second field is named according to the
/// value-key and contains the given value. The specified serializer performs
/// the actual serialization and thus controls the data format. For more
/// information on this tag-format, see the [module
/// documentation](crate::ser::adj::struc).
///
/// # Note
///
/// You should prefer this method to the [`Serializer`].
pub fn serialize<S, T, V>(
    serializer: S,
    name: &'static str,
    tag_key: &'static str,
    tag: &T,
    value_key: &'static str,
    value: &V,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
    T: serde::Serialize + ?Sized,
    V: serde::Serialize + ?Sized,
{
    use serde::Serialize;

    let tagged = Tagged {
        name,
        tag_key,
        tag,
        value_key,
        value,
    };
    tagged.serialize(serializer)
}

struct Tagged<'a, T, V>
where
    T: ?Sized + 'a,
    V: ?Sized + 'a,
{
    name:      &'static str,
    tag_key:   &'static str,
    tag:       &'a T,
    value_key: &'static str,
    value:     &'a V,
}

impl<'a, T, V> serde::Serialize for Tagged<'a, T, V>
where
    T: serde::Serialize + ?Sized,
    V: serde::Serialize + ?Sized,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct(self.name, 2)?;
        state.serialize_field(self.tag_key, self.tag)?;
        state.serialize_field(self.value_key, self.value)?;
        state.end()
    }
}


/// A serializer that Serializes the specified tag-key, tag, value-key and value
/// as struct.
///
/// The specified parameters will be serialized as a struct with with the given
/// name containing two fields. The first field is named according to the given
/// tag-key and contains the tag, the second field is named according to the
/// value-key and contains the given value. The specified serializer performs
/// the actual serialization and thus controls the data format. For more
/// information on this tag-format, see the [module
/// documentation](crate::ser::adj::struc).
///
/// # Warning
///
/// You should prefer the [`serialize`] function over this serializer
/// implementation. To serialize struct-entries, the serializer implementation
/// may need to allocate memory on the heap. This can be avoided in the
/// [`serialize`] function.
pub struct Serializer<'a, S, T>
where
    T: ?Sized + 'a,
{
    delegate:  S,
    name:      &'static str,
    tag_key:   &'static str,
    tag:       &'a T,
    value_key: &'static str,
}

impl<'a, S, T> Serializer<'a, S, T>
where
    S: serde::Serializer,
    T: serde::Serialize + ?Sized + 'a,
{
    pub fn new(
        delegate: S,
        name: &'static str,
        tag_key: &'static str,
        tag: &'a T,
        value_key: &'static str,
    ) -> Self {
        Serializer {
            delegate,
            name,
            tag_key,
            tag,
            value_key,
        }
    }

    fn serialize_as_struct_field<V>(self, value: &V) -> Result<S::Ok, S::Error>
    where
        V: serde::Serialize + ?Sized,
    {
        use serde::ser::SerializeStruct;

        let mut state = self.delegate.serialize_struct(self.name, 2)?;
        state.serialize_field(self.tag_key, self.tag)?;
        state.serialize_field(self.value_key, value)?;
        state.end()
    }
}

impl<'a, S, T> HasDelegate for Serializer<'a, S, T>
where
    S: serde::Serializer,
    T: serde::Serialize + ?Sized,
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

    type SerializeSeq = SerializeSeqAsStructField<S::SerializeStruct>;
    type SerializeTuple = SerializeTupleAsStructField<S::SerializeStruct>;
    type SerializeTupleStruct = SerializeTupleStructAsStructField<S::SerializeStruct>;
    type SerializeMap = SerializeMapAsStructField<S::SerializeStruct>;
    type SerializeStruct = SerializeStructAsStructField<S::SerializeStruct>;
    type SerializeTupleVariant = SerializeTupleVariantAsStructField<S::SerializeStruct>;
    type SerializeStructVariant = SerializeStructVariantAsStructField<S::SerializeStruct>;

    fn serialize_bool(self, value: bool) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_struct_field(&value)
    }

    fn serialize_i8(self, value: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_struct_field(&value)
    }

    fn serialize_i16(self, value: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_struct_field(&value)
    }

    fn serialize_i32(self, value: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_struct_field(&value)
    }

    fn serialize_i64(self, value: i64) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_struct_field(&value)
    }

    fn serialize_u8(self, value: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_struct_field(&value)
    }

    fn serialize_u16(self, value: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_struct_field(&value)
    }

    fn serialize_u32(self, value: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_struct_field(&value)
    }

    fn serialize_u64(self, value: u64) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_struct_field(&value)
    }

    fn serialize_f32(self, value: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_struct_field(&value)
    }

    fn serialize_f64(self, value: f64) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_struct_field(&value)
    }

    fn serialize_char(self, value: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_struct_field(&value)
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_struct_field(value)
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_struct_field(&forward::Bytes(value))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_struct_field(&forward::None)
    }

    fn serialize_some<V>(self, value: &V) -> Result<Self::Ok, Self::Error>
    where
        V: serde::Serialize + ?Sized,
    {
        self.serialize_as_struct_field(&forward::Some(value))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_struct_field(&forward::Unit)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_struct_field(&forward::UnitStruct(name))
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_struct_field(&forward::UnitVariant(name, variant_index, variant))
    }

    fn serialize_newtype_struct<V>(
        self,
        name: &'static str,
        value: &V,
    ) -> Result<Self::Ok, Self::Error>
    where
        V: serde::Serialize + ?Sized,
    {
        self.serialize_as_struct_field(&forward::NewtypeStruct(name, value))
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
        self.serialize_as_struct_field(&forward::NewtypeVariant(
            name,
            variant_index,
            variant,
            value,
        ))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        use serde::ser::SerializeStruct;

        let mut state = self.delegate.serialize_struct(self.name, 2)?;
        state.serialize_field(self.tag_key, self.tag)?;

        Ok(SerializeSeqAsStructField::new(state, self.value_key, len))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        use serde::ser::SerializeStruct;

        let mut state = self.delegate.serialize_struct(self.name, 2)?;
        state.serialize_field(self.tag_key, self.tag)?;

        Ok(SerializeTupleAsStructField::new(state, self.value_key, len))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        use serde::ser::SerializeStruct;

        let mut state = self.delegate.serialize_struct(self.name, 2)?;
        state.serialize_field(self.tag_key, self.tag)?;

        Ok(SerializeTupleStructAsStructField::new(
            state,
            self.value_key,
            name,
            len,
        ))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        use serde::ser::SerializeStruct;

        let mut state = self.delegate.serialize_struct(self.name, 2)?;
        state.serialize_field(self.tag_key, self.tag)?;

        Ok(SerializeTupleVariantAsStructField::new(
            state,
            self.value_key,
            name,
            variant_index,
            variant,
            len,
        ))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        use serde::ser::SerializeStruct;

        let mut state = self.delegate.serialize_struct(self.name, 2)?;
        state.serialize_field(self.tag_key, self.tag)?;

        Ok(SerializeMapAsStructField::new(state, self.value_key, len))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        use serde::ser::SerializeStruct;

        let mut state = self.delegate.serialize_struct(self.name, 2)?;
        state.serialize_field(self.tag_key, self.tag)?;

        Ok(SerializeStructAsStructField::new(
            state,
            self.value_key,
            name,
            len,
        ))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        use serde::ser::SerializeStruct;

        let mut state = self.delegate.serialize_struct(self.name, 2)?;
        state.serialize_field(self.tag_key, self.tag)?;

        Ok(SerializeStructVariantAsStructField::new(
            state,
            self.value_key,
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
        self.serialize_as_struct_field(&forward::CollectSeq::new(iter))
    }

    fn collect_map<K, V, I>(self, iter: I) -> Result<Self::Ok, Self::Error>
    where
        K: serde::Serialize,
        V: serde::Serialize,
        I: IntoIterator<Item = (K, V)>,
    {
        self.serialize_as_struct_field(&forward::CollectMap::new(iter))
    }

    fn collect_str<V>(self, value: &V) -> Result<Self::Ok, Self::Error>
    where
        V: Display + ?Sized,
    {
        self.serialize_as_struct_field(&forward::CollectStr(value))
    }

    fn is_human_readable(&self) -> bool {
        self.delegate.is_human_readable()
    }
}


#[doc(hidden)]
pub struct SerializeSeqAsStructField<S> {
    state:     S,
    value_key: &'static str,
    elements:  Vec<Content>,
}

impl<S> SerializeSeqAsStructField<S> {
    fn new(state: S, value_key: &'static str, len: Option<usize>) -> Self {
        let elements = match len {
            Some(len) => Vec::with_capacity(len),
            None => Vec::new(),
        };

        SerializeSeqAsStructField {
            state,
            value_key,
            elements,
        }
    }
}

impl<S> serde::ser::SerializeSeq for SerializeSeqAsStructField<S>
where
    S: serde::ser::SerializeStruct,
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
        self.state
            .serialize_field(self.value_key, &Content::Seq(self.elements))?;
        self.state.end()
    }
}


#[doc(hidden)]
pub struct SerializeTupleAsStructField<S> {
    state:     S,
    value_key: &'static str,
    elements:  Vec<Content>,
}

impl<S> SerializeTupleAsStructField<S> {
    fn new(state: S, value_key: &'static str, len: usize) -> Self {
        SerializeTupleAsStructField {
            state:     state,
            value_key: value_key,
            elements:  Vec::with_capacity(len),
        }
    }
}

impl<S> serde::ser::SerializeTuple for SerializeTupleAsStructField<S>
where
    S: serde::ser::SerializeStruct,
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
        self.state.serialize_field(self.value_key, &value)?;
        self.state.end()
    }
}


#[doc(hidden)]
pub struct SerializeTupleStructAsStructField<S> {
    state:     S,
    value_key: &'static str,
    name:      &'static str,
    elements:  Vec<Content>,
}

impl<S> SerializeTupleStructAsStructField<S> {
    fn new(state: S, value_key: &'static str, name: &'static str, len: usize) -> Self {
        SerializeTupleStructAsStructField {
            state:     state,
            value_key: value_key,
            name:      name,
            elements:  Vec::with_capacity(len),
        }
    }
}

impl<S> serde::ser::SerializeTupleStruct for SerializeTupleStructAsStructField<S>
where
    S: serde::ser::SerializeStruct,
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
        self.state.serialize_field(self.value_key, &value)?;
        self.state.end()
    }
}


#[doc(hidden)]
pub struct SerializeTupleVariantAsStructField<S> {
    state:         S,
    value_key:     &'static str,
    name:          &'static str,
    variant_index: u32,
    variant:       &'static str,
    elements:      Vec<Content>,
}

impl<S> SerializeTupleVariantAsStructField<S> {
    fn new(
        state: S,
        value_key: &'static str,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Self {
        SerializeTupleVariantAsStructField {
            state:         state,
            value_key:     value_key,
            name:          name,
            variant_index: variant_index,
            variant:       variant,
            elements:      Vec::with_capacity(len),
        }
    }
}

impl<S> serde::ser::SerializeTupleVariant for SerializeTupleVariantAsStructField<S>
where
    S: serde::ser::SerializeStruct,
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

        self.state.serialize_field(self.value_key, &value)?;
        self.state.end()
    }
}


#[doc(hidden)]
pub struct SerializeMapAsStructField<S> {
    state:     S,
    value_key: &'static str,
    elements:  Vec<(Content, Content)>,
}

impl<S> SerializeMapAsStructField<S> {
    fn new(state: S, value_key: &'static str, len: Option<usize>) -> Self {
        let elements = match len {
            Some(len) => Vec::with_capacity(len),
            None => Vec::new(),
        };

        SerializeMapAsStructField {
            elements,
            value_key,
            state,
        }
    }
}

impl<S> serde::ser::SerializeMap for SerializeMapAsStructField<S>
where
    S: serde::ser::SerializeStruct,
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
        self.state
            .serialize_field(self.value_key, &Content::Map(self.elements))?;
        self.state.end()
    }
}


#[doc(hidden)]
pub struct SerializeStructAsStructField<S> {
    state:     S,
    value_key: &'static str,
    name:      &'static str,
    fields:    Vec<(&'static str, Content)>,
}

impl<S> SerializeStructAsStructField<S> {
    fn new(state: S, value_key: &'static str, name: &'static str, len: usize) -> Self {
        SerializeStructAsStructField {
            state:     state,
            value_key: value_key,
            name:      name,
            fields:    Vec::with_capacity(len),
        }
    }
}

impl<S> serde::ser::SerializeStruct for SerializeStructAsStructField<S>
where
    S: serde::ser::SerializeStruct,
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
        self.state.serialize_field(self.value_key, &value)?;
        self.state.end()
    }
}


#[doc(hidden)]
pub struct SerializeStructVariantAsStructField<S> {
    state:         S,
    value_key:     &'static str,
    name:          &'static str,
    variant_index: u32,
    variant:       &'static str,
    fields:        Vec<(&'static str, Content)>,
}

impl<S> SerializeStructVariantAsStructField<S> {
    fn new(
        state: S,
        value_key: &'static str,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Self {
        SerializeStructVariantAsStructField {
            state:         state,
            value_key:     value_key,
            name:          name,
            variant_index: variant_index,
            variant:       variant,
            fields:        Vec::with_capacity(len),
        }
    }
}

impl<S> serde::ser::SerializeStructVariant for SerializeStructVariantAsStructField<S>
where
    S: serde::ser::SerializeStruct,
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
        self.state.serialize_field(self.value_key, &value)?;
        self.state.end()
    }
}
