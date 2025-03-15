//! An example for tagged serialization.

extern crate serde;
extern crate serde_json;
extern crate serde_tagged;

#[macro_use]
extern crate serde_derive;


use serde::Serialize;
use serde_json::Value;


/// The data structure we want to serialize.
#[derive(Serialize)]
struct Test {
    foo: &'static str,
    baz: i32,
}


/// A helper-function that does all the heavy lifting.
///
/// Serializes the given value with the given tag to a JSON string using the
/// external tagging format.
fn serialize_to_string_external<T, V>(tag: &T, value: &V) -> String
where
    T: Serialize + ?Sized,
    V: Serialize + ?Sized,
{
    // Create a buffer where we will store the data.
    let mut buf = Vec::with_capacity(128);
    {
        // Create a serializer.
        let mut serializer = serde_json::Serializer::new(&mut buf);

        // Serialize the value with the tag.
        serde_tagged::ser::external::serialize(&mut serializer, tag, value).unwrap();
    }

    // Create a string from the data.
    String::from_utf8(buf).unwrap()
}


/// A helper function to assert that two strings contain the same JSON data.
fn assert_json_equal(a: &str, b: &str) {
    let a: Value = serde_json::from_str(a).unwrap();
    let b: Value = serde_json::from_str(b).unwrap();
    assert_eq!(a, b);
}


fn main() {
    // A simple struct.
    let value_1 = Test {
        foo: "bar",
        baz: 42,
    };

    let ser = serialize_to_string_external("Type", &value_1);
    assert_json_equal(
        &ser,
        r###"
    {
        "Type": {
            "foo": "bar",
            "baz": 42
        }
    }
    "###,
    );

    // You can serialize everything that implements `Serialize` with a tag.
    let ser = serialize_to_string_external("Int", &42_i32);
    assert_json_equal(
        &ser,
        r###"
    {
        "Int": 42
    }
    "###,
    );
}
