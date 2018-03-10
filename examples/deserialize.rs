//! An example for the various deserialization methods available with this
//! crate.

extern crate erased_serde;
extern crate serde;
extern crate serde_json;
extern crate serde_tagged;

#[macro_use]
extern crate serde_derive;


use serde_tagged::de::external::deserialize;
use serde_tagged::de::{BoxFnSeed, SeedFactory, WithTag, WithoutTag};

use std::collections::BTreeMap;

use serde::Deserialize;


/// A trait that can help us identify the real type of a trait-object.
trait TypeId: std::fmt::Debug {
    fn type_id(&self) -> &'static str;
}


/// A simple struct.
#[derive(Debug, Serialize, Deserialize)]
struct Message {
    id:   u32,
    text: String,
}

impl TypeId for Message {
    fn type_id(&self) -> &'static str {
        "Message"
    }
}

// We can also store primitive types as trait-objects.
impl TypeId for i64 {
    fn type_id(&self) -> &'static str {
        "i64"
    }
}


fn main() {
    // First create some test-data by serializing the two types we want to use
    // (Message and i64).
    let mut buf_msg = Vec::new();
    {
        let value = Message {
            id:   0,
            text: "Hello World".to_owned(),
        };

        let mut serializer = serde_json::Serializer::new(&mut buf_msg);
        serde_tagged::ser::external::serialize(&mut serializer, value.type_id(), &value).unwrap();
    }

    let mut buf_i64 = Vec::new();
    {
        let value: i64 = 42;

        let mut serializer = serde_json::Serializer::new(&mut buf_i64);
        serde_tagged::ser::external::serialize(&mut serializer, value.type_id(), &value).unwrap();
    }

    println!("-- Serialize data --------------------------------------------------------------");
    println!("Serialized: {}", std::str::from_utf8(&buf_msg).unwrap());
    println!("Serialized: {}", std::str::from_utf8(&buf_i64).unwrap());


    // The general idea of deserialization is that an appropriate action is
    // chosen based on the given tag.
    // This works by providing the a `SeedFactory` implementation that then
    // returns an appropriate `DeserializeSeed` implementation which specifies
    // the instructions to be taken for deserialization.

    // For convenience multiple implementations are provided in this crate.
    // (i.e. `WithoutTag`, `WithTag`, and various implementations for standard
    // map types)

    // We can deserialize a known value directly without the need for a tag.
    // This simply discards the tag.
    println!();
    println!("-- Deserialize known value without tag -----------------------------------------");

    let mut de = serde_json::Deserializer::from_slice(&buf_msg);
    let v: Message = deserialize::<&str, _, _>(&mut de, WithoutTag::new()).unwrap();
    println!("Deserialized: value: {:?}", v);

    let mut de = serde_json::Deserializer::from_slice(&buf_i64);
    let v: i64 = deserialize::<&str, _, _>(&mut de, WithoutTag::new()).unwrap();
    println!("Deserialized: value: {:?}", v);


    // We can also deserialize a known value and retreive the tag, e.g. in case
    // we want to use it for something other than indicating the type of the
    // following value.
    println!();
    println!("-- Deserialize known value with tag --------------------------------------------");

    let mut de = serde_json::Deserializer::from_slice(&buf_msg);
    let (t, v): (&str, Message) = deserialize(&mut de, WithTag::new()).unwrap();
    println!("Deserialized: tag: {:?}, value: {:?}", t, v);

    let mut de = serde_json::Deserializer::from_slice(&buf_i64);
    let (t, v): (&str, i64) = deserialize(&mut de, WithTag::new()).unwrap();
    println!("Deserialized: tag: {:?}, value: {:?}", t, v);


    // You can also deserialize a type out of a fixed set of types. This is
    // basically identical in capabilities with an (externally tagged) enum
    // wrapping your data types. Thus there is little need to do this and you
    // should consider using an (externally tagged) enum instead.
    //
    // If you still want to do this, you can simply implement your own
    // `SeedFactory` as shown below.
    println!();
    println!("-- Deserialize trait-object with TypeSeedFactory -------------------------------");

    let mut de = serde_json::Deserializer::from_slice(&buf_msg);
    let v = deserialize(&mut de, TypeSeedFactory).unwrap();
    println!("Deserialized: value: {:?}", v);

    let mut de = serde_json::Deserializer::from_slice(&buf_i64);
    let v = deserialize(&mut de, TypeSeedFactory).unwrap();
    println!("Deserialized: value: {:?}", v);


    // If you do not know all data types (e.g. the user of your library is able
    // to extend this set of types) you have no choice but to use trait objects.
    // For this you should have a look at the `trait_objects` example, however
    // here is a small deserialization example.
    println!();
    println!("-- Deserialize trait-object with (BTree)Map ------------------------------------");

    // For this we need a type registry that contains a mapping from the tag of
    // a data type to instructions on how to deserialize this data type.
    // For this you need a `SeedFactory` that produces `DeserializeSeed`
    // implementations which are then used to deserialize the specific type.
    // To this end this crate provides a few convenience implementations that
    // we will use here.
    type RegSeedFn = BoxFnSeed<Box<TypeId>>;
    let mut seeds = BTreeMap::new();
    seeds.insert("Message".to_owned(), RegSeedFn::new(deserialize_msg));
    seeds.insert("i64".to_owned(), RegSeedFn::new(deserialize_i64));

    // We need to explicitly write down our deserialization-functions as the
    // (current) rust compiler does not correctly identify the lifetimes and
    // complains about them.
    fn deserialize_msg<'de>(
        de: &mut erased_serde::Deserializer<'de>,
    ) -> Result<Box<TypeId>, erased_serde::Error> {
        Ok(Box::new(Message::deserialize(de)?))
    }

    fn deserialize_i64<'de>(
        de: &mut erased_serde::Deserializer<'de>,
    ) -> Result<Box<TypeId>, erased_serde::Error> {
        Ok(Box::new(i64::deserialize(de)?))
    }

    // Now we can deserialize our data with the created registry.
    let mut de = serde_json::Deserializer::from_slice(&buf_msg);
    let v = deserialize(&mut de, &seeds).unwrap();
    println!("Deserialized: value: {:?}", v);

    let mut de = serde_json::Deserializer::from_slice(&buf_i64);
    let v = deserialize(&mut de, &seeds).unwrap();
    println!("Deserialized: value: {:?}", v);
}


/// A seed factory for a fixed set of types.
/// 
/// This simply creates a new `TypeSeed` with the given tag.
struct TypeSeedFactory;

impl<'de> SeedFactory<'de, &'de str> for TypeSeedFactory {
    type Value = Box<TypeId>;
    type Seed = TypeSeed<'de>;

    fn seed<E>(self, tag: &'de str) -> Result<Self::Seed, E>
    where
        E: serde::de::Error,
    {
        Ok(TypeSeed::new(tag))
    }
}


/// A `DeserializeSeed` implementation for a fixed set of types.
/// 
/// Decides which type should be deserialized by using a simple match statement
/// on the tag.
struct TypeSeed<'de> {
    tag: &'de str,
}

impl<'de> TypeSeed<'de> {
    fn new(tag: &'de str) -> Self {
        TypeSeed { tag }
    }
}

impl<'de> serde::de::DeserializeSeed<'de> for TypeSeed<'de> {
    type Value = Box<TypeId>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match self.tag {
            "Message" => Ok(Box::new(Message::deserialize(deserializer)?)),
            "i64" => Ok(Box::new(i64::deserialize(deserializer)?)),
            tag => Err(serde::de::Error::unknown_variant(tag, &["Message"])),
        }
    }
}
