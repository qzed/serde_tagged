//! Utilities for de-/serialization with `erased_serde`.

use erased_serde;
use serde;


/// A serializable type that serializes the enclosed value using `erased_serde`.
pub struct SerializeErased<'b, T: ?Sized + 'b>(pub &'b T);

impl<'b, T> serde::Serialize for SerializeErased<'b, T>
where
    T: erased_serde::Serialize + ?Sized,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        erased_serde::serialize(self.0, serializer)
    }
}
