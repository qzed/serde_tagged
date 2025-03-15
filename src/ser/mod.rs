//! Serialization of tagged values.

pub mod adj;
pub mod external;
pub mod internal;


use serde;

/// A trait to provide access to a delegate serializer.
///
/// The delegate is expected to be responsible for the data-format of a
/// serializer that implements this trait.
pub trait HasDelegate {
    type Ok;
    type Error: serde::ser::Error;
    type Delegate: serde::Serializer<Ok = Self::Ok, Error = Self::Error>;

    /// Returns the delegate-serializer of this object.
    fn delegate(self) -> Self::Delegate;
}
