//! Serialization of adjacently tagged values using tuples.
//!
//! Tagging a value adjacently using this strategy will create a tuple with
//! two elements, where the first element will be the tag and the second
//! element the value.
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
//! serde_tagged::ser::adj::tuple::serialize(&mut serializer, "bar", &foo).unwrap();
//! # }
//! ```
//!
//! with a tag value of `"bar"` will produce
//!
//! ```json
//! [ "bar", 42 ]
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
//! serde_tagged::ser::adj::tuple::serialize(&mut serializer, "my-tag", &foo).unwrap();
//! # }
//! ```
//!
//! with a tag-value of `"my-tag"` will produce the following JSON output:
//!
//! ```json
//! [ "my-tag", { "bar": "baz" } ]
//! ```
//!

use std::fmt::Display;

use serde;

use crate::ser::HasDelegate;
use crate::util::ser::content::{Content, ContentSerializer};
use crate::util::ser::forward;


/// Serializes the specified tag and value as tuple.
///
/// The tag-value pair will be serialized as tuple where the first element will
/// be the tag and the second element the tuple. The specified serializer
/// performs the actual serialization and thus controls the data format. For
/// more information on this tag-format, see the [module
/// documentation](::ser::adj::tuple).
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

struct Tagged<'a, T, V>
where
    T: ?Sized + 'a,
    V: ?Sized + 'a,
{
    tag:   &'a T,
    value: &'a V,
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
        use serde::ser::SerializeTuple;

        let mut state = serializer.serialize_tuple(2)?;
        state.serialize_element(self.tag)?;
        state.serialize_element(self.value)?;
        state.end()
    }
}


/// A serializer that serializes the specified tag and value as tuple.
///
/// The tag-value pair will be serialized as tuple where the first element will
/// be the tag and the second element the tuple. The specified serializer
/// performs the actual serialization and thus controls the data format. For
/// more information on this tag-format, see the [module
/// documentation](::ser::adj::tuple).
///
/// # Warning
///
/// You should prefer the [`serialize`](serialize) function over this serializer
/// implementation. To serialize a tuple, the serializer implementation may need
/// to allocate memory on the heap. This can be avoided in the
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
    T: serde::Serialize + 'a + ?Sized,
{
    /// Creates a new Serializer with the specified tag and underlying
    /// serializer.
    pub fn new(delegate: S, tag: &'a T) -> Self {
        Serializer { delegate, tag }
    }

    fn serialize_as_tuple_element<V>(self, value: &V) -> Result<S::Ok, S::Error>
    where
        V: serde::Serialize + ?Sized,
    {
        use serde::ser::SerializeTuple;

        let mut state = self.delegate.serialize_tuple(2)?;
        state.serialize_element(self.tag)?;
        state.serialize_element(value)?;
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
    T: serde::Serialize + 'a + ?Sized,
{
    type Ok = S::Ok;
    type Error = S::Error;

    type SerializeSeq = SerializeSeqAsTupleElement<S::SerializeTuple>;
    type SerializeTuple = SerializeTupleAsTupleElement<S::SerializeTuple>;
    type SerializeTupleStruct = SerializeTupleStructAsTupleElement<S::SerializeTuple>;
    type SerializeMap = SerializeMapAsTupleElement<S::SerializeTuple>;
    type SerializeStruct = SerializeStructAsTupleElement<S::SerializeTuple>;
    type SerializeTupleVariant = SerializeTupleVariantAsTupleElement<S::SerializeTuple>;
    type SerializeStructVariant = SerializeStructVariantAsTupleElement<S::SerializeTuple>;

    fn serialize_bool(self, value: bool) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_tuple_element(&value)
    }

    fn serialize_i8(self, value: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_tuple_element(&value)
    }

    fn serialize_i16(self, value: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_tuple_element(&value)
    }

    fn serialize_i32(self, value: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_tuple_element(&value)
    }

    fn serialize_i64(self, value: i64) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_tuple_element(&value)
    }

    fn serialize_u8(self, value: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_tuple_element(&value)
    }

    fn serialize_u16(self, value: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_tuple_element(&value)
    }

    fn serialize_u32(self, value: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_tuple_element(&value)
    }

    fn serialize_u64(self, value: u64) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_tuple_element(&value)
    }

    fn serialize_f32(self, value: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_tuple_element(&value)
    }

    fn serialize_f64(self, value: f64) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_tuple_element(&value)
    }

    fn serialize_char(self, value: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_tuple_element(&value)
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_tuple_element(value)
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_tuple_element(&forward::Bytes(value))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_tuple_element(&forward::None)
    }

    fn serialize_some<V>(self, value: &V) -> Result<Self::Ok, Self::Error>
    where
        V: serde::Serialize + ?Sized,
    {
        self.serialize_as_tuple_element(&forward::Some(value))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_tuple_element(&forward::Unit)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_tuple_element(&forward::UnitStruct(name))
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_as_tuple_element(&forward::UnitVariant(name, variant_index, variant))
    }

    fn serialize_newtype_struct<V>(
        self,
        name: &'static str,
        value: &V,
    ) -> Result<Self::Ok, Self::Error>
    where
        V: serde::Serialize + ?Sized,
    {
        self.serialize_as_tuple_element(&forward::NewtypeStruct(name, value))
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
        self.serialize_as_tuple_element(&forward::NewtypeVariant(
            name,
            variant_index,
            variant,
            value,
        ))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        use serde::ser::SerializeTuple;

        let mut state = self.delegate.serialize_tuple(2)?;
        state.serialize_element(self.tag)?;

        Ok(SerializeSeqAsTupleElement::new(state, len))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        use serde::ser::SerializeTuple;

        let mut state = self.delegate.serialize_tuple(2)?;
        state.serialize_element(self.tag)?;

        Ok(SerializeTupleAsTupleElement::new(state, len))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        use serde::ser::SerializeTuple;

        let mut state = self.delegate.serialize_tuple(2)?;
        state.serialize_element(self.tag)?;

        Ok(SerializeTupleStructAsTupleElement::new(state, name, len))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        use serde::ser::SerializeTuple;

        let mut state = self.delegate.serialize_tuple(2)?;
        state.serialize_element(self.tag)?;

        Ok(SerializeTupleVariantAsTupleElement::new(
            state,
            name,
            variant_index,
            variant,
            len,
        ))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        use serde::ser::SerializeTuple;

        let mut state = self.delegate.serialize_tuple(2)?;
        state.serialize_element(self.tag)?;

        Ok(SerializeMapAsTupleElement::new(state, len))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        use serde::ser::SerializeTuple;

        let mut state = self.delegate.serialize_tuple(2)?;
        state.serialize_element(self.tag)?;

        Ok(SerializeStructAsTupleElement::new(state, name, len))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        use serde::ser::SerializeTuple;

        let mut state = self.delegate.serialize_tuple(2)?;
        state.serialize_element(self.tag)?;

        Ok(SerializeStructVariantAsTupleElement::new(
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
        self.serialize_as_tuple_element(&forward::CollectSeq::new(iter))
    }

    fn collect_map<K, V, I>(self, iter: I) -> Result<Self::Ok, Self::Error>
    where
        K: serde::Serialize,
        V: serde::Serialize,
        I: IntoIterator<Item = (K, V)>,
    {
        self.serialize_as_tuple_element(&forward::CollectMap::new(iter))
    }

    fn collect_str<V>(self, value: &V) -> Result<Self::Ok, Self::Error>
    where
        V: Display + ?Sized,
    {
        self.serialize_as_tuple_element(&forward::CollectStr(value))
    }

    fn is_human_readable(&self) -> bool {
        self.delegate.is_human_readable()
    }
}


#[doc(hidden)]
pub struct SerializeSeqAsTupleElement<S> {
    state:    S,
    elements: Vec<Content>,
}

impl<S> SerializeSeqAsTupleElement<S> {
    fn new(state: S, len: Option<usize>) -> Self {
        let elements = match len {
            Some(len) => Vec::with_capacity(len),
            None => Vec::new(),
        };

        SerializeSeqAsTupleElement { state, elements }
    }
}

impl<S> serde::ser::SerializeSeq for SerializeSeqAsTupleElement<S>
where
    S: serde::ser::SerializeTuple,
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
        self.state.serialize_element(&Content::Seq(self.elements))?;
        self.state.end()
    }
}


#[doc(hidden)]
pub struct SerializeTupleAsTupleElement<S> {
    state:    S,
    elements: Vec<Content>,
}

impl<S> SerializeTupleAsTupleElement<S> {
    fn new(state: S, len: usize) -> Self {
        SerializeTupleAsTupleElement {
            state:    state,
            elements: Vec::with_capacity(len),
        }
    }
}

impl<S> serde::ser::SerializeTuple for SerializeTupleAsTupleElement<S>
where
    S: serde::ser::SerializeTuple,
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
        self.state.serialize_element(&value)?;
        self.state.end()
    }
}


#[doc(hidden)]
pub struct SerializeTupleStructAsTupleElement<S> {
    state:    S,
    name:     &'static str,
    elements: Vec<Content>,
}

impl<S> SerializeTupleStructAsTupleElement<S> {
    fn new(state: S, name: &'static str, len: usize) -> Self {
        SerializeTupleStructAsTupleElement {
            state:    state,
            name:     name,
            elements: Vec::with_capacity(len),
        }
    }
}

impl<S> serde::ser::SerializeTupleStruct for SerializeTupleStructAsTupleElement<S>
where
    S: serde::ser::SerializeTuple,
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
        self.state.serialize_element(&value)?;
        self.state.end()
    }
}


#[doc(hidden)]
pub struct SerializeTupleVariantAsTupleElement<S> {
    state:         S,
    name:          &'static str,
    variant_index: u32,
    variant:       &'static str,
    elements:      Vec<Content>,
}

impl<S> SerializeTupleVariantAsTupleElement<S> {
    fn new(
        state: S,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Self {
        SerializeTupleVariantAsTupleElement {
            state:         state,
            name:          name,
            variant_index: variant_index,
            variant:       variant,
            elements:      Vec::with_capacity(len),
        }
    }
}

impl<S> serde::ser::SerializeTupleVariant for SerializeTupleVariantAsTupleElement<S>
where
    S: serde::ser::SerializeTuple,
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

        self.state.serialize_element(&value)?;
        self.state.end()
    }
}


#[doc(hidden)]
pub struct SerializeMapAsTupleElement<S> {
    state:    S,
    elements: Vec<(Content, Content)>,
}

impl<S> SerializeMapAsTupleElement<S> {
    fn new(state: S, len: Option<usize>) -> Self {
        let elements = match len {
            Some(len) => Vec::with_capacity(len),
            None => Vec::new(),
        };

        SerializeMapAsTupleElement { elements, state }
    }
}

impl<S> serde::ser::SerializeMap for SerializeMapAsTupleElement<S>
where
    S: serde::ser::SerializeTuple,
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
        self.state.serialize_element(&Content::Map(self.elements))?;
        self.state.end()
    }
}


#[doc(hidden)]
pub struct SerializeStructAsTupleElement<S> {
    state:  S,
    name:   &'static str,
    fields: Vec<(&'static str, Content)>,
}

impl<S> SerializeStructAsTupleElement<S> {
    fn new(state: S, name: &'static str, len: usize) -> Self {
        SerializeStructAsTupleElement {
            state:  state,
            name:   name,
            fields: Vec::with_capacity(len),
        }
    }
}

impl<S> serde::ser::SerializeStruct for SerializeStructAsTupleElement<S>
where
    S: serde::ser::SerializeTuple,
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
        self.state.serialize_element(&value)?;
        self.state.end()
    }
}


#[doc(hidden)]
pub struct SerializeStructVariantAsTupleElement<S> {
    state:         S,
    name:          &'static str,
    variant_index: u32,
    variant:       &'static str,
    fields:        Vec<(&'static str, Content)>,
}

impl<S> SerializeStructVariantAsTupleElement<S> {
    fn new(
        state: S,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Self {
        SerializeStructVariantAsTupleElement {
            state:         state,
            name:          name,
            variant_index: variant_index,
            variant:       variant,
            fields:        Vec::with_capacity(len),
        }
    }
}

impl<S> serde::ser::SerializeStructVariant for SerializeStructVariantAsTupleElement<S>
where
    S: serde::ser::SerializeTuple,
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
        self.state.serialize_element(&value)?;
        self.state.end()
    }
}
