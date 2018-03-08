//! Format-specific serialization/deserialization functions to be used with
//! the macros defined in `logic`.

// Not all formats are used in every test file.
#![allow(unused)]


/// Functions for `serde_json`.
pub mod json {

    /// Functions for serialization.
    pub mod ser {

        /// Functions for external tagging.
        pub mod external {
            use serde::Serialize;
            use serde_json::{self, Result, Value};
            use serde_tagged::ser::external;


            /// Serialize the tagged value with an explicitly created
            /// serializer.
            pub fn serialize_with_serializer<V, T>(tag_value: &T, value: &V) -> Result<Value>
            where
                V: Serialize + ?Sized,
                T: Serialize + ?Sized,
            {
                let mut buffer = Vec::with_capacity(128);
                {
                    let mut json_ser = serde_json::Serializer::new(&mut buffer);
                    let tag_ser = external::Serializer::new(&mut json_ser, tag_value);
                    value.serialize(tag_ser)?;
                }
                serde_json::from_slice(&buffer)
            }

            /// Serialize the tagged value with the provided serialize function.
            /// Note: This does not create/use the Serializer object internally.
            pub fn serialize_wrapped<V, T>(tag_value: &T, value: &V) -> Result<Value>
            where
                V: Serialize + ?Sized,
                T: Serialize + ?Sized,
            {
                let mut buffer = Vec::with_capacity(128);
                {
                    let mut json_ser = serde_json::Serializer::new(&mut buffer);
                    external::serialize(&mut json_ser, tag_value, value)?;
                }
                serde_json::from_slice(&buffer)
            }
        }

        /// Functions for adjacent tagging using tuples.
        pub mod adj_tuple {
            use serde::Serialize;
            use serde_json::{self, Result, Value};
            use serde_tagged::ser::adj;


            /// Serialize the tagged value with an explicitly created
            /// serializer.
            pub fn serialize_with_serializer<V, T>(tag_value: &T, value: &V) -> Result<Value>
            where
                V: Serialize + ?Sized,
                T: Serialize + ?Sized,
            {
                let mut buffer = Vec::with_capacity(128);
                {
                    let mut json_ser = serde_json::Serializer::new(&mut buffer);
                    let tag_ser = adj::tuple::Serializer::new(&mut json_ser, tag_value);
                    value.serialize(tag_ser)?;
                }
                serde_json::from_slice(&buffer)
            }

            /// Serialize the tagged value with the provided serialize function.
            /// Note: This does not create/use the Serializer object internally.
            pub fn serialize_wrapped<V, T>(tag_value: &T, value: &V) -> Result<Value>
            where
                V: Serialize + ?Sized,
                T: Serialize + ?Sized,
            {
                let mut buffer = Vec::with_capacity(128);
                {
                    let mut json_ser = serde_json::Serializer::new(&mut buffer);
                    adj::tuple::serialize(&mut json_ser, tag_value, value)?;
                }
                serde_json::from_slice(&buffer)
            }
        }
    }
}

/// Functions for `serde_value`.
pub mod value {

    /// Functions for serialization.
    pub mod ser {

        /// Functions for external tagging.
        pub mod external {
            use serde::{Serialize, Serializer};
            use serde_tagged::ser::external;
            use serde_value::{to_value, SerializerError, Value};

            type Result<T> = ::std::result::Result<T, SerializerError>;


            /// Serialize the tagged value with an explicitly created
            /// serializer.
            pub fn serialize_with_serializer<V, T>(tag_value: &T, value: &V) -> Result<Value>
            where
                V: Serialize + ?Sized,
                T: Serialize + ?Sized,
            {
                struct Wrapper<'a, V: ?Sized + 'a, T: ?Sized + 'a> {
                    val: &'a V,
                    tag: &'a T,
                }

                impl<'a, T, V> Serialize for Wrapper<'a, T, V>
                where
                    T: Serialize + ?Sized,
                    V: Serialize + ?Sized,
                {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: Serializer,
                    {
                        let tag_ser = external::Serializer::new(serializer, self.tag);
                        self.val.serialize(tag_ser)
                    }
                }

                to_value(Wrapper {
                    tag: tag_value,
                    val: value,
                })
            }

            /// Serialize the tagged value with the provided serialize function.
            /// Note: This does not create/use the Serializer object internally.
            pub fn serialize_wrapped<V, T>(tag_value: &T, value: &V) -> Result<Value>
            where
                V: Serialize + ?Sized,
                T: Serialize + ?Sized,
            {
                struct Wrapper<'a, V: ?Sized + 'a, T: ?Sized + 'a> {
                    tag: &'a T,
                    val: &'a V,
                }

                impl<'a, T, V> Serialize for Wrapper<'a, T, V>
                where
                    T: Serialize + ?Sized,
                    V: Serialize + ?Sized,
                {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: Serializer,
                    {
                        external::serialize(serializer, self.tag, self.val)
                    }
                }

                to_value(Wrapper {
                    tag: tag_value,
                    val: value,
                })
            }
        }

        /// Functions for adjacent tagging using tuples.
        pub mod adj_tuple {
            use serde::{Serialize, Serializer};
            use serde_tagged::ser::adj;
            use serde_value::{to_value, SerializerError, Value};

            type Result<T> = ::std::result::Result<T, SerializerError>;


            /// Serialize the tagged value with an explicitly created
            /// serializer.
            pub fn serialize_with_serializer<V, T>(tag_value: &T, value: &V) -> Result<Value>
            where
                V: Serialize + ?Sized,
                T: Serialize + ?Sized,
            {
                struct Wrapper<'a, V: ?Sized + 'a, T: ?Sized + 'a> {
                    val: &'a V,
                    tag: &'a T,
                }

                impl<'a, T, V> Serialize for Wrapper<'a, T, V>
                where
                    T: Serialize + ?Sized,
                    V: Serialize + ?Sized,
                {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: Serializer,
                    {
                        let tag_ser = adj::tuple::Serializer::new(serializer, self.tag);
                        self.val.serialize(tag_ser)
                    }
                }

                to_value(Wrapper {
                    tag: tag_value,
                    val: value,
                })
            }

            /// Serialize the tagged value with the provided serialize function.
            /// Note: This does not create/use the Serializer object internally.
            pub fn serialize_wrapped<V, T>(tag_value: &T, value: &V) -> Result<Value>
            where
                V: Serialize + ?Sized,
                T: Serialize + ?Sized,
            {
                struct Wrapper<'a, V: ?Sized + 'a, T: ?Sized + 'a> {
                    tag: &'a T,
                    val: &'a V,
                }

                impl<'a, T, V> Serialize for Wrapper<'a, T, V>
                where
                    T: Serialize + ?Sized,
                    V: Serialize + ?Sized,
                {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: Serializer,
                    {
                        adj::tuple::serialize(serializer, self.tag, self.val)
                    }
                }

                to_value(Wrapper {
                    tag: tag_value,
                    val: value,
                })
            }
        }
    }
}
