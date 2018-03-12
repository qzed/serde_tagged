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

        /// Functions for adjacent tagging using maps.
        pub mod adj_map {
            use serde::Serialize;
            use serde_json::{self, Result, Value};
            use serde_tagged::ser::adj;


            /// Serialize the tagged value with an explicitly created
            /// serializer.
            pub fn serialize_with_serializer<Tk, Tv, Vk, V>(
                tag_key: &Tk,
                tag_value: &Tv,
                value_key: &Vk,
                value: &V,
            ) -> Result<Value>
            where
                Tk: Serialize + ?Sized,
                Tv: Serialize + ?Sized,
                Vk: Serialize + ?Sized,
                V: Serialize + ?Sized,
            {
                let mut buffer = Vec::with_capacity(128);
                {
                    let mut json_ser = serde_json::Serializer::new(&mut buffer);
                    let tag_ser =
                        adj::map::Serializer::new(&mut json_ser, tag_key, tag_value, value_key);
                    value.serialize(tag_ser)?;
                }
                serde_json::from_slice(&buffer)
            }

            /// Serialize the tagged value with the provided serialize function.
            /// Note: This does not create/use the Serializer object internally.
            pub fn serialize_wrapped<Tk, Tv, Vk, V>(
                tag_key: &Tk,
                tag_value: &Tv,
                value_key: &Vk,
                value: &V,
            ) -> Result<Value>
            where
                Tk: Serialize + ?Sized,
                Tv: Serialize + ?Sized,
                Vk: Serialize + ?Sized,
                V: Serialize + ?Sized,
            {
                let mut buffer = Vec::with_capacity(128);
                {
                    let mut json_ser = serde_json::Serializer::new(&mut buffer);
                    adj::map::serialize(&mut json_ser, tag_key, tag_value, value_key, value)?;
                }
                serde_json::from_slice(&buffer)
            }
        }

        /// Functions for adjacent tagging using structs.
        pub mod adj_struc {
            use serde::Serialize;
            use serde_json::{self, Result, Value};
            use serde_tagged::ser::adj;


            /// Serialize the tagged value with an explicitly created
            /// serializer.
            pub fn serialize_with_serializer<T, V>(
                tag_key: &'static str,
                tag_value: &T,
                value_key: &'static str,
                value: &V,
            ) -> Result<Value>
            where
                T: Serialize + ?Sized,
                V: Serialize + ?Sized,
            {
                let mut buffer = Vec::with_capacity(128);
                {
                    let mut json_ser = serde_json::Serializer::new(&mut buffer);
                    let tag_ser = adj::struc::Serializer::new(
                        &mut json_ser,
                        "Tagged",
                        tag_key,
                        tag_value,
                        value_key,
                    );
                    value.serialize(tag_ser)?;
                }
                serde_json::from_slice(&buffer)
            }

            /// Serialize the tagged value with the provided serialize function.
            /// Note: This does not create/use the Serializer object internally.
            pub fn serialize_wrapped<T, V>(
                tag_key: &'static str,
                tag_value: &T,
                value_key: &'static str,
                value: &V,
            ) -> Result<Value>
            where
                T: Serialize + ?Sized,
                V: Serialize + ?Sized,
            {
                let mut buffer = Vec::with_capacity(128);
                {
                    let mut json_ser = serde_json::Serializer::new(&mut buffer);
                    adj::struc::serialize(
                        &mut json_ser,
                        "Tagged",
                        tag_key,
                        tag_value,
                        value_key,
                        value,
                    )?;
                }
                serde_json::from_slice(&buffer)
            }
        }

        /// Functions for internal tagging.
        pub mod internal {
            use serde::Serialize;
            use serde_json::{self, Result, Value};
            use serde_tagged::ser::internal;


            /// Serialize the tagged value.
            pub fn serialize<T, V>(
                tag_key: &'static str,
                tag_value: &T,
                value: &V,
            ) -> Result<Value>
            where
                T: Serialize + ?Sized,
                V: Serialize + ?Sized,
            {
                let mut buffer = Vec::with_capacity(128);
                {
                    let mut json_ser = serde_json::Serializer::new(&mut buffer);
                    internal::serialize(
                        &mut json_ser,
                        tag_key,
                        tag_value,
                        value,
                    )?;
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

        /// Functions for adjacent tagging using maps.
        pub mod adj_map {
            use serde::{Serialize, Serializer};
            use serde_tagged::ser::adj;
            use serde_value::{to_value, SerializerError, Value};

            type Result<T> = ::std::result::Result<T, SerializerError>;


            /// Serialize the tagged value with an explicitly created
            /// serializer.
            pub fn serialize_with_serializer<Tk, Tv, Vk, V>(
                tag_key: &Tk,
                tag_value: &Tv,
                value_key: &Vk,
                value: &V,
            ) -> Result<Value>
            where
                Tk: Serialize + ?Sized,
                Tv: Serialize + ?Sized,
                Vk: Serialize + ?Sized,
                V: Serialize + ?Sized,
            {
                struct Wrapper<'a, Tk, Tv, Vk, V>
                where
                    Tk: Serialize + ?Sized + 'a,
                    Tv: Serialize + ?Sized + 'a,
                    Vk: Serialize + ?Sized + 'a,
                    V: Serialize + ?Sized + 'a,
                {
                    tag_key:   &'a Tk,
                    tag_value: &'a Tv,
                    value_key: &'a Vk,
                    value:     &'a V,
                }

                impl<'a, Tk, Tv, Vk, V> Serialize for Wrapper<'a, Tk, Tv, Vk, V>
                where
                    Tk: Serialize + ?Sized,
                    Tv: Serialize + ?Sized,
                    Vk: Serialize + ?Sized,
                    V: Serialize + ?Sized,
                {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: Serializer,
                    {
                        let tag_ser = adj::map::Serializer::new(
                            serializer,
                            self.tag_key,
                            self.tag_value,
                            self.value_key,
                        );
                        self.value.serialize(tag_ser)
                    }
                }

                to_value(Wrapper {
                    tag_key,
                    tag_value,
                    value_key,
                    value,
                })
            }

            /// Serialize the tagged value with the provided serialize function.
            /// Note: This does not create/use the Serializer object internally.
            pub fn serialize_wrapped<Tk, Tv, Vk, V>(
                tag_key: &Tk,
                tag_value: &Tv,
                value_key: &Vk,
                value: &V,
            ) -> Result<Value>
            where
                Tk: Serialize + ?Sized,
                Tv: Serialize + ?Sized,
                Vk: Serialize + ?Sized,
                V: Serialize + ?Sized,
            {
                struct Wrapper<'a, Tk, Tv, Vk, V>
                where
                    Tk: Serialize + ?Sized + 'a,
                    Tv: Serialize + ?Sized + 'a,
                    Vk: Serialize + ?Sized + 'a,
                    V: Serialize + ?Sized + 'a,
                {
                    tag_key:   &'a Tk,
                    tag_value: &'a Tv,
                    value_key: &'a Vk,
                    value:     &'a V,
                }

                impl<'a, Tk, Tv, Vk, V> Serialize for Wrapper<'a, Tk, Tv, Vk, V>
                where
                    Tk: Serialize + ?Sized,
                    Tv: Serialize + ?Sized,
                    Vk: Serialize + ?Sized,
                    V: Serialize + ?Sized,
                {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: Serializer,
                    {
                        adj::map::serialize(
                            serializer,
                            self.tag_key,
                            self.tag_value,
                            self.value_key,
                            self.value,
                        )
                    }
                }

                to_value(Wrapper {
                    tag_key,
                    tag_value,
                    value_key,
                    value,
                })
            }
        }

        /// Functions for adjacent tagging using structs.
        pub mod adj_struc {
            use serde::{Serialize, Serializer};
            use serde_tagged::ser::adj;
            use serde_value::{to_value, SerializerError, Value};

            type Result<T> = ::std::result::Result<T, SerializerError>;


            /// Serialize the tagged value with an explicitly created
            /// serializer.
            pub fn serialize_with_serializer<T, V>(
                tag_key: &'static str,
                tag_value: &T,
                value_key: &'static str,
                value: &V,
            ) -> Result<Value>
            where
                T: Serialize + ?Sized,
                V: Serialize + ?Sized,
            {
                struct Wrapper<'a, T, V>
                where
                    T: Serialize + ?Sized + 'a,
                    V: Serialize + ?Sized + 'a,
                {
                    tag_key:   &'static str,
                    tag_value: &'a T,
                    value_key: &'static str,
                    value:     &'a V,
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
                        let tag_ser = adj::struc::Serializer::new(
                            serializer,
                            "Tagged",
                            self.tag_key,
                            self.tag_value,
                            self.value_key,
                        );
                        self.value.serialize(tag_ser)
                    }
                }

                to_value(Wrapper {
                    tag_key,
                    tag_value,
                    value_key,
                    value,
                })
            }

            /// Serialize the tagged value with the provided serialize function.
            /// Note: This does not create/use the Serializer object internally.
            pub fn serialize_wrapped<T, V>(
                tag_key: &'static str,
                tag_value: &T,
                value_key: &'static str,
                value: &V,
            ) -> Result<Value>
            where
                T: Serialize + ?Sized,
                V: Serialize + ?Sized,
            {
                struct Wrapper<'a, T, V>
                where
                    T: Serialize + ?Sized + 'a,
                    V: Serialize + ?Sized + 'a,
                {
                    tag_key:   &'static str,
                    tag_value: &'a T,
                    value_key: &'static str,
                    value:     &'a V,
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
                        adj::struc::serialize(
                            serializer,
                            "Tagged",
                            self.tag_key,
                            self.tag_value,
                            self.value_key,
                            self.value,
                        )
                    }
                }

                to_value(Wrapper {
                    tag_key,
                    tag_value,
                    value_key,
                    value,
                })
            }
        }

        /// Functions for adjacent tagging using structs.
        pub mod internal {
            use serde::{Serialize, Serializer};
            use serde_tagged::ser::internal;
            use serde_value::{to_value, SerializerError, Value};

            type Result<T> = ::std::result::Result<T, SerializerError>;

            pub fn serialize<T, V>(
                tag_key: &'static str,
                tag_value: &T,
                value: &V,
            ) -> Result<Value>
            where
                T: Serialize + ?Sized,
                V: Serialize + ?Sized,
            {
                struct Wrapper<'a, T, V>
                where
                    T: Serialize + ?Sized + 'a,
                    V: Serialize + ?Sized + 'a,
                {
                    tag_key:   &'static str,
                    tag_value: &'a T,
                    value:     &'a V,
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
                        internal::serialize(
                            serializer,
                            self.tag_key,
                            self.tag_value,
                            self.value,
                        )
                    }
                }

                to_value(Wrapper {
                    tag_key,
                    tag_value,
                    value,
                })
            }
        }
    }
}
