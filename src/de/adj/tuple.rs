//! Deserialization of adjacently tagged values using tuples.
//!
//! See [`ser::adj::tuple`](::ser::adj::tuple) for a description of this tagging
//! format.

use de::seed::SeedFactory;

use std::fmt;
use std::marker::PhantomData;

use serde;


pub fn deserialize<'de, T, V, D, F>(deserializer: D, seed_factory: F) -> Result<V, D::Error>
where
    T: serde::Deserialize<'de>,
    D: serde::Deserializer<'de>,
    F: SeedFactory<'de, T, Value = V>,
{
    deserializer.deserialize_tuple(2, Visitor::<T, V, F>::new(seed_factory))
}


pub struct Visitor<T, V, F> {
    seed_factory: F,
    _phantom_t:   PhantomData<T>,
    _phantom_v:   PhantomData<V>,
}

impl<T, V, F> Visitor<T, V, F> {
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

        let tag: T = seq.next_element()?
            .ok_or_else(|| Error::invalid_length(0, &self))?;

        seq.next_element_seed(self.seed_factory.seed(tag)?)?
            .ok_or_else(|| Error::invalid_length(1, &"a tuple with exactly two elements"))
    }
}
