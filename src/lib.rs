//! # Tagging values for de-/serialization
//! 
//! `serde` does not provide a way to store type-information during
//! serialization, thus de-/serializing trait-objects requires a considerable
//! amount of boilerplate code to work. This library aims to help with that by
//! providing multiple ways to store information, i.e. a tag, associated with
//! a value and later retreive that information during deserialization.
//! The retreived tag information can then be used to select a type-specific
//! deserializer.
//! 
//! A tag can be almost any type, its requirements mainly depend on what you
//! want to use it for. It should implement `Serialize` and `Deserialize`
//! (otherwise it would of course be impossible to de-/serialize it). It
//! should also provide means for comparison, which is required to access the
//! specific deserializer associated with that tag (unless the tag serves
//! another purpose). Further restrictions on the tag type may be imposed by
//! the data- and tag-format you choose (e.g. JSON only allows strings as keys
//! in JSON-objects).
//! 
//! This library provides multiple formats to store (and retreive) tags, that
//! are somewhat similar to the way enums can be tagged in `serde`. The
//! (currently) supported formats are:
//! 
//! - [externally tagged](::ser::external), as in `{ <tag> => <value> }`
//! - [adjacently tagged using tuples](::ser::adj::tuple), as in `( <tag>, <value> )`
//! 
//! ## Examples
//! For some examples have a look at the examples directory in the repository.
//! 
//! ## Hint
//! Tagged serialization requires access to a `Serializer`, however some data
//! formats do not provide direct access to the serializer. In such a case you
//! could create a wrapper-type with a custom `Serialize` implementation.

#![cfg_attr(feature="cargo-clippy", allow(redundant_field_names))]

extern crate serde;

#[cfg(feature = "erased")]
extern crate erased_serde;


pub mod de;
pub mod ser;
pub mod util;
