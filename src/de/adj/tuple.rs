//! Deserialization of adjacently tagged values using tuples.
//!
//! See [`ser::adj::tuple`](::ser::adj::tuple) for a description of this tagging
//! format.

use crate::de::seed::SeedFactory;

use std::fmt;
use std::marker::PhantomData;

use serde;


/// Deserialize a tuple-based adjacently tagged value.
///
/// The deserializer controls the underlying data format while the seed-factory
/// specifies the instructions (depending on the tag) on how the value should be
/// deserialized.
///
/// See [`de`](::de) for more information on
/// [`SeedFactory`](::de::SeedFactory) and implementations thereof.
///
/// See [`deserialize_seed`](deserialize_seed) for a version that allows you to
/// pass a `DeserializeSeed` implementation to deserialize the tag. This version
/// is equivalent to `deserialize_seed(deserializer, seed_factory,
/// PhantomData<T>)`
pub fn deserialize<'de, T, D, F>(deserializer: D, seed_factory: F) -> Result<F::Value, D::Error>
where
    T: serde::Deserialize<'de>,
    D: serde::Deserializer<'de>,
    F: SeedFactory<'de, T>,
{
    deserialize_seed(deserializer, seed_factory, PhantomData::<T>)
}


/// Deserialize a tuple-based adjacently tagged value with the given tag-seed.
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
    deserializer.deserialize_tuple(2, Visitor::new(seed_factory, tag_seed))
}


/// A visitor that can be used to deserialize a tuple-based adjacently tagged
/// value.
///
/// This visitor handles a tuple-based adjacently tagged value, which is
/// represented by a tuple containing exactly two elements. The first element of
/// this tuple is the tag, the second element the value. Thus this visitor will
/// return an error if the visited type is not a tuple (i.e. sequence) with two
/// elements.
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
        write!(fmtr, "a tuple with exactly two elements")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        use serde::de::Error;

        // try to validate the length
        match seq.size_hint() {
            Some(n) if n != 2 => Err(Error::invalid_length(n, &self))?,
            _ => {},
        }

        let tag = seq
            .next_element_seed(self.tag_seed)?
            .ok_or_else(|| Error::invalid_length(0, &"a tuple with exactly two elements"))?;

        seq.next_element_seed(self.seed_factory.seed(tag)?)?
            .ok_or_else(|| Error::invalid_length(1, &"a tuple with exactly two elements"))
    }
}
