//! Serialization of internally tagged values.
//!
//! Tagging values internally will embed the tag in the value. As the tag is
//! being embedded in the value, not all types can be supported by this format
//! (see section below).
//!
//! This format is similar to the internally-tagged enum format provided by
//! serde, however allows for various tag types (not only `str` and `u32`).
//!
//! # Warning
//!
//! Deserialization of internally tagged values requires a self-describing
//! data format.
//!
//! Furthermore, neither [`serialize`] nor the
//! [`Serializer`] check for collision of the tag-key with
//! field-names or map-keys of the value with the tag-key. It is up to the
//! caller to make sure that such collisions do not occur.
//!
//! # Supported types
//!
//! Only the following types (of [Serde's data model][datamodel]) are supported
//! by this tagging format:
//!
//! - __seq__
//!   * Dynamically sized sequences.
//!   * The tag will be the first element of the sequence.
//! - __tuple__
//!   * Statically sized sequences.
//!   * The tag will be the first element of the tuple.
//! - __map__
//!   * Dynamically sized mappings.
//!   * The tag-key and tag pair will be added as (first) entry of the mapping:
//! - __struct__
//!   * Any normal struct (e.g. `struct Name { ... }`).
//!   * The tag will be added as first field under the specified name.
//! - __unit struct__
//!   * A struct without content (e.g. `struct Unit;`).
//!   * The struct will be serialized as normal struct and the tag will be added
//!     as first (and only) field under the specified name.
//! - __newtype struct__ _only if it contains a value that can be serialized
//!   with this format_
//!   * A tuple struct containing only a single value (e.g.
//!     `struct Newtype(i32)`).
//!   * The struct will be serialized as tuple struct and the tag will be added
//!     as first element of the tuple.
//! - __tuple struct__
//!   * A struct containing multiple unnamed members (e.g.
//!     `struct Tuple(i32, i32)`).
//!   * The tag will be added as first element of the tuple.
//! - __internally tagged enum__: any variant
//!   * An enum with the `#[serde(tag="<tag-key>")]` attribute.
//!   * The tag will be added as first field under the specified name.
//! - __adjacently tagged enum__: any variant
//!   * An enum with the `#[serde(tag="<tag-key>", content="<content-key>")]`
//!     attribute.
//!   * The tag will be added as entry with the specified name as key to the
//!     generated mapping.
//! - __untagged enum__: _tuple_, _non-primitive newtype_, and _struct_ variants
//!   only
//!   * An enum with the `#[serde(untagged)]` attribute.
//!   * The tag will be embedded using the previously elaborated rules
//!     corresponding to the respective variant type.
//!
//! Primitive types and externally tagged enums are not supported.
//!
//! # Examples serializing to JSON
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
//! serde_tagged::ser::internal::serialize(&mut serializer, "tag-key", "my-tag", &foo).unwrap();
//! # }
//! ```
//!
//! with a tag-value of `"my-tag"` and a tag-field-name of `"tag-key"` will
//! produce the following JSON output:
//!
//! ```json
//! {
//!     "tag-key": "my-tag",
//!     "bar": "baz"
//! }
//! ```
//!
//!
//! [datamodel]: https://serde.rs/data-model.html

use serde;

use crate::ser::HasDelegate;


/// Embeds a tag into the specified value and then serializes it using the
/// provided serializer.
///
/// Due to the tag being embedded into the value, not all value-types are
/// supported. The specified serializer performs the actual serialization and
/// thus controls the data format. For more information on this trag-format and
/// the supported values, see the [module documentation](crate::ser::internal).
///
/// This method is a convenience function that creates and uses the
/// [`Serializer`] internally.
///
/// # Warning
///
/// This function does not provide any checks regarding collisions of the
/// `tag_key` with field-names or map-keys. The responsibility for such checks
/// reside with the caller.
pub fn serialize<S, T, V>(
    serializer: S,
    tag_key: &'static str,
    tag: &T,
    value: &V,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
    T: serde::Serialize + ?Sized,
    V: serde::Serialize + ?Sized,
{
    value.serialize(Serializer::new(serializer, tag_key, tag))
}


/// A serializer that embeds a tag into to the provided value and then
/// serializes it.
///
/// Due to the tag being embedded into the value, not all value-types are
/// supported. The provided serializer performs the actual serialization and
/// thus controls the data format. For more information on this trag-format and
/// the supported values, see the [module documentation](crate::ser::internal).
///
/// Due to the tag being embedded into the value, not all value-types are
/// supported. For more details see the [module
/// documentation](crate::ser::internal).
///
/// # Warning
///
/// This serializer does not provide any checks regarding collisions of the
/// `tag_key` with field-names or map-keys. The responsibility for such checks
/// reside with the caller.
pub struct Serializer<'a, S, T>
where
    T: ?Sized + 'a,
{
    delegate: S,
    tag_key:  &'static str,
    tag:      &'a T,
}

impl<'a, S, T> Serializer<'a, S, T>
where
    S: serde::Serializer,
    T: serde::Serialize + ?Sized + 'a,
{
    /// Creates a new Serializer with the specified tag-key, tag and underlying
    /// serializer.
    pub fn new(delegate: S, tag_key: &'static str, tag: &'a T) -> Self {
        Serializer {
            delegate,
            tag_key,
            tag,
        }
    }

    fn unsupported(&self, what: &'static str) -> S::Error {
        serde::ser::Error::custom(format_args!("cannot serialize {} as tagged value", what))
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

    type SerializeSeq = S::SerializeSeq;
    type SerializeTuple = S::SerializeTuple;
    type SerializeTupleStruct = S::SerializeTupleStruct;
    type SerializeMap = S::SerializeMap;
    type SerializeStruct = S::SerializeStruct;
    type SerializeTupleVariant = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = serde::ser::Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, _value: bool) -> Result<Self::Ok, Self::Error> {
        Err(self.unsupported("a boolean"))
    }

    fn serialize_i8(self, _value: i8) -> Result<Self::Ok, Self::Error> {
        Err(self.unsupported("an integer"))
    }

    fn serialize_i16(self, _value: i16) -> Result<Self::Ok, Self::Error> {
        Err(self.unsupported("an integer"))
    }

    fn serialize_i32(self, _value: i32) -> Result<Self::Ok, Self::Error> {
        Err(self.unsupported("an integer"))
    }

    fn serialize_i64(self, _value: i64) -> Result<Self::Ok, Self::Error> {
        Err(self.unsupported("an integer"))
    }

    fn serialize_u8(self, _value: u8) -> Result<Self::Ok, Self::Error> {
        Err(self.unsupported("an integer"))
    }

    fn serialize_u16(self, _value: u16) -> Result<Self::Ok, Self::Error> {
        Err(self.unsupported("an integer"))
    }

    fn serialize_u32(self, _value: u32) -> Result<Self::Ok, Self::Error> {
        Err(self.unsupported("an integer"))
    }

    fn serialize_u64(self, _value: u64) -> Result<Self::Ok, Self::Error> {
        Err(self.unsupported("an integer"))
    }

    fn serialize_f32(self, _value: f32) -> Result<Self::Ok, Self::Error> {
        Err(self.unsupported("a float"))
    }

    fn serialize_f64(self, _value: f64) -> Result<Self::Ok, Self::Error> {
        Err(self.unsupported("a float"))
    }

    fn serialize_char(self, _value: char) -> Result<Self::Ok, Self::Error> {
        Err(self.unsupported("a char"))
    }

    fn serialize_str(self, _value: &str) -> Result<Self::Ok, Self::Error> {
        Err(self.unsupported("a string"))
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(self.unsupported("a byte array"))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(self.unsupported("an optional"))
    }

    fn serialize_some<V>(self, _value: &V) -> Result<Self::Ok, Self::Error>
    where
        V: serde::Serialize + ?Sized,
    {
        Err(self.unsupported("an optional"))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(self.unsupported("a unit"))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(self.unsupported("a unit-variant"))
    }

    fn serialize_newtype_variant<V>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &V,
    ) -> Result<Self::Ok, Self::Error>
    where
        V: serde::Serialize + ?Sized,
    {
        Err(self.unsupported("a newtype-variant"))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(self.unsupported("a tuple-variant"))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(self.unsupported("a struct-variant"))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        use serde::ser::SerializeTuple;

        let mut state = self.delegate.serialize_tuple(len + 1)?;
        state.serialize_element(self.tag)?;
        Ok(state)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        use serde::ser::SerializeSeq;

        let mut state = self.delegate.serialize_seq(len.map(|len| len + 1))?;
        state.serialize_element(self.tag)?;
        Ok(state)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        use serde::ser::SerializeMap;

        let mut state = self.delegate.serialize_map(len.map(|len| len + 1))?;
        state.serialize_entry(self.tag_key, self.tag)?;
        Ok(state)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        use serde::ser::SerializeStruct;

        let mut state = self.delegate.serialize_struct(name, 1)?;
        state.serialize_field(self.tag_key, self.tag)?;
        state.end()
    }

    fn serialize_newtype_struct<V>(
        self,
        _name: &'static str,
        value: &V,
    ) -> Result<Self::Ok, Self::Error>
    where
        V: serde::Serialize + ?Sized,
    {
        value.serialize(self)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        use serde::ser::SerializeTupleStruct;

        let mut state = self.delegate.serialize_tuple_struct(name, len + 1)?;
        state.serialize_field(self.tag)?;
        Ok(state)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        use serde::ser::SerializeStruct;

        let mut state = self.delegate.serialize_struct(name, len + 1)?;
        state.serialize_field(self.tag_key, self.tag)?;
        Ok(state)
    }

    fn is_human_readable(&self) -> bool {
        self.delegate.is_human_readable()
    }
}
