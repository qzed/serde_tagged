//! Serialization of tagged values.

pub mod adj;
pub mod external;
pub mod internal;


use serde;

pub trait HasDelegate {
    type Ok;
    type Error: serde::ser::Error;
    type Delegate: serde::Serializer<Ok=Self::Ok, Error=Self::Error>;

    fn delegate(self) -> Self::Delegate;
}
