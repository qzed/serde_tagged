//! Deserialization of externally tagged values.
//!
//! See [`ser::external`](::ser::external) for a description of this tagging
//! format.

use de::seed::SeedFactory;

use std::fmt;
use std::marker::PhantomData;

use serde;


/// Deserialize an externally tagged value with the given deserializer and
/// seed-factory.
///
/// The deserializer controls the underlying data format while the seed-factory
/// specifies the instructions (depending on the tag) on how the value should be
/// deserialized.
///
/// See [`de::seed`](::de::seed) for more information on
/// [`SeedFactory`](::de::seed::SeedFactory) and implementations thereof.
pub fn deserialize<'de, T, V, D, F>(deserializer: D, seed_factory: F) -> Result<V, D::Error>
where
    T: serde::Deserialize<'de>,
    D: serde::Deserializer<'de>,
    F: SeedFactory<'de, T, Value = V>,
{
    deserializer.deserialize_map(Visitor::<T, V, F>::new(seed_factory))
}


/// A visitor that can be used to deserialize an externally tagged value.
///
/// This visitor handles an externally tagged value, which is represented by a
/// map containing a single entry, where the key is the tag and the value is the
/// value that should be deserialized. Thus it will return an error if the
/// visited type is not a map.
///
/// The [`SeedFactory`](::de::seed::SeedFactory) provided to this visitor
/// provides a `serde::de::DeserializeSeed` implementation depending on the tag,
/// which then determines how the value is going to be deserialized.
///
/// See [`de::seed`](::de::seed) for more information on
/// [`SeedFactory`](::de::seed::SeedFactory) and implementations thereof.
pub struct Visitor<T, V, F> {
    seed_factory: F,
    _phantom_t:   PhantomData<T>,
    _phantom_v:   PhantomData<V>,
}

impl<T, V, F> Visitor<T, V, F> {
    /// Creates a new visitor with the given
    /// [`SeedFactory`](::de::seed::SeedFactory).
    pub fn new(seed_factory: F) -> Self {
        Visitor {
            seed_factory,
            _phantom_t: PhantomData,
            _phantom_v: PhantomData,
        }
    }
}

impl<'de, T, V, F> serde::de::Visitor<'de> for Visitor<T, V, F>
where
    T: serde::Deserialize<'de>,
    F: SeedFactory<'de, T, Value = V>,
{
    type Value = V;

    fn expecting(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "a map with exactly one entry")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let tag: T = map.next_key()?
            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;

        map.next_value_seed(self.seed_factory.seed(tag)?)
    }
}
