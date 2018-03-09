//! Tests using `serde-value`.

#![cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]

extern crate serde;
extern crate serde_bytes;
extern crate serde_json;
extern crate serde_tagged;
extern crate serde_value;

#[macro_use]
extern crate serde_derive;


#[macro_use]
mod common;


/// Tests for serialization of tagged values.
mod ser {

    /// Tests for serialization of externally-tagged values.
    mod external {
        use serde_value::Value;

        /// Apply a string as external tag to the specified value.
        fn ts(tag: &str, value: Value) -> Value {
            Value::Map(map![Value::String(tag.to_owned()) => value])
        }

        /// Tests for serialization of externally-tagged using the
        /// wrapper-struct.
        mod wrapped {
            use super::ts;
            use common::types::*;
            use serde_bytes;
            use serde_value::Value::*;

            generate_tests_ser_1! {
                using: ::common::formats::value::ser::external::serialize_wrapped,

                { case: bool, tag: "<tag>", value: true, expect: ts("<tag>", Bool(true)) },

                { case: i8,  tag: "<tag>", value:     -56_i8,  expect: ts("<tag>",  I8(    -56)) },
                { case: i16, tag: "<tag>", value:    -197_i16, expect: ts("<tag>", I16(   -197)) },
                { case: i32, tag: "<tag>", value:   49206_i32, expect: ts("<tag>", I32(  49206)) },
                { case: i64, tag: "<tag>", value: -817696_i64, expect: ts("<tag>", I64(-817696)) },

                { case: u8,  tag: "<tag>", value:     234_u8,  expect: ts("<tag>",  U8(    234)) },
                { case: u16, tag: "<tag>", value:   25507_u16, expect: ts("<tag>", U16(  25507)) },
                { case: u32, tag: "<tag>", value: 2051984_u32, expect: ts("<tag>", U32(2051984)) },
                { case: u64, tag: "<tag>", value: 3331520_u64, expect: ts("<tag>", U64(3331520)) },

                { case: f32, tag: "<tag>", value: 2.0_f32, expect: ts("<tag>", F32(2.0)) },
                { case: f64, tag: "<tag>", value: 2.0_f64, expect: ts("<tag>", F64(2.0)) },

                {
                    case:   char,
                    tag:    "<tag>",
                    value:  'c',
                    expect: ts("<tag>", Char('c')),
                },{
                    case:   str,
                    tag:    "<tag>",
                    value:  "foobar",
                    expect: ts("<tag>", String("foobar".into())),
                },{
                    case:   bytes,
                    tag:    "<tag>",
                    value:  serde_bytes::Bytes::new(&[0, 1, 2, 3]),
                    expect: ts("<tag>", Bytes(vec![0, 1, 2, 3])),
                },

                {
                    case:   none,
                    tag:    "<tag>",
                    value:  None as ::std::option::Option<i32>,
                    expect: ts("<tag>", Option(None)),
                },{
                    case:   some,
                    tag:    "<tag>",
                    value:  Some(361_i32),
                    expect: ts("<tag>", Option(Some(Box::new(I32(361))))),
                },{
                    case:   unit,
                    tag:    "<tag>",
                    value:  (),
                    expect: ts("<tag>", Unit),
                },

                {
                    case:   tuple,
                    tag:    "<tag>",
                    value:  (1_i32, 2_i32, 3_i32),
                    expect: ts("<tag>", Seq(vec![I32(1), I32(2), I32(3)])),
                },{
                    case:   seq,
                    tag:    "<tag>",
                    value:  SerializeSeq(&vec![1_i32, 2_i32, 3_i32]),
                    expect: ts("<tag>", Seq(vec![I32(1), I32(2), I32(3)])),
                },{
                    case:   seq_len_hidden,
                    tag:    "<tag>",
                    value:  SerializeSeqLenHidden(&vec![1_i32, 2_i32, 3_i32]),
                    expect: ts("<tag>", Seq(vec![I32(1), I32(2), I32(3)])),
                },{
                    case:   map,
                    tag:    "<tag>",
                    value:  SerializeMap(&map![
                        String("a".to_owned()) => I32(1),
                        String("b".to_owned()) => I32(2),
                        String("c".to_owned()) => I32(3),
                    ]),
                    expect: ts("<tag>", Map(map![
                        String("a".to_owned()) => I32(1),
                        String("b".to_owned()) => I32(2),
                        String("c".to_owned()) => I32(3),
                    ])),
                },{
                    case:   map_len_hidden,
                    tag:    "<tag>",
                    value:  SerializeMapLenHidden(&map![
                        String("a".to_owned()) => I32(1),
                        String("b".to_owned()) => I32(2),
                        String("c".to_owned()) => I32(3),
                    ]),
                    expect: ts("<tag>", Map(map![
                        String("a".to_owned()) => I32(1),
                        String("b".to_owned()) => I32(2),
                        String("c".to_owned()) => I32(3),
                    ])),
                },

                {
                    case:   struct_unit,
                    tag:    "<tag>",
                    value:  UnitStruct,
                    expect: ts("<tag>", Unit),
                },{
                    case:   struct_newtype_primitive,
                    tag:    "<tag>",
                    value:  NewtypeStruct(42_i32),
                    expect: ts("<tag>", Newtype(Box::new(I32(42)))),
                },{
                    case:   struct_newtype_nonprimitive,
                    tag:    "<tag>",
                    value:  NewtypeStruct(Struct { foo: "bar" }),
                    expect: ts("<tag>", Newtype(Box::new(Map(map![
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])))),
                },{
                    case:   struct_tuple,
                    tag:    "<tag>",
                    value:  TupleStruct(1_i32, 2_i32, 3_i32, 4_i32),
                    expect: ts("<tag>", Seq(vec![I32(1), I32(2), I32(3), I32(4)])),
                },{
                    case:   struct_normal,
                    tag:    "<tag>",
                    value:  Struct { foo: "bar" },
                    expect: ts("<tag>", Map(map![
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])),
                },

                // TODO: tests for externally tagged enums
                //       missing due to https://github.com/arcnmx/serde-value/issues/18

                {
                    case:   enum_internal_unit,
                    tag:    "<tag>",
                    value:  EnumTaggedInternal::Unit,
                    expect: ts("<tag>", Map(map![
                        String("t".to_owned()) => String("Unit".to_owned()),
                    ])),
                },{
                    case:   enum_internal_newtype_nonprimitive,
                    tag:    "<tag>",
                    value:  EnumTaggedInternal::NewtypeC(Struct { foo: "bar" }),
                    expect: ts("<tag>", Map(map![
                        String("t".to_owned()) => String("NewtypeC".to_owned()),
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])),
                },{
                    case:   enum_internal_struct,
                    tag:    "<tag>",
                    value:  EnumTaggedInternal::Struct{ foo: "bar" },
                    expect: ts("<tag>", Map(map![
                        String("t".to_owned()) => String("Struct".to_owned()),
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])),
                },

                {
                    case:   enum_adjacent_unit,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::Unit,
                    expect: ts("<tag>", Map(map![
                        String("t".to_owned()) => String("Unit".to_owned())
                    ])),
                },{
                    case:   enum_adjacent_newtype_primitive,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::NewtypeP(42),
                    expect: ts("<tag>", Map(map![
                        String("t".to_owned()) => String("NewtypeP".to_owned()),
                        String("c".to_owned()) => I32(42),
                    ])),
                },{
                    case:   enum_adjacent_newtype_nonprimitive,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::NewtypeC(Struct { foo: "bar" }),
                    expect: ts("<tag>", Map(map![
                        String("t".to_owned()) => String("NewtypeC".to_owned()),
                        String("c".to_owned()) => Map(map![
                            String("foo".to_owned()) => String("bar".to_owned()),
                        ]),
                    ])),
                },{
                    case:   enum_adjacent_tuple,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::Tuple(3, 2, 1),
                    expect: ts("<tag>", Map(map![
                        String("t".to_owned()) => String("Tuple".to_owned()),
                        String("c".to_owned()) => Seq(vec![I32(3), I32(2), I32(1)]),
                    ])),
                },{
                    case:   enum_adjacent_struct,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::Struct{ foo: "bar" },
                    expect: ts("<tag>", Map(map![
                        String("t".to_owned()) => String("Struct".to_owned()),
                        String("c".to_owned()) => Map(map![
                            String("foo".to_owned()) => String("bar".to_owned()),
                        ]),
                    ])),
                },

                {
                    case:   enum_untagged_unit,
                    tag:    "<tag>",
                    value:  EnumUntagged::Unit,
                    expect: ts("<tag>", Unit),
                },{
                    case:   enum_untagged_newtype_primitive,
                    tag:    "<tag>",
                    value:  EnumUntagged::NewtypeP(42),
                    expect: ts("<tag>", I32(42)),
                },{
                    case:   enum_untagged_newtype_nonprimitive,
                    tag:    "<tag>",
                    value:  EnumUntagged::NewtypeC(Struct { foo: "bar" }),
                    expect: ts("<tag>", Map(map![
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])),
                },{
                    case:   enum_untagged_tuple,
                    tag:    "<tag>",
                    value:  EnumUntagged::Tuple(3, 2, 1),
                    expect: ts("<tag>", Seq(vec![I32(3), I32(2), I32(1)])),
                },{
                    case:   enum_untagged_struct,
                    tag:    "<tag>",
                    value:  EnumUntagged::Struct{ foo: "bar" },
                    expect: ts("<tag>", Map(map![
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])),
                },

                {
                    case:   collect_seq,
                    tag:    "<tag>",
                    value:  CollectSeq(vec![1_i32, 2_i32, 3_i32, 4_i32]),
                    expect: ts("<tag>", Seq(vec![I32(1), I32(2), I32(3), I32(4)])),
                },{
                    case:   collect_map,
                    tag:    "<tag>",
                    value:  CollectMap(map![
                        String("a".to_owned()) => I32(1),
                        String("b".to_owned()) => I32(2),
                        String("c".to_owned()) => I32(3),
                    ]),
                    expect: ts("<tag>", Map(map![
                        String("a".to_owned()) => I32(1),
                        String("b".to_owned()) => I32(2),
                        String("c".to_owned()) => I32(3),
                    ])),
                },{
                    case:   collect_str,
                    tag:    "<tag>",
                    value:  CollectStr("foobar"),
                    expect: ts("<tag>", String("foobar".into())),
                },
            }
        }

        /// Tests for serialization of externally-tagged using the Serializer.
        mod with_serializer {
            use super::ts;
            use common::types::*;
            use serde_bytes;
            use serde_value::Value::*;

            generate_tests_ser_1! {
                using: ::common::formats::value::ser::external::serialize_with_serializer,

                { case: bool, tag: "<tag>", value: true, expect: ts("<tag>", Bool(true)) },

                { case: i8,  tag: "<tag>", value:     -56_i8,  expect: ts("<tag>",  I8(    -56)) },
                { case: i16, tag: "<tag>", value:    -197_i16, expect: ts("<tag>", I16(   -197)) },
                { case: i32, tag: "<tag>", value:   49206_i32, expect: ts("<tag>", I32(  49206)) },
                { case: i64, tag: "<tag>", value: -817696_i64, expect: ts("<tag>", I64(-817696)) },

                { case: u8,  tag: "<tag>", value:     234_u8,  expect: ts("<tag>",  U8(    234)) },
                { case: u16, tag: "<tag>", value:   25507_u16, expect: ts("<tag>", U16(  25507)) },
                { case: u32, tag: "<tag>", value: 2051984_u32, expect: ts("<tag>", U32(2051984)) },
                { case: u64, tag: "<tag>", value: 3331520_u64, expect: ts("<tag>", U64(3331520)) },

                { case: f32, tag: "<tag>", value: 2.0_f32, expect: ts("<tag>", F32(2.0)) },
                { case: f64, tag: "<tag>", value: 2.0_f64, expect: ts("<tag>", F64(2.0)) },

                {
                    case:   char,
                    tag:    "<tag>",
                    value:  'c',
                    expect: ts("<tag>", Char('c')),
                },{
                    case:   str,
                    tag:    "<tag>",
                    value:  "foobar",
                    expect: ts("<tag>", String("foobar".into())),
                },{
                    case:   bytes,
                    tag:    "<tag>",
                    value:  serde_bytes::Bytes::new(&[0, 1, 2, 3]),
                    expect: ts("<tag>", Bytes(vec![0, 1, 2, 3])),
                },

                {
                    case:   none,
                    tag:    "<tag>",
                    value:  None as ::std::option::Option<i32>,
                    expect: ts("<tag>", Option(None)),
                },{
                    case:   some,
                    tag:    "<tag>",
                    value:  Some(361_i32),
                    expect: ts("<tag>", Option(Some(Box::new(I32(361))))),
                },{
                    case:   unit,
                    tag:    "<tag>",
                    value:  (),
                    expect: ts("<tag>", Unit),
                },

                {
                    case:   tuple,
                    tag:    "<tag>",
                    value:  (1_i32, 2_i32, 3_i32),
                    expect: ts("<tag>", Seq(vec![I32(1), I32(2), I32(3)])),
                },{
                    case:   seq,
                    tag:    "<tag>",
                    value:  SerializeSeq(&vec![1_i32, 2_i32, 3_i32]),
                    expect: ts("<tag>", Seq(vec![I32(1), I32(2), I32(3)])),
                },{
                    case:   seq_len_hidden,
                    tag:    "<tag>",
                    value:  SerializeSeqLenHidden(&vec![1_i32, 2_i32, 3_i32]),
                    expect: ts("<tag>", Seq(vec![I32(1), I32(2), I32(3)])),
                },{
                    case:   map,
                    tag:    "<tag>",
                    value:  SerializeMap(&map![
                        String("a".to_owned()) => I32(1),
                        String("b".to_owned()) => I32(2),
                        String("c".to_owned()) => I32(3),
                    ]),
                    expect: ts("<tag>", Map(map![
                        String("a".to_owned()) => I32(1),
                        String("b".to_owned()) => I32(2),
                        String("c".to_owned()) => I32(3),
                    ])),
                },{
                    case:   map_len_hidden,
                    tag:    "<tag>",
                    value:  SerializeMapLenHidden(&map![
                        String("a".to_owned()) => I32(1),
                        String("b".to_owned()) => I32(2),
                        String("c".to_owned()) => I32(3),
                    ]),
                    expect: ts("<tag>", Map(map![
                        String("a".to_owned()) => I32(1),
                        String("b".to_owned()) => I32(2),
                        String("c".to_owned()) => I32(3),
                    ])),
                },

                {
                    case:   struct_unit,
                    tag:    "<tag>",
                    value:  UnitStruct,
                    expect: ts("<tag>", Unit),
                },{
                    case:   struct_newtype_primitive,
                    tag:    "<tag>",
                    value:  NewtypeStruct(42_i32),
                    expect: ts("<tag>", Newtype(Box::new(I32(42)))),
                },{
                    case:   struct_newtype_nonprimitive,
                    tag:    "<tag>",
                    value:  NewtypeStruct(Struct { foo: "bar" }),
                    expect: ts("<tag>", Newtype(Box::new(Map(map![
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])))),
                },{
                    case:   struct_tuple,
                    tag:    "<tag>",
                    value:  TupleStruct(1_i32, 2_i32, 3_i32, 4_i32),
                    expect: ts("<tag>", Seq(vec![I32(1), I32(2), I32(3), I32(4)])),
                },{
                    case:   struct_normal,
                    tag:    "<tag>",
                    value:  Struct { foo: "bar" },
                    expect: ts("<tag>", Map(map![
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])),
                },

                // TODO: tests for externally tagged enums
                //       missing due to https://github.com/arcnmx/serde-value/issues/18

                {
                    case:   enum_internal_unit,
                    tag:    "<tag>",
                    value:  EnumTaggedInternal::Unit,
                    expect: ts("<tag>", Map(map![
                        String("t".to_owned()) => String("Unit".to_owned()),
                    ])),
                },{
                    case:   enum_internal_newtype_nonprimitive,
                    tag:    "<tag>",
                    value:  EnumTaggedInternal::NewtypeC(Struct { foo: "bar" }),
                    expect: ts("<tag>", Map(map![
                        String("t".to_owned()) => String("NewtypeC".to_owned()),
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])),
                },{
                    case:   enum_internal_struct,
                    tag:    "<tag>",
                    value:  EnumTaggedInternal::Struct{ foo: "bar" },
                    expect: ts("<tag>", Map(map![
                        String("t".to_owned()) => String("Struct".to_owned()),
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])),
                },

                {
                    case:   enum_adjacent_unit,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::Unit,
                    expect: ts("<tag>", Map(map![
                        String("t".to_owned()) => String("Unit".to_owned())
                    ])),
                },{
                    case:   enum_adjacent_newtype_primitive,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::NewtypeP(42),
                    expect: ts("<tag>", Map(map![
                        String("t".to_owned()) => String("NewtypeP".to_owned()),
                        String("c".to_owned()) => I32(42),
                    ])),
                },{
                    case:   enum_adjacent_newtype_nonprimitive,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::NewtypeC(Struct { foo: "bar" }),
                    expect: ts("<tag>", Map(map![
                        String("t".to_owned()) => String("NewtypeC".to_owned()),
                        String("c".to_owned()) => Map(map![
                            String("foo".to_owned()) => String("bar".to_owned()),
                        ]),
                    ])),
                },{
                    case:   enum_adjacent_tuple,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::Tuple(3, 2, 1),
                    expect: ts("<tag>", Map(map![
                        String("t".to_owned()) => String("Tuple".to_owned()),
                        String("c".to_owned()) => Seq(vec![I32(3), I32(2), I32(1)]),
                    ])),
                },{
                    case:   enum_adjacent_struct,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::Struct{ foo: "bar" },
                    expect: ts("<tag>", Map(map![
                        String("t".to_owned()) => String("Struct".to_owned()),
                        String("c".to_owned()) => Map(map![
                            String("foo".to_owned()) => String("bar".to_owned()),
                        ]),
                    ])),
                },

                {
                    case:   enum_untagged_unit,
                    tag:    "<tag>",
                    value:  EnumUntagged::Unit,
                    expect: ts("<tag>", Unit),
                },{
                    case:   enum_untagged_newtype_primitive,
                    tag:    "<tag>",
                    value:  EnumUntagged::NewtypeP(42),
                    expect: ts("<tag>", I32(42)),
                },{
                    case:   enum_untagged_newtype_nonprimitive,
                    tag:    "<tag>",
                    value:  EnumUntagged::NewtypeC(Struct { foo: "bar" }),
                    expect: ts("<tag>", Map(map![
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])),
                },{
                    case:   enum_untagged_tuple,
                    tag:    "<tag>",
                    value:  EnumUntagged::Tuple(3, 2, 1),
                    expect: ts("<tag>", Seq(vec![I32(3), I32(2), I32(1)])),
                },{
                    case:   enum_untagged_struct,
                    tag:    "<tag>",
                    value:  EnumUntagged::Struct{ foo: "bar" },
                    expect: ts("<tag>", Map(map![
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])),
                },

                {
                    case:   collect_seq,
                    tag:    "<tag>",
                    value:  CollectSeq(vec![1_i32, 2_i32, 3_i32, 4_i32]),
                    expect: ts("<tag>", Seq(vec![I32(1), I32(2), I32(3), I32(4)])),
                },{
                    case:   collect_map,
                    tag:    "<tag>",
                    value:  CollectMap(map![
                        String("a".to_owned()) => I32(1),
                        String("b".to_owned()) => I32(2),
                        String("c".to_owned()) => I32(3),
                    ]),
                    expect: ts("<tag>", Map(map![
                        String("a".to_owned()) => I32(1),
                        String("b".to_owned()) => I32(2),
                        String("c".to_owned()) => I32(3),
                    ])),
                },{
                    case:   collect_str,
                    tag:    "<tag>",
                    value:  CollectStr("foobar"),
                    expect: ts("<tag>", String("foobar".into())),
                },
            }
        }
    }

    /// Tests for serialization of tuple-based adjacently-tagged values.
    mod adj_tuple {
        use serde_value::Value;

        /// Apply a string as external tag to the specified value.
        fn ts(tag: &str, value: Value) -> Value {
            Value::Seq(vec![Value::String(tag.to_owned()), value])
        }

        /// Tests for serialization of tuple-based adjacently-tagged values
        /// using the wrapper-struct.
        mod wrapped {
            use super::ts;
            use common::types::*;
            use serde_bytes;
            use serde_value::Value::*;

            generate_tests_ser_1! {
                using: ::common::formats::value::ser::adj_tuple::serialize_wrapped,

                { case: bool, tag: "<tag>", value: true, expect: ts("<tag>", Bool(true)) },

                { case: i8,  tag: "<tag>", value:     -56_i8,  expect: ts("<tag>",  I8(    -56)) },
                { case: i16, tag: "<tag>", value:    -197_i16, expect: ts("<tag>", I16(   -197)) },
                { case: i32, tag: "<tag>", value:   49206_i32, expect: ts("<tag>", I32(  49206)) },
                { case: i64, tag: "<tag>", value: -817696_i64, expect: ts("<tag>", I64(-817696)) },

                { case: u8,  tag: "<tag>", value:     234_u8,  expect: ts("<tag>",  U8(    234)) },
                { case: u16, tag: "<tag>", value:   25507_u16, expect: ts("<tag>", U16(  25507)) },
                { case: u32, tag: "<tag>", value: 2051984_u32, expect: ts("<tag>", U32(2051984)) },
                { case: u64, tag: "<tag>", value: 3331520_u64, expect: ts("<tag>", U64(3331520)) },

                { case: f32, tag: "<tag>", value: 2.0_f32, expect: ts("<tag>", F32(2.0)) },
                { case: f64, tag: "<tag>", value: 2.0_f64, expect: ts("<tag>", F64(2.0)) },

                {
                    case:   char,
                    tag:    "<tag>",
                    value:  'c',
                    expect: ts("<tag>", Char('c')),
                },{
                    case:   str,
                    tag:    "<tag>",
                    value:  "foobar",
                    expect: ts("<tag>", String("foobar".into())),
                },{
                    case:   bytes,
                    tag:    "<tag>",
                    value:  serde_bytes::Bytes::new(&[0, 1, 2, 3]),
                    expect: ts("<tag>", Bytes(vec![0, 1, 2, 3])),
                },

                {
                    case:   none,
                    tag:    "<tag>",
                    value:  None as ::std::option::Option<i32>,
                    expect: ts("<tag>", Option(None)),
                },{
                    case:   some,
                    tag:    "<tag>",
                    value:  Some(361_i32),
                    expect: ts("<tag>", Option(Some(Box::new(I32(361))))),
                },{
                    case:   unit,
                    tag:    "<tag>",
                    value:  (),
                    expect: ts("<tag>", Unit),
                },

                {
                    case:   tuple,
                    tag:    "<tag>",
                    value:  (1_i32, 2_i32, 3_i32),
                    expect: ts("<tag>", Seq(vec![I32(1), I32(2), I32(3)])),
                },{
                    case:   seq,
                    tag:    "<tag>",
                    value:  SerializeSeq(&vec![1_i32, 2_i32, 3_i32]),
                    expect: ts("<tag>", Seq(vec![I32(1), I32(2), I32(3)])),
                },{
                    case:   seq_len_hidden,
                    tag:    "<tag>",
                    value:  SerializeSeqLenHidden(&vec![1_i32, 2_i32, 3_i32]),
                    expect: ts("<tag>", Seq(vec![I32(1), I32(2), I32(3)])),
                },{
                    case:   map,
                    tag:    "<tag>",
                    value:  SerializeMap(&map![
                        String("a".to_owned()) => I32(1),
                        String("b".to_owned()) => I32(2),
                        String("c".to_owned()) => I32(3),
                    ]),
                    expect: ts("<tag>", Map(map![
                        String("a".to_owned()) => I32(1),
                        String("b".to_owned()) => I32(2),
                        String("c".to_owned()) => I32(3),
                    ])),
                },{
                    case:   map_len_hidden,
                    tag:    "<tag>",
                    value:  SerializeMapLenHidden(&map![
                        String("a".to_owned()) => I32(1),
                        String("b".to_owned()) => I32(2),
                        String("c".to_owned()) => I32(3),
                    ]),
                    expect: ts("<tag>", Map(map![
                        String("a".to_owned()) => I32(1),
                        String("b".to_owned()) => I32(2),
                        String("c".to_owned()) => I32(3),
                    ])),
                },

                {
                    case:   struct_unit,
                    tag:    "<tag>",
                    value:  UnitStruct,
                    expect: ts("<tag>", Unit),
                },{
                    case:   struct_newtype_primitive,
                    tag:    "<tag>",
                    value:  NewtypeStruct(42_i32),
                    expect: ts("<tag>", Newtype(Box::new(I32(42)))),
                },{
                    case:   struct_newtype_nonprimitive,
                    tag:    "<tag>",
                    value:  NewtypeStruct(Struct { foo: "bar" }),
                    expect: ts("<tag>", Newtype(Box::new(Map(map![
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])))),
                },{
                    case:   struct_tuple,
                    tag:    "<tag>",
                    value:  TupleStruct(1_i32, 2_i32, 3_i32, 4_i32),
                    expect: ts("<tag>", Seq(vec![I32(1), I32(2), I32(3), I32(4)])),
                },{
                    case:   struct_normal,
                    tag:    "<tag>",
                    value:  Struct { foo: "bar" },
                    expect: ts("<tag>", Map(map![
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])),
                },

                // TODO: tests for externally tagged enums
                //       missing due to https://github.com/arcnmx/serde-value/issues/18

                {
                    case:   enum_internal_unit,
                    tag:    "<tag>",
                    value:  EnumTaggedInternal::Unit,
                    expect: ts("<tag>", Map(map![
                        String("t".to_owned()) => String("Unit".to_owned()),
                    ])),
                },{
                    case:   enum_internal_newtype_nonprimitive,
                    tag:    "<tag>",
                    value:  EnumTaggedInternal::NewtypeC(Struct { foo: "bar" }),
                    expect: ts("<tag>", Map(map![
                        String("t".to_owned()) => String("NewtypeC".to_owned()),
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])),
                },{
                    case:   enum_internal_struct,
                    tag:    "<tag>",
                    value:  EnumTaggedInternal::Struct{ foo: "bar" },
                    expect: ts("<tag>", Map(map![
                        String("t".to_owned()) => String("Struct".to_owned()),
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])),
                },

                {
                    case:   enum_adjacent_unit,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::Unit,
                    expect: ts("<tag>", Map(map![
                        String("t".to_owned()) => String("Unit".to_owned())
                    ])),
                },{
                    case:   enum_adjacent_newtype_primitive,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::NewtypeP(42),
                    expect: ts("<tag>", Map(map![
                        String("t".to_owned()) => String("NewtypeP".to_owned()),
                        String("c".to_owned()) => I32(42),
                    ])),
                },{
                    case:   enum_adjacent_newtype_nonprimitive,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::NewtypeC(Struct { foo: "bar" }),
                    expect: ts("<tag>", Map(map![
                        String("t".to_owned()) => String("NewtypeC".to_owned()),
                        String("c".to_owned()) => Map(map![
                            String("foo".to_owned()) => String("bar".to_owned()),
                        ]),
                    ])),
                },{
                    case:   enum_adjacent_tuple,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::Tuple(3, 2, 1),
                    expect: ts("<tag>", Map(map![
                        String("t".to_owned()) => String("Tuple".to_owned()),
                        String("c".to_owned()) => Seq(vec![I32(3), I32(2), I32(1)]),
                    ])),
                },{
                    case:   enum_adjacent_struct,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::Struct{ foo: "bar" },
                    expect: ts("<tag>", Map(map![
                        String("t".to_owned()) => String("Struct".to_owned()),
                        String("c".to_owned()) => Map(map![
                            String("foo".to_owned()) => String("bar".to_owned()),
                        ]),
                    ])),
                },

                {
                    case:   enum_untagged_unit,
                    tag:    "<tag>",
                    value:  EnumUntagged::Unit,
                    expect: ts("<tag>", Unit),
                },{
                    case:   enum_untagged_newtype_primitive,
                    tag:    "<tag>",
                    value:  EnumUntagged::NewtypeP(42),
                    expect: ts("<tag>", I32(42)),
                },{
                    case:   enum_untagged_newtype_nonprimitive,
                    tag:    "<tag>",
                    value:  EnumUntagged::NewtypeC(Struct { foo: "bar" }),
                    expect: ts("<tag>", Map(map![
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])),
                },{
                    case:   enum_untagged_tuple,
                    tag:    "<tag>",
                    value:  EnumUntagged::Tuple(3, 2, 1),
                    expect: ts("<tag>", Seq(vec![I32(3), I32(2), I32(1)])),
                },{
                    case:   enum_untagged_struct,
                    tag:    "<tag>",
                    value:  EnumUntagged::Struct{ foo: "bar" },
                    expect: ts("<tag>", Map(map![
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])),
                },

                {
                    case:   collect_seq,
                    tag:    "<tag>",
                    value:  CollectSeq(vec![1_i32, 2_i32, 3_i32, 4_i32]),
                    expect: ts("<tag>", Seq(vec![I32(1), I32(2), I32(3), I32(4)])),
                },{
                    case:   collect_map,
                    tag:    "<tag>",
                    value:  CollectMap(map![
                        String("a".to_owned()) => I32(1),
                        String("b".to_owned()) => I32(2),
                        String("c".to_owned()) => I32(3),
                    ]),
                    expect: ts("<tag>", Map(map![
                        String("a".to_owned()) => I32(1),
                        String("b".to_owned()) => I32(2),
                        String("c".to_owned()) => I32(3),
                    ])),
                },{
                    case:   collect_str,
                    tag:    "<tag>",
                    value:  CollectStr("foobar"),
                    expect: ts("<tag>", String("foobar".into())),
                },
            }
        }

        /// Tests for serialization of tuple-based adjacently-tagged values
        /// using the Serializer.
        mod with_serializer {
            use super::ts;
            use common::types::*;
            use serde_bytes;
            use serde_value::Value::*;

            generate_tests_ser_1! {
                using: ::common::formats::value::ser::adj_tuple::serialize_with_serializer,

                { case: bool, tag: "<tag>", value: true, expect: ts("<tag>", Bool(true)) },

                { case: i8,  tag: "<tag>", value:     -56_i8,  expect: ts("<tag>",  I8(    -56)) },
                { case: i16, tag: "<tag>", value:    -197_i16, expect: ts("<tag>", I16(   -197)) },
                { case: i32, tag: "<tag>", value:   49206_i32, expect: ts("<tag>", I32(  49206)) },
                { case: i64, tag: "<tag>", value: -817696_i64, expect: ts("<tag>", I64(-817696)) },

                { case: u8,  tag: "<tag>", value:     234_u8,  expect: ts("<tag>",  U8(    234)) },
                { case: u16, tag: "<tag>", value:   25507_u16, expect: ts("<tag>", U16(  25507)) },
                { case: u32, tag: "<tag>", value: 2051984_u32, expect: ts("<tag>", U32(2051984)) },
                { case: u64, tag: "<tag>", value: 3331520_u64, expect: ts("<tag>", U64(3331520)) },

                { case: f32, tag: "<tag>", value: 2.0_f32, expect: ts("<tag>", F32(2.0)) },
                { case: f64, tag: "<tag>", value: 2.0_f64, expect: ts("<tag>", F64(2.0)) },

                {
                    case:   char,
                    tag:    "<tag>",
                    value:  'c',
                    expect: ts("<tag>", Char('c')),
                },{
                    case:   str,
                    tag:    "<tag>",
                    value:  "foobar",
                    expect: ts("<tag>", String("foobar".into())),
                },{
                    case:   bytes,
                    tag:    "<tag>",
                    value:  serde_bytes::Bytes::new(&[0, 1, 2, 3]),
                    expect: ts("<tag>", Bytes(vec![0, 1, 2, 3])),
                },

                {
                    case:   none,
                    tag:    "<tag>",
                    value:  None as ::std::option::Option<i32>,
                    expect: ts("<tag>", Option(None)),
                },{
                    case:   some,
                    tag:    "<tag>",
                    value:  Some(361_i32),
                    expect: ts("<tag>", Option(Some(Box::new(I32(361))))),
                },{
                    case:   unit,
                    tag:    "<tag>",
                    value:  (),
                    expect: ts("<tag>", Unit),
                },

                {
                    case:   tuple,
                    tag:    "<tag>",
                    value:  (1_i32, 2_i32, 3_i32),
                    expect: ts("<tag>", Seq(vec![I32(1), I32(2), I32(3)])),
                },{
                    case:   seq,
                    tag:    "<tag>",
                    value:  SerializeSeq(&vec![1_i32, 2_i32, 3_i32]),
                    expect: ts("<tag>", Seq(vec![I32(1), I32(2), I32(3)])),
                },{
                    case:   seq_len_hidden,
                    tag:    "<tag>",
                    value:  SerializeSeqLenHidden(&vec![1_i32, 2_i32, 3_i32]),
                    expect: ts("<tag>", Seq(vec![I32(1), I32(2), I32(3)])),
                },{
                    case:   map,
                    tag:    "<tag>",
                    value:  SerializeMap(&map![
                        String("a".to_owned()) => I32(1),
                        String("b".to_owned()) => I32(2),
                        String("c".to_owned()) => I32(3),
                    ]),
                    expect: ts("<tag>", Map(map![
                        String("a".to_owned()) => I32(1),
                        String("b".to_owned()) => I32(2),
                        String("c".to_owned()) => I32(3),
                    ])),
                },{
                    case:   map_len_hidden,
                    tag:    "<tag>",
                    value:  SerializeMapLenHidden(&map![
                        String("a".to_owned()) => I32(1),
                        String("b".to_owned()) => I32(2),
                        String("c".to_owned()) => I32(3),
                    ]),
                    expect: ts("<tag>", Map(map![
                        String("a".to_owned()) => I32(1),
                        String("b".to_owned()) => I32(2),
                        String("c".to_owned()) => I32(3),
                    ])),
                },

                {
                    case:   struct_unit,
                    tag:    "<tag>",
                    value:  UnitStruct,
                    expect: ts("<tag>", Unit),
                },{
                    case:   struct_newtype_primitive,
                    tag:    "<tag>",
                    value:  NewtypeStruct(42_i32),
                    expect: ts("<tag>", Newtype(Box::new(I32(42)))),
                },{
                    case:   struct_newtype_nonprimitive,
                    tag:    "<tag>",
                    value:  NewtypeStruct(Struct { foo: "bar" }),
                    expect: ts("<tag>", Newtype(Box::new(Map(map![
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])))),
                },{
                    case:   struct_tuple,
                    tag:    "<tag>",
                    value:  TupleStruct(1_i32, 2_i32, 3_i32, 4_i32),
                    expect: ts("<tag>", Seq(vec![I32(1), I32(2), I32(3), I32(4)])),
                },{
                    case:   struct_normal,
                    tag:    "<tag>",
                    value:  Struct { foo: "bar" },
                    expect: ts("<tag>", Map(map![
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])),
                },

                // TODO: tests for externally tagged enums
                //       missing due to https://github.com/arcnmx/serde-value/issues/18

                {
                    case:   enum_internal_unit,
                    tag:    "<tag>",
                    value:  EnumTaggedInternal::Unit,
                    expect: ts("<tag>", Map(map![
                        String("t".to_owned()) => String("Unit".to_owned()),
                    ])),
                },{
                    case:   enum_internal_newtype_nonprimitive,
                    tag:    "<tag>",
                    value:  EnumTaggedInternal::NewtypeC(Struct { foo: "bar" }),
                    expect: ts("<tag>", Map(map![
                        String("t".to_owned()) => String("NewtypeC".to_owned()),
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])),
                },{
                    case:   enum_internal_struct,
                    tag:    "<tag>",
                    value:  EnumTaggedInternal::Struct{ foo: "bar" },
                    expect: ts("<tag>", Map(map![
                        String("t".to_owned()) => String("Struct".to_owned()),
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])),
                },

                {
                    case:   enum_adjacent_unit,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::Unit,
                    expect: ts("<tag>", Map(map![
                        String("t".to_owned()) => String("Unit".to_owned())
                    ])),
                },{
                    case:   enum_adjacent_newtype_primitive,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::NewtypeP(42),
                    expect: ts("<tag>", Map(map![
                        String("t".to_owned()) => String("NewtypeP".to_owned()),
                        String("c".to_owned()) => I32(42),
                    ])),
                },{
                    case:   enum_adjacent_newtype_nonprimitive,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::NewtypeC(Struct { foo: "bar" }),
                    expect: ts("<tag>", Map(map![
                        String("t".to_owned()) => String("NewtypeC".to_owned()),
                        String("c".to_owned()) => Map(map![
                            String("foo".to_owned()) => String("bar".to_owned()),
                        ]),
                    ])),
                },{
                    case:   enum_adjacent_tuple,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::Tuple(3, 2, 1),
                    expect: ts("<tag>", Map(map![
                        String("t".to_owned()) => String("Tuple".to_owned()),
                        String("c".to_owned()) => Seq(vec![I32(3), I32(2), I32(1)]),
                    ])),
                },{
                    case:   enum_adjacent_struct,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::Struct{ foo: "bar" },
                    expect: ts("<tag>", Map(map![
                        String("t".to_owned()) => String("Struct".to_owned()),
                        String("c".to_owned()) => Map(map![
                            String("foo".to_owned()) => String("bar".to_owned()),
                        ]),
                    ])),
                },

                {
                    case:   enum_untagged_unit,
                    tag:    "<tag>",
                    value:  EnumUntagged::Unit,
                    expect: ts("<tag>", Unit),
                },{
                    case:   enum_untagged_newtype_primitive,
                    tag:    "<tag>",
                    value:  EnumUntagged::NewtypeP(42),
                    expect: ts("<tag>", I32(42)),
                },{
                    case:   enum_untagged_newtype_nonprimitive,
                    tag:    "<tag>",
                    value:  EnumUntagged::NewtypeC(Struct { foo: "bar" }),
                    expect: ts("<tag>", Map(map![
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])),
                },{
                    case:   enum_untagged_tuple,
                    tag:    "<tag>",
                    value:  EnumUntagged::Tuple(3, 2, 1),
                    expect: ts("<tag>", Seq(vec![I32(3), I32(2), I32(1)])),
                },{
                    case:   enum_untagged_struct,
                    tag:    "<tag>",
                    value:  EnumUntagged::Struct{ foo: "bar" },
                    expect: ts("<tag>", Map(map![
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])),
                },

                {
                    case:   collect_seq,
                    tag:    "<tag>",
                    value:  CollectSeq(vec![1_i32, 2_i32, 3_i32, 4_i32]),
                    expect: ts("<tag>", Seq(vec![I32(1), I32(2), I32(3), I32(4)])),
                },{
                    case:   collect_map,
                    tag:    "<tag>",
                    value:  CollectMap(map!["a" => 1_i32, "b" => 2_i32, "c" => 3_i32]),
                    expect: ts("<tag>", Map(map![
                        String("a".to_owned()) => I32(1),
                        String("b".to_owned()) => I32(2),
                        String("c".to_owned()) => I32(3),
                    ])),
                },{
                    case:   collect_str,
                    tag:    "<tag>",
                    value:  CollectStr("foobar"),
                    expect: ts("<tag>", String("foobar".into())),
                },
            }
        }
    }

    /// Tests for serialization of map-based adjacently-tagged values.
    mod adj_map {
        use serde_value::Value;

        /// Apply a string as external tag to the specified value.
        fn ts(
            tag_key: &'static str,
            tag_value: &'static str,
            value_key: &'static str,
            value: Value,
        ) -> Value {
            Value::Map(map![
                Value::String(tag_key.to_owned()) => Value::String(tag_value.to_owned()),
                Value::String(value_key.to_owned()) => value,
            ])
        }

        /// Tests for serialization of map-based adjacently-tagged values
        /// using the wrapper-struct.
        mod wrapped {
            use super::ts;
            use common::types::*;
            use serde_bytes;
            use serde_value::Value::*;
            use std::option;

            generate_tests_ser_3! {
                using: ::common::formats::value::ser::adj_map::serialize_wrapped,

                {
                    case:   bool,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  true,
                    expect: ts("t", "<tag>", "c", Bool(true)),
                },

                {
                    case:   i8,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  -56_i8,
                    expect: ts("t", "<tag>", "c", I8(-56)),
                },{
                    case:   i16,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  -197_i16,
                    expect: ts("t", "<tag>", "c", I16(-197)),
                },{
                    case:   i32,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  49206_i32,
                    expect: ts("t", "<tag>", "c", I32(49206)),
                },{
                    case:   i64,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  -817696_i64,
                    expect: ts("t", "<tag>", "c", I64(-817696)),
                },

                {
                    case:   u8,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  234_u8,
                    expect: ts("t", "<tag>", "c", U8(234))
                },{
                    case:   u16,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  25507_u16,
                    expect: ts("t", "<tag>", "c", U16(25507))
                },{
                    case:   u32,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  2051984_u32,
                    expect: ts("t", "<tag>", "c", U32(2051984))
                },{
                    case:   u64,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  3331520_u64,
                    expect: ts("t", "<tag>", "c", U64(3331520))
                },

                {
                    case:   f32,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  2.0_f32,
                    expect: ts("t", "<tag>", "c", F32(2.0)),
                },{
                    case:   f64,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  2.0_f64,
                    expect: ts("t", "<tag>", "c", F64(2.0)),
                },

                {
                    case:   char,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  'c',
                    expect: ts("t", "<tag>", "c", Char('c')),
                },{
                    case:   str,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  "foobar",
                    expect: ts("t", "<tag>", "c", String("foobar".to_owned())),
                },

                {
                    case:   bytes,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  serde_bytes::Bytes::new(&[0, 1, 2, 3]),
                    expect: ts("t", "<tag>", "c", Bytes(vec![0, 1, 2, 3])),
                },

                {
                    case:   none,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  option::Option::None::<i32>,
                    expect: ts("t", "<tag>", "c", Option(option::Option::None)),
                },{
                    case:   some,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  option::Option::Some(361_i32),
                    expect: ts("t", "<tag>", "c", Option(option::Option::Some(Box::new(I32(361))))),
                },{
                    case:   unit,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  (),
                    expect: ts("t", "<tag>", "c", Unit),
                },

                {
                    case:   tuple,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  (1_i32, 2_i32, 3_i32),
                    expect: ts("t", "<tag>", "c", Seq(vec![I32(1), I32(2), I32(3)])),
                },{
                    case:   seq,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  SerializeSeq(&vec![1_i32, 2_i32, 3_i32]),
                    expect: ts("t", "<tag>", "c", Seq(vec![I32(1), I32(2), I32(3)])),
                },{
                    case:   seq_len_hidden,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  SerializeSeqLenHidden(&vec![1_i32, 2_i32, 3_i32]),
                    expect: ts("t", "<tag>", "c", Seq(vec![I32(1), I32(2), I32(3)])),
                },{
                    case:   map,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  SerializeMap(&map!["a" => 1_i32, "b" => 2_i32, "c" => 3_i32]),
                    expect: ts("t", "<tag>", "c", Map(map![
                        String("a".to_owned()) => I32(1),
                        String("b".to_owned()) => I32(2),
                        String("c".to_owned()) => I32(3),
                    ])),
                },{
                    case:   map_len_hidden,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  SerializeMapLenHidden(&map!["a" => 1_i32, "b" => 2_i32, "c" => 3_i32]),
                    expect: ts("t", "<tag>", "c", Map(map!{
                        String("a".to_owned()) => I32(1),
                        String("b".to_owned()) => I32(2),
                        String("c".to_owned()) => I32(3),
                    })),
                },

                {
                    case:   struct_unit,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  UnitStruct,
                    expect: ts("t", "<tag>", "c", Unit),
                },{
                    case:   struct_newtype_primitive,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  NewtypeStruct(42),
                    expect: ts("t", "<tag>", "c", Newtype(Box::new(I32(42)))),
                },{
                    case:   struct_newtype_nonprimitive,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  NewtypeStruct(Struct { foo: "bar" }),
                    expect: ts("t", "<tag>", "c", Newtype(Box::new(Map(map![
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])))),
                },{
                    case:   struct_tuple,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  TupleStruct(1, 2, 3, 4),
                    expect: ts("t", "<tag>", "c", Seq(vec![I32(1), I32(2), I32(3), I32(4)])),
                },{
                    case:   struct_normal,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  Struct { foo: "bar" },
                    expect: ts("t", "<tag>", "c", Map(map![
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])),
                },

                // TODO: tests for externally tagged enums
                //       missing due to https://github.com/arcnmx/serde-value/issues/18

                {
                    case:   enum_internal_unit,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedInternal::Unit,
                    expect: ts("t", "<tag>", "c", Map(map![
                        String("t".to_owned()) => String("Unit".to_owned()),
                    ])),
                },{
                    case:   enum_internal_newtype_nonprimitive,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedInternal::NewtypeC(Struct { foo: "bar" }),
                    expect: ts("t", "<tag>", "c", Map(map![
                        String("t".to_owned()) => String("NewtypeC".to_owned()),
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])),
                },{
                    case:   enum_internal_struct,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedInternal::Struct{ foo: "bar" },
                    expect: ts("t", "<tag>", "c", Map(map![
                        String("t".to_owned()) => String("Struct".to_owned()),
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])),
                },

                {
                    case:   enum_adjacent_unit,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedAdjacent::Unit,
                    expect: ts("t", "<tag>", "c", Map(map![
                        String("t".to_owned()) => String("Unit".to_owned())
                    ])),
                },{
                    case:   enum_adjacent_newtype_primitive,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedAdjacent::NewtypeP(42),
                    expect: ts("t", "<tag>", "c", Map(map![
                        String("t".to_owned()) => String("NewtypeP".to_owned()),
                        String("c".to_owned()) => I32(42),
                    ])),
                },{
                    case:   enum_adjacent_newtype_nonprimitive,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedAdjacent::NewtypeC(Struct { foo: "bar" }),
                    expect: ts("t", "<tag>", "c", Map(map![
                        String("t".to_owned()) => String("NewtypeC".to_owned()),
                        String("c".to_owned()) => Map(map![
                            String("foo".to_owned()) => String("bar".to_owned()),
                        ]),
                    ])),
                },{
                    case:   enum_adjacent_tuple,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedAdjacent::Tuple(3, 2, 1),
                    expect: ts("t", "<tag>", "c", Map(map![
                        String("t".to_owned()) => String("Tuple".to_owned()),
                        String("c".to_owned()) => Seq(vec![I32(3), I32(2), I32(1)]),
                    ])),
                },{
                    case:   enum_adjacent_struct,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedAdjacent::Struct{ foo: "bar" },
                    expect: ts("t", "<tag>", "c", Map(map![
                        String("t".to_owned()) => String("Struct".to_owned()),
                        String("c".to_owned()) => Map(map![
                            String("foo".to_owned()) => String("bar".to_owned()),
                        ]),
                    ])),
                },

                {
                    case:   enum_untagged_unit,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumUntagged::Unit,
                    expect: ts("t", "<tag>", "c", Unit),
                },{
                    case:   enum_untagged_newtype_primitive,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumUntagged::NewtypeP(42),
                    expect: ts("t", "<tag>", "c", I32(42)),
                },{
                    case:   enum_untagged_newtype_nonprimitive,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumUntagged::NewtypeC(Struct { foo: "bar" }),
                    expect: ts("t", "<tag>", "c", Map(map![
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])),
                },{
                    case:   enum_untagged_tuple,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumUntagged::Tuple(3, 2, 1),
                    expect: ts("t", "<tag>", "c", Seq(vec![I32(3), I32(2), I32(1)])),
                },{
                    case:   enum_untagged_struct,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumUntagged::Struct{ foo: "bar" },
                    expect: ts("t", "<tag>", "c", Map(map![
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])),
                },

                {
                    case:   collect_seq,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  CollectSeq(vec![1_i32, 2_i32, 3_i32, 4_i32]),
                    expect: ts("t", "<tag>", "c", Seq(vec![I32(1), I32(2), I32(3), I32(4)])),
                },{
                    case:   collect_map,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  CollectMap(map!["a" => 1_i32, "b" => 2_i32, "c" => 3_i32]),
                    expect: ts("t", "<tag>", "c", Map(map![
                        String("a".to_owned()) => I32(1),
                        String("b".to_owned()) => I32(2),
                        String("c".to_owned()) => I32(3),
                    ])),
                },{
                    case:   collect_str,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  CollectStr("foobar"),
                    expect: ts("t", "<tag>", "c", String("foobar".into())),
                },
            }
        }

        /// Tests for serialization of map-based adjacently-tagged values
        /// using the Serializer.
        mod with_serializer {
            use super::ts;
            use common::types::*;
            use serde_bytes;
            use serde_value::Value::*;
            use std::option;

            generate_tests_ser_3! {
                using: ::common::formats::value::ser::adj_map::serialize_with_serializer,

                {
                    case:   bool,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  true,
                    expect: ts("t", "<tag>", "c", Bool(true)),
                },

                {
                    case:   i8,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  -56_i8,
                    expect: ts("t", "<tag>", "c", I8(-56)),
                },{
                    case:   i16,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  -197_i16,
                    expect: ts("t", "<tag>", "c", I16(-197)),
                },{
                    case:   i32,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  49206_i32,
                    expect: ts("t", "<tag>", "c", I32(49206)),
                },{
                    case:   i64,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  -817696_i64,
                    expect: ts("t", "<tag>", "c", I64(-817696)),
                },

                {
                    case:   u8,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  234_u8,
                    expect: ts("t", "<tag>", "c", U8(234))
                },{
                    case:   u16,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  25507_u16,
                    expect: ts("t", "<tag>", "c", U16(25507))
                },{
                    case:   u32,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  2051984_u32,
                    expect: ts("t", "<tag>", "c", U32(2051984))
                },{
                    case:   u64,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  3331520_u64,
                    expect: ts("t", "<tag>", "c", U64(3331520))
                },

                {
                    case:   f32,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  2.0_f32,
                    expect: ts("t", "<tag>", "c", F32(2.0)),
                },{
                    case:   f64,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  2.0_f64,
                    expect: ts("t", "<tag>", "c", F64(2.0)),
                },

                {
                    case:   char,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  'c',
                    expect: ts("t", "<tag>", "c", Char('c')),
                },{
                    case:   str,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  "foobar",
                    expect: ts("t", "<tag>", "c", String("foobar".to_owned())),
                },

                {
                    case:   bytes,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  serde_bytes::Bytes::new(&[0, 1, 2, 3]),
                    expect: ts("t", "<tag>", "c", Bytes(vec![0, 1, 2, 3])),
                },

                {
                    case:   none,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  option::Option::None::<i32>,
                    expect: ts("t", "<tag>", "c", Option(option::Option::None)),
                },{
                    case:   some,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  option::Option::Some(361_i32),
                    expect: ts("t", "<tag>", "c", Option(option::Option::Some(Box::new(I32(361))))),
                },{
                    case:   unit,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  (),
                    expect: ts("t", "<tag>", "c", Unit),
                },

                {
                    case:   tuple,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  (1_i32, 2_i32, 3_i32),
                    expect: ts("t", "<tag>", "c", Seq(vec![I32(1), I32(2), I32(3)])),
                },{
                    case:   seq,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  SerializeSeq(&vec![1_i32, 2_i32, 3_i32]),
                    expect: ts("t", "<tag>", "c", Seq(vec![I32(1), I32(2), I32(3)])),
                },{
                    case:   seq_len_hidden,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  SerializeSeqLenHidden(&vec![1_i32, 2_i32, 3_i32]),
                    expect: ts("t", "<tag>", "c", Seq(vec![I32(1), I32(2), I32(3)])),
                },{
                    case:   map,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  SerializeMap(&map!["a" => 1_i32, "b" => 2_i32, "c" => 3_i32]),
                    expect: ts("t", "<tag>", "c", Map(map![
                        String("a".to_owned()) => I32(1),
                        String("b".to_owned()) => I32(2),
                        String("c".to_owned()) => I32(3),
                    ])),
                },{
                    case:   map_len_hidden,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  SerializeMapLenHidden(&map!["a" => 1_i32, "b" => 2_i32, "c" => 3_i32]),
                    expect: ts("t", "<tag>", "c", Map(map!{
                        String("a".to_owned()) => I32(1),
                        String("b".to_owned()) => I32(2),
                        String("c".to_owned()) => I32(3),
                    })),
                },

                {
                    case:   struct_unit,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  UnitStruct,
                    expect: ts("t", "<tag>", "c", Unit),
                },{
                    case:   struct_newtype_primitive,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  NewtypeStruct(42),
                    expect: ts("t", "<tag>", "c", Newtype(Box::new(I32(42)))),
                },{
                    case:   struct_newtype_nonprimitive,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  NewtypeStruct(Struct { foo: "bar" }),
                    expect: ts("t", "<tag>", "c", Newtype(Box::new(Map(map![
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])))),
                },{
                    case:   struct_tuple,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  TupleStruct(1, 2, 3, 4),
                    expect: ts("t", "<tag>", "c", Seq(vec![I32(1), I32(2), I32(3), I32(4)])),
                },{
                    case:   struct_normal,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  Struct { foo: "bar" },
                    expect: ts("t", "<tag>", "c", Map(map![
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])),
                },

                // TODO: tests for externally tagged enums
                //       missing due to https://github.com/arcnmx/serde-value/issues/18

                {
                    case:   enum_internal_unit,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedInternal::Unit,
                    expect: ts("t", "<tag>", "c", Map(map![
                        String("t".to_owned()) => String("Unit".to_owned()),
                    ])),
                },{
                    case:   enum_internal_newtype_nonprimitive,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedInternal::NewtypeC(Struct { foo: "bar" }),
                    expect: ts("t", "<tag>", "c", Map(map![
                        String("t".to_owned()) => String("NewtypeC".to_owned()),
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])),
                },{
                    case:   enum_internal_struct,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedInternal::Struct{ foo: "bar" },
                    expect: ts("t", "<tag>", "c", Map(map![
                        String("t".to_owned()) => String("Struct".to_owned()),
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])),
                },

                {
                    case:   enum_adjacent_unit,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedAdjacent::Unit,
                    expect: ts("t", "<tag>", "c", Map(map![
                        String("t".to_owned()) => String("Unit".to_owned())
                    ])),
                },{
                    case:   enum_adjacent_newtype_primitive,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedAdjacent::NewtypeP(42),
                    expect: ts("t", "<tag>", "c", Map(map![
                        String("t".to_owned()) => String("NewtypeP".to_owned()),
                        String("c".to_owned()) => I32(42),
                    ])),
                },{
                    case:   enum_adjacent_newtype_nonprimitive,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedAdjacent::NewtypeC(Struct { foo: "bar" }),
                    expect: ts("t", "<tag>", "c", Map(map![
                        String("t".to_owned()) => String("NewtypeC".to_owned()),
                        String("c".to_owned()) => Map(map![
                            String("foo".to_owned()) => String("bar".to_owned()),
                        ]),
                    ])),
                },{
                    case:   enum_adjacent_tuple,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedAdjacent::Tuple(3, 2, 1),
                    expect: ts("t", "<tag>", "c", Map(map![
                        String("t".to_owned()) => String("Tuple".to_owned()),
                        String("c".to_owned()) => Seq(vec![I32(3), I32(2), I32(1)]),
                    ])),
                },{
                    case:   enum_adjacent_struct,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedAdjacent::Struct{ foo: "bar" },
                    expect: ts("t", "<tag>", "c", Map(map![
                        String("t".to_owned()) => String("Struct".to_owned()),
                        String("c".to_owned()) => Map(map![
                            String("foo".to_owned()) => String("bar".to_owned()),
                        ]),
                    ])),
                },

                {
                    case:   enum_untagged_unit,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumUntagged::Unit,
                    expect: ts("t", "<tag>", "c", Unit),
                },{
                    case:   enum_untagged_newtype_primitive,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumUntagged::NewtypeP(42),
                    expect: ts("t", "<tag>", "c", I32(42)),
                },{
                    case:   enum_untagged_newtype_nonprimitive,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumUntagged::NewtypeC(Struct { foo: "bar" }),
                    expect: ts("t", "<tag>", "c", Map(map![
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])),
                },{
                    case:   enum_untagged_tuple,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumUntagged::Tuple(3, 2, 1),
                    expect: ts("t", "<tag>", "c", Seq(vec![I32(3), I32(2), I32(1)])),
                },{
                    case:   enum_untagged_struct,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumUntagged::Struct{ foo: "bar" },
                    expect: ts("t", "<tag>", "c", Map(map![
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])),
                },

                {
                    case:   collect_seq,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  CollectSeq(vec![1_i32, 2_i32, 3_i32, 4_i32]),
                    expect: ts("t", "<tag>", "c", Seq(vec![I32(1), I32(2), I32(3), I32(4)])),
                },{
                    case:   collect_map,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  CollectMap(map!["a" => 1_i32, "b" => 2_i32, "c" => 3_i32]),
                    expect: ts("t", "<tag>", "c", Map(map![
                        String("a".to_owned()) => I32(1),
                        String("b".to_owned()) => I32(2),
                        String("c".to_owned()) => I32(3),
                    ])),
                },{
                    case:   collect_str,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  CollectStr("foobar"),
                    expect: ts("t", "<tag>", "c", String("foobar".into())),
                },
            }
        }
    }
}

/// Tests for deserialization of tagged values.
mod de {

    /// Tests for deserialization of externally-tagged values.
    mod external {
        use common::types::*;
        use serde_value::Value;

        #[test]
        fn without_tag_phantom() {
            use serde_tagged::de::external::deserialize as de;
            use std::marker::PhantomData;

            let value = Value::Map(map![
                Value::String("tag".to_owned()) => Value::Map(map![
                    Value::String("foo".to_owned()) => Value::String("bar".to_owned()),
                ]),
            ]);

            let v = de::<String, Struct<String>, _, _>(value, PhantomData).unwrap();

            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn without_tag() {
            use serde_tagged::de::WithoutTag;
            use serde_tagged::de::external::deserialize as de;

            let value = Value::Map(map![
                Value::String("tag".to_owned()) => Value::Map(map![
                    Value::String("foo".to_owned()) => Value::String("bar".to_owned()),
                ]),
            ]);

            let v = de::<String, Struct<String>, _, _>(value, WithoutTag::new()).unwrap();

            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn with_tag() {
            use serde_tagged::de::WithTag;
            use serde_tagged::de::external::deserialize as de;

            let value = Value::Map(map![
                Value::String("tag".to_owned()) => Value::Map(map![
                    Value::String("foo".to_owned()) => Value::String("bar".to_owned()),
                ]),
            ]);

            let (t, v) = de::<_, (String, Struct<String>), _, _>(value, WithTag::new()).unwrap();

            assert_eq!(t, "tag");
            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        #[should_panic]
        fn error_map_empty() {
            use serde_tagged::de::WithTag;
            use serde_tagged::de::external::deserialize as de;

            let value = Value::Map(map![]);

            let (_t, _v) = de::<_, (String, String), _, _>(value, WithTag::new()).unwrap();
        }

        #[test]
        #[should_panic]
        fn error_map_len() {
            use serde_tagged::de::WithTag;
            use serde_tagged::de::external::deserialize as de;

            let value = Value::Map(map![
                Value::String("a".to_owned()) => Value::String("b".to_owned()),
                Value::String("c".to_owned()) => Value::String("d".to_owned()),
            ]);

            let (_t, _v) = de::<_, (String, String), _, _>(value, WithTag::new()).unwrap();
        }

        #[test]
        #[should_panic]
        fn error_type() {
            use serde_tagged::de::WithTag;
            use serde_tagged::de::external::deserialize as de;

            let value = Value::I32(42);

            let (_t, _v) = de::<_, (&str, &str), _, _>(value, WithTag::new()).unwrap();
        }
    }

    /// Tests for deserialization of tuple-based adjacently-tagged values.
    mod adj_tuple {
        use common::types::*;
        use serde_value::Value;

        #[test]
        fn without_tag_phantom() {
            use serde_tagged::de::adj::tuple::deserialize as de;
            use std::marker::PhantomData;

            let value = Value::Seq(vec![
                Value::String("tag".to_owned()),
                Value::Map(map![
                    Value::String("foo".to_owned()) => Value::String("bar".to_owned()),
                ]),
            ]);

            let v = de::<String, Struct<String>, _, _>(value, PhantomData).unwrap();

            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn without_tag() {
            use serde_tagged::de::WithoutTag;
            use serde_tagged::de::adj::tuple::deserialize as de;

            let value = Value::Seq(vec![
                Value::String("tag".to_owned()),
                Value::Map(map![
                    Value::String("foo".to_owned()) => Value::String("bar".to_owned()),
                ]),
            ]);

            let v = de::<String, Struct<String>, _, _>(value, WithoutTag::new()).unwrap();

            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn with_tag() {
            use serde_tagged::de::WithTag;
            use serde_tagged::de::adj::tuple::deserialize as de;

            let value = Value::Seq(vec![
                Value::String("tag".to_owned()),
                Value::Map(map![
                    Value::String("foo".to_owned()) => Value::String("bar".to_owned()),
                ]),
            ]);

            let (t, v) = de::<_, (String, Struct<String>), _, _>(value, WithTag::new()).unwrap();

            assert_eq!(t, "tag");
            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        #[should_panic]
        fn error_tuple_empty() {
            use serde_tagged::de::WithTag;
            use serde_tagged::de::adj::tuple::deserialize as de;

            let value = Value::Seq(vec![]);

            let (_t, _v) = de::<_, (String, String), _, _>(value, WithTag::new()).unwrap();
        }

        #[test]
        #[should_panic]
        fn error_tuple_len() {
            use serde_tagged::de::WithTag;
            use serde_tagged::de::adj::tuple::deserialize as de;

            let value = Value::Seq(vec![
                Value::String("a".to_owned()),
                Value::String("b".to_owned()),
                Value::String("c".to_owned()),
                Value::String("d".to_owned()),
            ]);

            let (_t, _v) = de::<_, (String, String), _, _>(value, WithTag::new()).unwrap();
        }

        #[test]
        #[should_panic]
        fn error_type() {
            use serde_tagged::de::WithTag;
            use serde_tagged::de::adj::tuple::deserialize as de;

            let value = Value::I32(42);

            let (_t, _v) = de::<_, (&str, &str), _, _>(value, WithTag::new()).unwrap();
        }
    }
}
