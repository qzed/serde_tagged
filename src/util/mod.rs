//! Utilities for de-/serialization.

#[cfg(feature = "erased")]
pub mod erased;

pub(crate) mod de;
pub(crate) mod ser;
