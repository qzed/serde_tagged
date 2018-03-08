//! An example for de-/serialization of trait objects.
//! 
//! Serializes trait-objects by enhancing the stored information with a tag,
//! then later deserializes the stored tag, based on which a deserializer will
//! be chosen for the value.
//! 
//! The data-structures in this example are straightforward, meaning that
//! using an enum would probably make more sense here. However, enums can not
//! be extended, e.g. by a user of your library, thus sometimes trait-objects
//! are the only way.

extern crate serde;
extern crate serde_json;
extern crate serde_tagged;

extern crate erased_serde;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate downcast_rs;


use std::collections::BTreeMap;

use serde_tagged::de::BoxFnSeed;
use serde_tagged::util::erased::SerializeErased;

use downcast_rs::Downcast;


// Let's begin by defining some data-types.

/// Our first type.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct A {
    foo: String,
}

/// Our second type.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum B {
    Str(String),
    Int(i64),
}


// You can use all de-/serializable data types in combination with this crate,
// we however will limit ourselves to these two for this example.

// We now need a way to identify our types.
// For this, we create a new trait. This trait returns a tag that will later be
// stored with our trait-object to describe its type.
// In general, the only requirements on the tag are that it implements
// `Serialize`. However since we are using the JSON format in combination with
// external tagging (and JSON only allows strings for object-keys), the tag
// must be a string.

/// A trait that provides a (unique) ID for a type.
pub trait TypeId {
    fn type_id(&self) -> &'static str;
}

// We also need to implement this trait for our types (and all types we want to
// de-/serialize as the same trait objects).

impl TypeId for A {
    fn type_id(&self) -> &'static str {
        "A"
    }
}

impl TypeId for B {
    fn type_id(&self) -> &'static str {
        "B"
    }
}


// Next we define the trait that we actually want to store as trait-object.
// this trait should require our `TypeId` trait, as well as all other traits
// that we want to be able to use on the trait object.

// One trait that is required for serialization and must be present on the
// trait-object to work is `erased_serde::Serialize`.
// Note that we can not use `serde::Serialize` due to it containing a generic
// method, however `erased_serde::Serialize` is automatically implemented for
// all types implementing `serde::Serialize`, so the only thing we have to do
// is add the trait bound here. No changes on our types are required.

// FYI: This is required to enforce a fixed v-table layout, which is required
//      to create a trait object from a set of traits.

/// The trait we actually want to store as trait object.
pub trait Stored: erased_serde::Serialize + TypeId + std::fmt::Debug + Downcast {}

// In this case, we also want to automatically implement it for all types which
// meet our requirements.
impl<T> Stored for T
where
    T: erased_serde::Serialize + TypeId + std::fmt::Debug + Downcast,
{}

// We also want a way to access the actual object.
// For this we can use `downcast_rs` (or a similar library).
impl_downcast!(Stored);


// Now we can implement `Serialize` and `Deserialize` for our trait objects.
// In this example we use external tagging, but you could also use any other
// strategy provided by this crate.

// WARNING:
// If we would serialize non-trait-objects (i.e. `Box<A>` or `Box<B>`), no
// tag will be emitted, as the `Serialize` implementation of `Box<A>`
// forwards to the `Serialize` implementation of `A`.
// Thus you should make sure that you always serialize a trait-object when
// you want to deserialize a trait object. To enforce this at compile time,
// you could implement a custom wrapper type.

impl<'a> serde::Serialize for Stored + 'a {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // As tag we simply use the ID provided by our `TypeId` trait.
        // To serialize our trait object value (without the tag) we actually
        // need to call `erased_serde::serialize`. We can do this by wrapping
        // the object in `SerializeErased`.
        // The `serialize` method of `serde_erased::ser::external` will apply
        // our type-id as tag to the trait-object.
        serde_tagged::ser::external::serialize(serializer, self.type_id(), &SerializeErased(self))
    }
}

impl<'de> serde::Deserialize<'de> for Box<Stored> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // For deserialization we need to choose the same tag format as with
        // serialization.
        // Additionally, we need to provide a `serde::de::DeserializeSeed`
        // implementation for every tag (`type_id`) we want to support, which
        // creates our trait-object (usually in the form of `Box<Trait>`).
        // These implementations are provided by a `SeedFactory`.
        // Here, this factory is provided by the `get_registry()` function.
        serde_tagged::de::external::deserialize(deserializer, get_registry())
    }
}


// The last component we need is our `SeedFactory` and its `DeserializeSeed`
// implementations.
// In this example we simply use a `BTreeMap` with closures (stored in
// `BoxFnSeed`), for which the necessary traits are already implemented.

// You would probably want to implement your own registry type here. For this
// you should have a look at the traits provided in `de::seed`.

/// The type of our `SeedFactory`.
pub type TypeRegistry = BTreeMap<&'static str, BoxFnSeed<Box<Stored>>>;

/// Return the type registry required for deserialization.
pub fn get_registry() -> &'static TypeRegistry {
    lazy_static! {
        static ref REG: TypeRegistry = {
            let mut reg = TypeRegistry::new();
            reg.insert("A", BoxFnSeed::new(deserialize_erased::a));
            reg.insert("B", BoxFnSeed::new(deserialize_erased::b));
            reg
        };
    }

    &REG
}

// Due to the complexity of the function signature, we need to implement our
// deserialization functions as actual functions. `rustc` will complain about
// lifetime-issues (for the `Deserializer`) if we use closures.
mod deserialize_erased {
    use super::*;
    use erased_serde::{Deserializer, Error};
    use serde::Deserialize;

    /// Deserialize a value of type `A` as trait-object.
    pub fn a<'de>(de: &mut Deserializer<'de>) -> Result<Box<Stored>, Error> {
        Ok(Box::new(A::deserialize(de)?))
    }

    /// Deserialize a value of type `B` as trait-object.
    pub fn b<'de>(de: &mut Deserializer<'de>) -> Result<Box<Stored>, Error> {
        Ok(Box::new(B::deserialize(de)?))
    }
}


// We have finally set up everything, now we can test it.
fn main() {
    // Let's begin by creating our test data ...
    let a = Box::new(A { foo: "bar".to_owned() });
    let b = Box::new(B::Str("Hello World".to_owned()));
    let c = Box::new(B::Int(42));

    // ... and then transform it to trait objects.
    // We use clone here so we can later assert that de-/serialization does not
    // change anything.
    let ser_a: Box<Stored> = a.clone();
    let ser_b: Box<Stored> = b.clone();
    let ser_c: Box<Stored> = c.clone();

    // Now we can serialize our trait-objects.
    // Thanks to our `Serialize` implementation for trait objects this works
    // just like with any other type.
    let ser_a = serde_json::to_string_pretty(&ser_a).unwrap();
    let ser_b = serde_json::to_string_pretty(&ser_b).unwrap();
    let ser_c = serde_json::to_string_pretty(&ser_c).unwrap();

    // Again note the warning regarding serialization of non-trait-objects
    // above.

    // We specified external tagging, so we expect the following:
    assert_json_equal(&ser_a, r###"
    {
        "A": {
            "foo": "bar"
        }
    }
    "###);

    assert_json_equal(&ser_b, r###"
    {
        "B": {
            "Str": "Hello World"
        }
    }
    "###);

    assert_json_equal(&ser_c, r###"
    {
        "B": {
            "Int": 42
        }
    }
    "###);


    // Now we let's deserialize our trait objects.
    // This works also just like any other type.
    let de_a: Box<Stored> = serde_json::from_str(&ser_a).unwrap();
    let de_b: Box<Stored> = serde_json::from_str(&ser_b).unwrap();
    let de_c: Box<Stored> = serde_json::from_str(&ser_c).unwrap();

    // Using `downcast_rs` we can get the real types ...
    let ref_a: &A = de_a.downcast_ref().unwrap();
    let ref_b: &B = de_b.downcast_ref().unwrap();
    let ref_c: &B = de_c.downcast_ref().unwrap();

    // ... and values should be equal to what we have serialized.
    assert_eq!(&*a, ref_a);
    assert_eq!(&*b, ref_b);
    assert_eq!(&*c, ref_c);
}


/// A helper function to assert that two strings contain the same JSON data.
fn assert_json_equal(a: &str, b: &str) {
    let a: serde_json::Value = serde_json::from_str(a).unwrap();
    let b: serde_json::Value = serde_json::from_str(b).unwrap();
    assert_eq!(a, b);
}
