//! Deserialization of externally tagged values.
//!
//! See [`ser::external`](::ser::external) for a description of this tagging
//! format.

use crate::de::seed::SeedFactory;

use std::fmt;
use std::marker::PhantomData;

use serde;


/// Deserialize an externally tagged value.
///
/// The deserializer controls the underlying data format while the seed-factory
/// specifies the instructions (depending on the tag) on how the value should be
/// deserialized.
///
/// See [`de`](::de) for more information on
/// [`SeedFactory`](::de::SeedFactory) and implementations thereof.
///
/// See [`deserialize_seed`](deserialize_seed) for a version that allows you to
/// pass a `DeserializeSeed` to deserialize the tag. This version is equivalent
/// to `deserialize_seed(deserializer, seed_factory, PhantomData<T>)`
pub fn deserialize<'de, T, D, F>(deserializer: D, seed_factory: F) -> Result<F::Value, D::Error>
where
    T: serde::Deserialize<'de>,
    D: serde::Deserializer<'de>,
    F: SeedFactory<'de, T>,
{
    deserialize_seed(deserializer, seed_factory, PhantomData::<T>)
}


/// Deserialize an externally tagged value with the given tag-seed.
///
/// The deserializer controls the underlying data format while the seed-factory
/// specifies the instructions (depending on the tag) on how the value should be
/// deserialized.
///
/// See [`de`](::de) for more information on
/// [`SeedFactory`](::de::SeedFactory) and implementations thereof.
pub fn deserialize_seed<'de, D, F, S>(
    deserializer: D,
    seed_factory: F,
    tag_seed: S,
) -> Result<F::Value, D::Error>
where
    D: serde::Deserializer<'de>,
    F: SeedFactory<'de, S::Value>,
    S: serde::de::DeserializeSeed<'de>,
{
    deserializer.deserialize_map(Visitor::new(seed_factory, tag_seed))
}


/// A visitor that can be used to deserialize an externally tagged value.
///
/// This visitor handles an externally tagged value, which is represented by a
/// map containing a single entry, where the key is the tag and the value is the
/// value that should be deserialized. Thus it will return an error if the
/// visited type is not a map.
///
/// The [`SeedFactory`](::de::SeedFactory) provided to this visitor
/// provides a `serde::de::DeserializeSeed` implementation depending on the tag,
/// which then determines how the value is going to be deserialized.
///
/// See [`de`](::de) for more information on
/// [`SeedFactory`](::de::SeedFactory) and implementations thereof.
pub struct Visitor<F, S> {
    seed_factory: F,
    tag_seed:     S,
}

impl<F, S> Visitor<F, S> {
    /// Creates a new visitor with the given
    /// [`SeedFactory`](::de::SeedFactory).
    pub fn new(seed_factory: F, tag_seed: S) -> Self {
        Visitor {
            seed_factory,
            tag_seed,
        }
    }
}

impl<'de, F, S> serde::de::Visitor<'de> for Visitor<F, S>
where
    F: SeedFactory<'de, S::Value>,
    S: serde::de::DeserializeSeed<'de>,
{
    type Value = F::Value;

    fn expecting(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "a map with exactly one entry")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        use serde::de::Error;

        // try to validate the length
        match map.size_hint() {
            Some(n) if n != 1 => Err(serde::de::Error::invalid_length(n, &self))?,
            _ => {},
        }

        let tag = map
            .next_key_seed(self.tag_seed)?
            .ok_or_else(|| Error::invalid_length(0, &"a map with exactly one entry"))?;

        map.next_value_seed(self.seed_factory.seed(tag)?)
    }
}
