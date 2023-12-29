//! Utilities for de-/serialization.

#[cfg(feature = "erased")]
pub mod erased;

pub(crate) mod de;
pub(crate) mod ser;


use serde;
use std;


/// A type that can be used to as a potentially temporary string-based tag.
/// 
/// This type can be constructed (and deserialiezd) from both an owned string
/// and a borroewd string. While there are similarities to `Cow<str>`, this type
/// will always be deserialized as `Borrowed` if the deserializer allows this.
/// Unlike `&'de str`, this type can, however, also be deserialized from an
/// owned string or a temporary string not fulfilling the required lifetime
/// bound.
/// 
/// The intended use of this type is as a temporary tag store/reference to be
/// passed on to a string-based `SeedFactory` implementation.
#[derive(Clone, Debug)]
pub enum TagString<'a> {
    Owned(String),
    Borrowed(&'a str),
}

impl<'a> From<&'a str> for TagString<'a> {
    fn from(source: &'a str) -> Self {
        TagString::Borrowed(source)
    }
}

impl<'a> From<String> for TagString<'a> {
    fn from(source: String) -> Self {
        TagString::Owned(source)
    }
}

impl<'a> From<std::borrow::Cow<'a, str>> for TagString<'a> {
    fn from(source: std::borrow::Cow<'a, str>) -> Self {
        match source {
            std::borrow::Cow::Owned(v) => TagString::Owned(v),
            std::borrow::Cow::Borrowed(v) => TagString::Borrowed(v),
        }
    }
}

impl<'a> From<TagString<'a>> for std::borrow::Cow<'a, str> {
    fn from(val: TagString<'a>) -> Self {
        match val {
            TagString::Owned(v) => std::borrow::Cow::Owned(v),
            TagString::Borrowed(v) => std::borrow::Cow::Borrowed(v),
        }
    }
}

impl<'a> std::ops::Deref for TagString<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match *self {
            TagString::Owned(ref v) => v,
            TagString::Borrowed(v) => v,
        }
    }
}

impl<'a, B> std::cmp::PartialEq<B> for TagString<'a>
where
    B: PartialEq<str>,
{
    fn eq(&self, other: &B) -> bool {
        other.eq(&**self)
    }
}

impl<'a, 'b> std::cmp::PartialEq<TagString<'b>> for TagString<'a> {
    fn eq(&self, other: &TagString<'b>) -> bool {
        (**self).eq(&**other)
    }
}

impl<'a> std::cmp::Eq for TagString<'a> {}

impl<'a, B> std::cmp::PartialOrd<B> for TagString<'a>
where
    B: PartialOrd<str>,
{
    fn partial_cmp(&self, other: &B) -> Option<std::cmp::Ordering> {
        other.partial_cmp(&**self).map(std::cmp::Ordering::reverse)
    }
}

impl<'a, 'b> std::cmp::PartialOrd<TagString<'b>> for TagString<'a> {
    fn partial_cmp(&self, other: &TagString) -> Option<std::cmp::Ordering> {
        (**self).partial_cmp(&**other)
    }
}

impl<'a> std::cmp::Ord for TagString<'a> {
    fn cmp(&self, other: &TagString) -> std::cmp::Ordering {
        (**self).cmp(&**other)
    }
}

impl<'a> std::fmt::Display for TagString<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        (**self).fmt(f)
    }
}

impl<'a> std::hash::Hash for TagString<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (**self).hash(state)
    }
}

impl<'a> AsRef<str> for TagString<'a> {
    fn as_ref(&self) -> &str {
        self
    }
}


impl<'de> serde::Deserialize<'de> for TagString<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TagString<'de>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "any type of string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(TagString::Owned(v.to_owned()))
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(TagString::Borrowed(v))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(TagString::Owned(v))
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

impl<'a> serde::Serialize for TagString<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        serializer.serialize_str(self)
    }
}
