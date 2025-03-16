//! # Tagging values for de-/serialization
//!
//! `serde` does not provide a way to store type-information during
//! serialization, thus de-/serializing trait-objects requires a considerable
//! amount of boilerplate code to work. This library aims to help with that by
//! providing multiple ways to store information, i.e. a tag, associated with
//! a value and later retrieve that information during deserialization.
//! The retrieved tag information can then be used to select a type-specific
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
//! This library provides multiple formats to store (and retrieve) tags, that
//! are somewhat similar to the way enums can be tagged in `serde`. The
//! (currently) supported formats are:
//!
//! - [externally tagged](::ser::external), as in `{ <tag> => <value> }`
//! - [internally tagged](::ser::internal) (i.e. embedded in the value)
//! - [adjacently tagged using tuples](::ser::adj::tuple), as in
//!   `( <tag>, <value> )`
//! - [adjacently tagged using maps](::ser::adj::map), as in
//!   `{ <tag-key> => <tag>, <value-key> => <value> }`
//! - [adjacently tagged using structs](::ser::adj::struc), as in
//!   `{ <tag-key>: <tag>, <value-key>: <value> }`
//!
//! ## A quick overview
//!
//! This crate is separated into two main modules: [`ser`](::ser) for
//! serialization and [`de`](::de) for deserialization. Both modules contain
//! further submodules, each representing a separate tagging-format.
//!
//! ### Serialization
//!
//! For each tagging-format both, a `serialize` function and a `Serializer`
//! are provided, which allow for values to be serialized with a pre-existing
//! serializer defining the data format, a tag, and possibly further
//! format-specific parameters. You should always prefer the `serialize`
//! function to the `Serializer` as it, in most cases, allows for a better
//! performance.
//!
//! __Note:__
//! Tagged serialization requires access to a `Serializer`, however some data
//! formats do not provide direct access to the serializer. In such a case you
//! could create a wrapper-type with a custom `Serialize` implementation.
//!
//! ### Deserialization
//!
//! For deserialization, the `deserialize` function (and in some cases also
//! variants of it) are provided in the respective format-modules. Have a look
//! the respective function documentation for more details.
//!
//! ## Examples
//!
//! For some examples have a look at the examples directory in the repository.

#![allow(clippy::redundant_field_names)]

#[macro_use]
extern crate serde;

#[cfg(feature = "erased")]
extern crate erased_serde;


pub mod de;
pub mod ser;
pub mod util;
