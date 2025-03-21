//! Tests using `serde-value`.

#![allow(clippy::unreadable_literal)]

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
        use crate::common::types::*;

        use serde_value::Value;
        use serde_value::Value::*;

        /// Apply a string as external tag to the specified value.
        fn ts(tag: &str, value: Value) -> Value {
            Value::Map(map![Value::String(tag.to_owned()) => value])
        }

        generate_tests_ser_1! {
            use crate::common::formats::value::ser::external::serialize_wrapped : wrapped,
            use crate::common::formats::value::ser::external::serialize_with_serializer : with_serializer,

            with {
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
                    value:  NewtypeStruct(Struct { foo: "bar".to_owned() }),
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
                    value:  Struct { foo: "bar".to_owned() },
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
                    value:  EnumTaggedInternal::NewtypeC(Struct { foo: "bar".to_owned() }),
                    expect: ts("<tag>", Map(map![
                        String("t".to_owned()) => String("NewtypeC".to_owned()),
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])),
                },{
                    case:   enum_internal_struct,
                    tag:    "<tag>",
                    value:  EnumTaggedInternal::Struct { foo: "bar".to_owned() },
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
                    value:  EnumTaggedAdjacent::NewtypeC(Struct { foo: "bar".to_owned() }),
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
                    value:  EnumTaggedAdjacent::Struct { foo: "bar".to_owned() },
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
                    value:  EnumUntagged::NewtypeC(Struct { foo: "bar".to_owned() }),
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
                    value:  EnumUntagged::Struct { baz: "bar".to_owned() },
                    expect: ts("<tag>", Map(map![
                        String("baz".to_owned()) => String("bar".to_owned()),
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
        use crate::common::types::*;

        use serde_value::Value;
        use serde_value::Value::*;

        /// Apply a string as external tag to the specified value.
        fn ts(tag: &str, value: Value) -> Value {
            Value::Seq(vec![Value::String(tag.to_owned()), value])
        }


        generate_tests_ser_1! {
            use crate::common::formats::value::ser::adj_tuple::serialize_wrapped : wrapped,
            use crate::common::formats::value::ser::adj_tuple::serialize_with_serializer : with_serializer,

            with {
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
                    value:  NewtypeStruct(Struct { foo: "bar".to_owned() }),
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
                    value:  Struct { foo: "bar".to_owned() },
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
                    value:  EnumTaggedInternal::NewtypeC(Struct { foo: "bar".to_owned() }),
                    expect: ts("<tag>", Map(map![
                        String("t".to_owned()) => String("NewtypeC".to_owned()),
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])),
                },{
                    case:   enum_internal_struct,
                    tag:    "<tag>",
                    value:  EnumTaggedInternal::Struct { foo: "bar".to_owned() },
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
                    value:  EnumTaggedAdjacent::NewtypeC(Struct { foo: "bar".to_owned() }),
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
                    value:  EnumTaggedAdjacent::Struct { foo: "bar".to_owned() },
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
                    value:  EnumUntagged::NewtypeC(Struct { foo: "bar".to_owned() }),
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
                    value:  EnumUntagged::Struct { baz: "bar".to_owned() },
                    expect: ts("<tag>", Map(map![
                        String("baz".to_owned()) => String("bar".to_owned()),
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

    /// Tests for serialization of map-based adjacently-tagged values.
    mod adj_map {
        use crate::common::types::*;

        use serde_value::Value;
        use serde_value::Value::*;
        use std::option;

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

        generate_tests_ser_3! {
            use crate::common::formats::value::ser::adj_map::serialize_wrapped : wrapped,
            use crate::common::formats::value::ser::adj_map::serialize_with_serializer : with_serializer,

            with {
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
                    value:  NewtypeStruct(Struct { foo: "bar".to_owned() }),
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
                    value:  Struct { foo: "bar".to_owned() },
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
                    value:  EnumTaggedInternal::NewtypeC(Struct { foo: "bar".to_owned() }),
                    expect: ts("t", "<tag>", "c", Map(map![
                        String("t".to_owned()) => String("NewtypeC".to_owned()),
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])),
                },{
                    case:   enum_internal_struct,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedInternal::Struct { foo: "bar".to_owned() },
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
                    value:  EnumTaggedAdjacent::NewtypeC(Struct { foo: "bar".to_owned() }),
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
                    value:  EnumTaggedAdjacent::Struct { foo: "bar".to_owned() },
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
                    value:  EnumUntagged::NewtypeC(Struct { foo: "bar".to_owned() }),
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
                    value:  EnumUntagged::Struct { baz: "bar".to_owned() },
                    expect: ts("t", "<tag>", "c", Map(map![
                        String("baz".to_owned()) => String("bar".to_owned()),
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

    /// Tests for serialization of struct-based adjacently-tagged values.
    mod adj_struc {
        use crate::common::types::*;

        use serde_value::Value;
        use serde_value::Value::*;
        use std::option;

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

        generate_tests_ser_3! {
            use crate::common::formats::value::ser::adj_struc::serialize_wrapped : wrapped,
            use crate::common::formats::value::ser::adj_struc::serialize_with_serializer : with_serializer,

            with {
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
                    value:  NewtypeStruct(Struct { foo: "bar".to_owned() }),
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
                    value:  Struct { foo: "bar".to_owned() },
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
                    value:  EnumTaggedInternal::NewtypeC(Struct { foo: "bar".to_owned() }),
                    expect: ts("t", "<tag>", "c", Map(map![
                        String("t".to_owned()) => String("NewtypeC".to_owned()),
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ])),
                },{
                    case:   enum_internal_struct,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedInternal::Struct { foo: "bar".to_owned() },
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
                    value:  EnumTaggedAdjacent::NewtypeC(Struct { foo: "bar".to_owned() }),
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
                    value:  EnumTaggedAdjacent::Struct { foo: "bar".to_owned() },
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
                    value:  EnumUntagged::NewtypeC(Struct { foo: "bar".to_owned() }),
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
                    value:  EnumUntagged::Struct { baz: "bar".to_owned() },
                    expect: ts("t", "<tag>", "c", Map(map![
                        String("baz".to_owned()) => String("bar".to_owned()),
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

    /// Tests for serialization of internally-tagged values.
    mod internal {
        use crate::common::types::*;

        use serde_value::Value::*;
        use std::option;

        generate_tests_ser_2! {
            use crate::common::formats::value::ser::internal::serialize: serialize,

            with {
                {
                    case:   tuple,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  (1_i32, 2_i32, 3_i32),
                    expect: Seq(vec![String("<tag>".to_owned()), I32(1), I32(2), I32(3)]),
                },{
                    case:   seq,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  SerializeSeq(&vec![1_i32, 2_i32, 3_i32]),
                    expect: Seq(vec![String("<tag>".to_owned()), I32(1), I32(2), I32(3)]),
                },{
                    case:   seq_len_hidden,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  SerializeSeqLenHidden(&vec![1_i32, 2_i32, 3_i32]),
                    expect: Seq(vec![String("<tag>".to_owned()), I32(1), I32(2), I32(3)]),
                },{
                    case:   map,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  SerializeMap(&map!["a" => 1_i32, "b" => 2_i32, "c" => 3_i32]),
                    expect: Map(map![
                        String("tag".to_owned()) => String("<tag>".to_owned()),
                        String("a".to_owned()) => I32(1),
                        String("b".to_owned()) => I32(2),
                        String("c".to_owned()) => I32(3),
                    ]),
                },{
                    case:   map_len_hidden,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  SerializeMapLenHidden(&map!["a" => 1_i32, "b" => 2_i32, "c" => 3_i32]),
                    expect: Map(map![
                        String("tag".to_owned()) => String("<tag>".to_owned()),
                        String("a".to_owned()) => I32(1),
                        String("b".to_owned()) => I32(2),
                        String("c".to_owned()) => I32(3),
                    ]),
                },

                {
                    case:   struct_unit,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  UnitStruct,
                    expect: Map(map![
                        String("tag".to_owned()) => String("<tag>".to_owned()),
                    ]),
                },{
                    case:   struct_newtype_nonprimitive,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  NewtypeStruct(Struct { foo: "bar".to_owned() }),
                    expect: Map(map![
                        String("tag".to_owned()) => String("<tag>".to_owned()),
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ]),
                },{
                    case:   struct_tuple,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  TupleStruct(1, 2, 3, 4),
                    expect: Seq(vec![String("<tag>".to_owned()), I32(1), I32(2), I32(3), I32(4)]),
                },{
                    case:   struct_normal,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  Struct { foo: "bar".to_owned() },
                    expect: Map(map![
                        String("tag".to_owned()) => String("<tag>".to_owned()),
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ]),
                },

                {
                    case:   enum_internal_unit,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  EnumTaggedInternal::Unit,
                    expect: Map(map![
                        String("tag".to_owned()) => String("<tag>".to_owned()),
                        String("t".to_owned()) => String("Unit".to_owned()),
                    ]),
                },{
                    case:   enum_internal_newtype_nonprimitive,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  EnumTaggedInternal::NewtypeC(Struct { foo: "bar".to_owned() }),
                    expect: Map(map![
                        String("tag".to_owned()) => String("<tag>".to_owned()),
                        String("t".to_owned()) => String("NewtypeC".to_owned()),
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ]),
                },{
                    case:   enum_internal_struct,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  EnumTaggedInternal::Struct { foo: "bar".to_owned() },
                    expect: Map(map![
                        String("tag".to_owned()) => String("<tag>".to_owned()),
                        String("t".to_owned()) => String("Struct".to_owned()),
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ]),
                },

                {
                    case:   enum_adjacent_unit,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  EnumTaggedAdjacent::Unit,
                    expect: Map(map![
                        String("tag".to_owned()) => String("<tag>".to_owned()),
                        String("t".to_owned()) => String("Unit".to_owned()),
                    ]),
                },{
                    case:   enum_adjacent_newtype_primitive,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  EnumTaggedAdjacent::NewtypeP(42),
                    expect: Map(map![
                        String("tag".to_owned()) => String("<tag>".to_owned()),
                        String("t".to_owned()) => String("NewtypeP".to_owned()),
                        String("c".to_owned()) => I32(42),
                    ]),
                },{
                    case:   enum_adjacent_newtype_nonprimitive,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  EnumTaggedAdjacent::NewtypeC(Struct { foo: "bar".to_owned() }),
                    expect: Map(map![
                        String("tag".to_owned()) => String("<tag>".to_owned()),
                        String("t".to_owned()) => String("NewtypeC".to_owned()),
                        String("c".to_owned()) => Map(map![
                            String("foo".to_owned()) => String("bar".to_owned()),
                        ]),
                    ]),
                },{
                    case:   enum_adjacent_tuple,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  EnumTaggedAdjacent::Tuple(3, 2, 1),
                    expect: Map(map![
                        String("tag".to_owned()) => String("<tag>".to_owned()),
                        String("t".to_owned()) => String("Tuple".to_owned()),
                        String("c".to_owned()) => Seq(vec![I32(3), I32(2), I32(1)]),
                    ]),
                },{
                    case:   enum_adjacent_struct,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  EnumTaggedAdjacent::Struct { foo: "bar".to_owned() },
                    expect: Map(map![
                        String("tag".to_owned()) => String("<tag>".to_owned()),
                        String("t".to_owned()) => String("Struct".to_owned()),
                        String("c".to_owned()) => Map(map![
                            String("foo".to_owned()) => String("bar".to_owned()),
                        ]),
                    ]),
                },

                {
                    case:   enum_untagged_newtype_nonprimitive,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  EnumUntagged::NewtypeC(Struct { foo: "bar".to_owned() }),
                    expect: Map(map![
                        String("tag".to_owned()) => String("<tag>".to_owned()),
                        String("foo".to_owned()) => String("bar".to_owned()),
                    ]),
                },{
                    case:   enum_untagged_tuple,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  EnumUntagged::Tuple(1, 2, 3),
                    expect: Seq(vec![String("<tag>".to_owned()), I32(1), I32(2), I32(3)]),
                },{
                    case:   enum_untagged_struct,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  EnumUntagged::Struct { baz: "bar".to_owned() },
                    expect: Map(map![
                        String("tag".to_owned()) => String("<tag>".to_owned()),
                        String("baz".to_owned()) => String("bar".to_owned()),
                    ]),
                },
            }
        }

        #[test]
        #[should_panic]
        fn i8() {
            use crate::common::formats::value::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &42_i8).unwrap();
        }

        #[test]
        #[should_panic]
        fn i16() {
            use crate::common::formats::value::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &42_i16).unwrap();
        }

        #[test]
        #[should_panic]
        fn i32() {
            use crate::common::formats::value::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &42_i32).unwrap();
        }

        #[test]
        #[should_panic]
        fn i64() {
            use crate::common::formats::value::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &42_i64).unwrap();
        }

        #[test]
        #[should_panic]
        fn u8() {
            use crate::common::formats::value::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &42_u8).unwrap();
        }

        #[test]
        #[should_panic]
        fn u16() {
            use crate::common::formats::value::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &42_u16).unwrap();
        }

        #[test]
        #[should_panic]
        fn u32() {
            use crate::common::formats::value::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &42_u32).unwrap();
        }

        #[test]
        #[should_panic]
        fn u64() {
            use crate::common::formats::value::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &42_u64).unwrap();
        }

        #[test]
        #[should_panic]
        fn f32() {
            use crate::common::formats::value::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &2.0_f32).unwrap();
        }

        #[test]
        #[should_panic]
        fn f64() {
            use crate::common::formats::value::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &2.0_f64).unwrap();
        }

        #[test]
        #[should_panic]
        fn char() {
            use crate::common::formats::value::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &('c')).unwrap();
        }

        #[test]
        #[should_panic]
        fn str() {
            use crate::common::formats::json::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", "test").unwrap();
        }

        #[test]
        #[should_panic]
        fn bytes() {
            use crate::common::formats::value::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &serde_bytes::Bytes::new(&[0, 1, 2, 3])).unwrap();
        }

        #[test]
        #[should_panic]
        fn none() {
            use crate::common::formats::value::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &None as &option::Option<i32>).unwrap();
        }

        #[test]
        #[should_panic]
        fn some() {
            use crate::common::formats::value::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &Some(42_i32)).unwrap();
        }

        #[test]
        #[should_panic]
        fn unit() {
            use crate::common::formats::value::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &()).unwrap();
        }

        #[test]
        #[should_panic]
        fn enum_external_unit() {
            use crate::common::formats::value::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &EnumTaggedExternal::Unit).unwrap();
        }

        #[test]
        #[should_panic]
        fn enum_external_newtype_primitive() {
            use crate::common::formats::value::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &EnumTaggedExternal::NewtypeP(42)).unwrap();
        }

        #[test]
        #[should_panic]
        fn enum_external_newtype_nonprimitive() {
            use crate::common::formats::value::ser::internal::serialize;
            let _v = serialize(
                "tag",
                "<tag>",
                &EnumTaggedExternal::NewtypeC(Struct {
                    foo: "bar".to_owned(),
                }),
            )
            .unwrap();
        }

        #[test]
        #[should_panic]
        fn enum_external_tuple() {
            use crate::common::formats::value::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &EnumTaggedExternal::Tuple(3, 4, 5)).unwrap();
        }

        #[test]
        #[should_panic]
        fn enum_external_struct() {
            use crate::common::formats::value::ser::internal::serialize;
            let _v = serialize(
                "tag",
                "<tag>",
                &EnumTaggedExternal::Struct {
                    foo: "bar".to_owned(),
                },
            )
            .unwrap();
        }

        #[test]
        #[should_panic]
        fn enum_untagged_unit() {
            use crate::common::formats::value::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &EnumUntagged::Unit).unwrap();
        }

        #[test]
        #[should_panic]
        fn enum_untagged_newtype_primitive() {
            use crate::common::formats::value::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &EnumUntagged::NewtypeP(42)).unwrap();
        }
    }
}

/// Tests for deserialization of tagged values.
mod de {

    /// Tests for deserialization of externally-tagged values.
    mod external {
        use crate::common::types::*;
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

            let v: Struct<String> = de::<String, _, _>(value, PhantomData).unwrap();

            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn without_tag() {
            use serde_tagged::de::external::deserialize as de;
            use serde_tagged::de::WithoutTag;

            let value = Value::Map(map![
                Value::String("tag".to_owned()) => Value::Map(map![
                    Value::String("foo".to_owned()) => Value::String("bar".to_owned()),
                ]),
            ]);

            let v: Struct<String> = de::<String, _, _>(value, WithoutTag::new()).unwrap();

            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn with_tag() {
            use serde_tagged::de::external::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::Map(map![
                Value::String("tag".to_owned()) => Value::Map(map![
                    Value::String("foo".to_owned()) => Value::String("bar".to_owned()),
                ]),
            ]);

            let (t, v): (String, Struct<String>) = de(value, WithTag::new()).unwrap();

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
            use serde_tagged::de::external::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::Map(map![]);

            let (_t, _v): (String, String) = de(value, WithTag::new()).unwrap();
        }

        #[test]
        #[should_panic]
        fn error_map_len() {
            use serde_tagged::de::external::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::Map(map![
                Value::String("a".to_owned()) => Value::String("b".to_owned()),
                Value::String("c".to_owned()) => Value::String("d".to_owned()),
            ]);

            let (_t, _v): (String, String) = de(value, WithTag::new()).unwrap();
        }

        #[test]
        #[should_panic]
        fn error_type() {
            use serde_tagged::de::external::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::I32(42);

            let (_t, _v): (String, String) = de(value, WithTag::new()).unwrap();
        }
    }

    /// Tests for deserialization of tuple-based adjacently-tagged values.
    mod adj_tuple {
        use crate::common::types::*;
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

            let v: Struct<String> = de::<String, _, _>(value, PhantomData).unwrap();

            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn without_tag() {
            use serde_tagged::de::adj::tuple::deserialize as de;
            use serde_tagged::de::WithoutTag;

            let value = Value::Seq(vec![
                Value::String("tag".to_owned()),
                Value::Map(map![
                    Value::String("foo".to_owned()) => Value::String("bar".to_owned()),
                ]),
            ]);

            let v: Struct<String> = de::<String, _, _>(value, WithoutTag::new()).unwrap();

            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn with_tag() {
            use serde_tagged::de::adj::tuple::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::Seq(vec![
                Value::String("tag".to_owned()),
                Value::Map(map![
                    Value::String("foo".to_owned()) => Value::String("bar".to_owned()),
                ]),
            ]);

            let (t, v): (String, Struct<String>) = de(value, WithTag::new()).unwrap();

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
            use serde_tagged::de::adj::tuple::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::Seq(vec![]);

            let (_t, _v): (String, String) = de(value, WithTag::new()).unwrap();
        }

        #[test]
        #[should_panic]
        fn error_tuple_len() {
            use serde_tagged::de::adj::tuple::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::Seq(vec![
                Value::String("a".to_owned()),
                Value::String("b".to_owned()),
                Value::String("c".to_owned()),
                Value::String("d".to_owned()),
            ]);

            let (_t, _v): (String, String) = de(value, WithTag::new()).unwrap();
        }

        #[test]
        #[should_panic]
        fn error_type() {
            use serde_tagged::de::adj::tuple::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::I32(42);

            let (_t, _v): (String, String) = de(value, WithTag::new()).unwrap();
        }
    }

    /// Tests for deserialization of map-based adjacently-tagged values.
    mod adj_map {
        use crate::common::types::*;
        use serde_value::Value;

        #[test]
        fn without_tag_phantom() {
            use serde_tagged::de::adj::map::deserialize as de;
            use std::marker::PhantomData;

            let value = Value::Map(map![
                Value::String("t".to_owned()) => Value::String("tag".to_owned()),
                Value::String("c".to_owned()) => Value::Map(map![
                    Value::String("foo".to_owned()) => Value::String("bar".to_owned()),
                ]),
            ]);

            let v: Struct<String> =
                de::<String, String, _, _, _>(value, "t", "c", PhantomData).unwrap();

            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn without_tag() {
            use serde_tagged::de::adj::map::deserialize as de;
            use serde_tagged::de::WithoutTag;

            let value = Value::Map(map![
                Value::String("t".to_owned()) => Value::String("tag".to_owned()),
                Value::String("c".to_owned()) => Value::Map(map![
                    Value::String("foo".to_owned()) => Value::String("bar".to_owned()),
                ]),
            ]);

            let v: Struct<String> =
                de::<String, String, _, _, _>(value, "t", "c", WithoutTag::new()).unwrap();

            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn with_tag() {
            use serde_tagged::de::adj::map::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::Map(map![
                Value::String("t".to_owned()) => Value::String("tag".to_owned()),
                Value::String("c".to_owned()) => Value::Map(map![
                    Value::String("foo".to_owned()) => Value::String("bar".to_owned()),
                ]),
            ]);

            let (t, v): (String, Struct<String>) =
                de::<_, String, _, _, _>(value, "t", "c", WithTag::new()).unwrap();

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
            use serde_tagged::de::adj::map::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::Map(map![]);

            let (_t, _v): (String, String) =
                de::<_, String, _, _, _>(value, "t", "c", WithTag::new()).unwrap();
        }

        #[test]
        #[should_panic]
        fn error_map_len() {
            use serde_tagged::de::adj::map::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::Map(map![
                Value::String("t".to_owned()) => Value::String("b".to_owned()),
                Value::String("c".to_owned()) => Value::String("d".to_owned()),
                Value::String("e".to_owned()) => Value::String("f".to_owned()),
            ]);

            let (_t, _v): (String, String) =
                de::<_, String, _, _, _>(value, "t", "c", WithTag::new()).unwrap();
        }

        #[test]
        #[should_panic]
        fn error_keys() {
            use serde_tagged::de::adj::map::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::Map(map![
                Value::String("t".to_owned()) => Value::String("b".to_owned()),
                Value::String("c".to_owned()) => Value::String("d".to_owned()),
            ]);

            let (_t, _v): (String, String) =
                de::<_, String, _, _, _>(value, "a", "b", WithTag::new()).unwrap();
        }

        #[test]
        #[should_panic]
        fn error_type() {
            use serde_tagged::de::adj::map::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::Seq(vec![]);

            let (_t, _v): (String, String) =
                de::<_, String, _, _, _>(value, "t", "c", WithTag::new()).unwrap();
        }

        #[test]
        fn known() {
            use serde_tagged::de::adj::map::deserialize_known as de;

            let value = Value::Map(map![
                Value::String("t".to_owned()) => Value::String("tag".to_owned()),
                Value::String("c".to_owned()) => Value::Map(map![
                    Value::String("foo".to_owned()) => Value::String("bar".to_owned()),
                ]),
            ]);

            let (t, v): (String, Struct<String>) =
                de::<_, _, String, _, _>(value, "t", "c").unwrap();

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
        fn known_error_map_empty() {
            use serde_tagged::de::adj::map::deserialize_known as de;

            let value = Value::Map(map![]);

            let (_t, _v): (String, String) = de::<_, _, String, _, _>(value, "t", "c").unwrap();
        }

        #[test]
        #[should_panic]
        fn known_error_map_len() {
            use serde_tagged::de::adj::map::deserialize_known as de;

            let value = Value::Map(map![
                Value::String("t".to_owned()) => Value::String("b".to_owned()),
                Value::String("c".to_owned()) => Value::String("d".to_owned()),
                Value::String("e".to_owned()) => Value::String("f".to_owned()),
            ]);

            let (_t, _v): (String, String) = de::<_, _, String, _, _>(value, "t", "c").unwrap();
        }

        #[test]
        #[should_panic]
        fn known_error_keys() {
            use serde_tagged::de::adj::map::deserialize_known as de;

            let value = Value::Map(map![
                Value::String("t".to_owned()) => Value::String("b".to_owned()),
                Value::String("c".to_owned()) => Value::String("d".to_owned()),
            ]);

            let (_t, _v): (String, String) = de::<_, _, String, _, _>(value, "a", "b").unwrap();
        }

        #[test]
        #[should_panic]
        fn known_error_type() {
            use serde_tagged::de::adj::map::deserialize_known as de;

            let value = Value::Seq(vec![]);

            let (_t, _v): (String, String) = de::<_, _, String, _, _>(value, "t", "c").unwrap();
        }
    }

    /// Tests for deserialization of struct-based adjacently-tagged values.
    mod adj_struc {
        use crate::common::types::*;
        use serde_value::Value;

        #[test]
        fn without_tag_phantom() {
            use serde_tagged::de::adj::struc::deserialize as de;
            use std::marker::PhantomData;

            let value = Value::Map(map![
                Value::String("t".to_owned()) => Value::String("tag".to_owned()),
                Value::String("c".to_owned()) => Value::Map(map![
                    Value::String("foo".to_owned()) => Value::String("bar".to_owned()),
                ]),
            ]);

            let v: Struct<String> =
                de::<String, _, _>(value, "Tagged", "t", "c", PhantomData).unwrap();

            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn without_tag_phantom_seq() {
            use serde_tagged::de::adj::struc::deserialize as de;
            use std::marker::PhantomData;

            let value = Value::Seq(vec![
                Value::String("tag".to_owned()),
                Value::Map(map![
                    Value::String("foo".to_owned()) => Value::String("bar".to_owned()),
                ]),
            ]);

            let v: Struct<String> =
                de::<String, _, _>(value, "Tagged", "t", "c", PhantomData).unwrap();

            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn without_tag() {
            use serde_tagged::de::adj::struc::deserialize as de;
            use serde_tagged::de::WithoutTag;

            let value = Value::Map(map![
                Value::String("t".to_owned()) => Value::String("tag".to_owned()),
                Value::String("c".to_owned()) => Value::Map(map![
                    Value::String("foo".to_owned()) => Value::String("bar".to_owned()),
                ]),
            ]);

            let v: Struct<String> =
                de::<String, _, _>(value, "Tagged", "t", "c", WithoutTag::new()).unwrap();

            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn without_tag_seq() {
            use serde_tagged::de::adj::struc::deserialize as de;
            use serde_tagged::de::WithoutTag;

            let value = Value::Seq(vec![
                Value::String("tag".to_owned()),
                Value::Map(map![
                    Value::String("foo".to_owned()) => Value::String("bar".to_owned()),
                ]),
            ]);

            let v: Struct<String> =
                de::<String, _, _>(value, "Tagged", "t", "c", WithoutTag::new()).unwrap();

            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn with_tag() {
            use serde_tagged::de::adj::struc::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::Map(map![
                Value::String("t".to_owned()) => Value::String("tag".to_owned()),
                Value::String("c".to_owned()) => Value::Map(map![
                    Value::String("foo".to_owned()) => Value::String("bar".to_owned()),
                ]),
            ]);

            let (t, v): (String, Struct<String>) =
                de(value, "Tagged", "t", "c", WithTag::new()).unwrap();

            assert_eq!(t, "tag");
            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn with_tag_seq() {
            use serde_tagged::de::adj::struc::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::Seq(vec![
                Value::String("tag".to_owned()),
                Value::Map(map![
                    Value::String("foo".to_owned()) => Value::String("bar".to_owned()),
                ]),
            ]);

            let (t, v): (String, Struct<String>) =
                de(value, "Tagged", "t", "c", WithTag::new()).unwrap();

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
            use serde_tagged::de::adj::struc::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::Map(map![]);

            let (_t, _v): (String, String) = de(value, "Tagged", "t", "c", WithTag::new()).unwrap();
        }

        #[test]
        #[should_panic]
        fn error_seq_empty() {
            use serde_tagged::de::adj::struc::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::Seq(vec![]);

            let (_t, _v): (String, String) = de(value, "Tagged", "t", "c", WithTag::new()).unwrap();
        }

        #[test]
        #[should_panic]
        fn error_map_len() {
            use serde_tagged::de::adj::struc::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::Map(map![
                Value::String("t".to_owned()) => Value::String("b".to_owned()),
                Value::String("c".to_owned()) => Value::String("d".to_owned()),
                Value::String("e".to_owned()) => Value::String("f".to_owned()),
            ]);

            let (_t, _v): (String, String) = de(value, "Tagged", "t", "c", WithTag::new()).unwrap();
        }

        #[test]
        #[should_panic]
        fn error_keys() {
            use serde_tagged::de::adj::struc::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::Map(map![
                Value::String("t".to_owned()) => Value::String("b".to_owned()),
                Value::String("c".to_owned()) => Value::String("d".to_owned()),
            ]);

            let (_t, _v): (String, String) = de(value, "Tagged", "a", "b", WithTag::new()).unwrap();
        }

        #[test]
        #[should_panic]
        fn error_type() {
            use serde_tagged::de::adj::struc::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::I32(1);

            let (_t, _v): (String, String) = de(value, "Tagged", "t", "c", WithTag::new()).unwrap();
        }

        #[test]
        fn known() {
            use serde_tagged::de::adj::struc::deserialize_known as de;

            let value = Value::Map(map![
                Value::String("t".to_owned()) => Value::String("tag".to_owned()),
                Value::String("c".to_owned()) => Value::Map(map![
                    Value::String("foo".to_owned()) => Value::String("bar".to_owned()),
                ]),
            ]);

            let (t, v) = de::<String, Struct<String>, _>(value, "Tagged", "t", "c").unwrap();

            assert_eq!(t, "tag");
            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn known_seq() {
            use serde_tagged::de::adj::struc::deserialize_known as de;

            let value = Value::Seq(vec![
                Value::String("tag".to_owned()),
                Value::Map(map![
                    Value::String("foo".to_owned()) => Value::String("bar".to_owned()),
                ]),
            ]);

            let (t, v) = de::<String, Struct<String>, _>(value, "Tagged", "t", "c").unwrap();

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
        fn known_error_map_empty() {
            use serde_tagged::de::adj::struc::deserialize_known as de;

            let value = Value::Map(map![]);

            let (_t, _v) = de::<String, String, _>(value, "Tagged", "t", "c").unwrap();
        }

        #[test]
        #[should_panic]
        fn known_error_seq_empty() {
            use serde_tagged::de::adj::struc::deserialize_known as de;

            let value = Value::Seq(vec![]);

            let (_t, _v) = de::<String, String, _>(value, "Tagged", "t", "c").unwrap();
        }

        #[test]
        #[should_panic]
        fn known_error_map_len() {
            use serde_tagged::de::adj::struc::deserialize_known as de;

            let value = Value::Map(map![
                Value::String("t".to_owned()) => Value::String("b".to_owned()),
                Value::String("c".to_owned()) => Value::String("d".to_owned()),
                Value::String("e".to_owned()) => Value::String("f".to_owned()),
            ]);

            let (_t, _v) = de::<String, String, _>(value, "Tagged", "t", "c").unwrap();
        }

        #[test]
        #[should_panic]
        fn known_error_keys() {
            use serde_tagged::de::adj::struc::deserialize_known as de;

            let value = Value::Map(map![
                Value::String("t".to_owned()) => Value::String("b".to_owned()),
                Value::String("c".to_owned()) => Value::String("d".to_owned()),
            ]);

            let (_t, _v) = de::<String, String, _>(value, "Tagged", "a", "b").unwrap();
        }

        #[test]
        #[should_panic]
        fn known_error_type() {
            use serde_tagged::de::adj::struc::deserialize_known as de;

            let value = Value::I32(1);

            let (_t, _v) = de::<String, String, _>(value, "Tagged", "t", "c").unwrap();
        }
    }

    /// Tests for deserialization of internally-tagged values.
    mod internal {
        use crate::common::types::*;
        use serde_value::Value;
        use std::collections::BTreeMap;

        #[test]
        fn seq() {
            use serde_tagged::de::internal::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::Seq(vec![
                Value::String("foo".to_owned()),
                Value::I32(1),
                Value::I32(2),
                Value::I32(3),
                Value::I32(4),
            ]);

            let (t, v): (String, Vec<i32>) = de(value, "tag", WithTag::new()).unwrap();

            assert_eq!(t, "foo");
            assert_eq!(v, [1, 2, 3, 4]);
        }

        #[test]
        fn map() {
            use serde_tagged::de::internal::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::Map(map![
                Value::String("a".to_owned()) => Value::String("b".to_owned()),
                Value::String("c".to_owned()) => Value::String("d".to_owned()),
                Value::String("tag".to_owned()) => Value::String("foo".to_owned()),
            ]);

            let (t, v): (String, BTreeMap<String, String>) =
                de(value, "tag", WithTag::new()).unwrap();

            assert_eq!(t, "foo");
            assert_eq!(
                v,
                map![
                    "a".to_owned() => "b".to_owned(),
                    "c".to_owned() => "d".to_owned(),
                ]
            );
        }

        #[test]
        fn struct_unit_map() {
            use serde_tagged::de::internal::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::Map(map![
                Value::String("tag".to_owned()) => Value::String("foo".to_owned()),
            ]);

            let (t, v): (String, UnitStruct) = de(value, "tag", WithTag::new()).unwrap();

            assert_eq!(t, "foo");
            assert_eq!(v, UnitStruct);
        }

        #[test]
        fn struct_unit_seq() {
            use serde_tagged::de::internal::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::Seq(vec![Value::String("foo".to_owned())]);

            let (t, v): (String, UnitStruct) = de(value, "tag", WithTag::new()).unwrap();

            assert_eq!(t, "foo");
            assert_eq!(v, UnitStruct);
        }

        #[test]
        fn struct_newtype_nonprimitive() {
            use serde_tagged::de::internal::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::Map(map![
                Value::String("foo".to_owned()) => Value::String("bar".to_owned()),
                Value::String("tag".to_owned()) => Value::String("foo".to_owned()),
            ]);

            let (t, v): (String, NewtypeStruct<Struct<String>>) =
                de(value, "tag", WithTag::new()).unwrap();

            assert_eq!(t, "foo");
            assert_eq!(
                v,
                NewtypeStruct(Struct {
                    foo: "bar".to_owned(),
                })
            );
        }

        #[test]
        fn struct_tuple() {
            use serde_tagged::de::internal::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::Seq(vec![
                Value::String("foo".to_owned()),
                Value::I32(1),
                Value::I32(2),
                Value::I32(3),
                Value::I32(4),
            ]);

            let (t, v): (String, TupleStruct<i32, i32, i32, i32>) =
                de(value, "tag", WithTag::new()).unwrap();

            assert_eq!(t, "foo");
            assert_eq!(v, TupleStruct(1, 2, 3, 4));
        }

        #[test]
        fn struct_normal() {
            use serde_tagged::de::internal::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::Map(map![
                Value::String("foo".to_owned()) => Value::String("bar".to_owned()),
                Value::String("tag".to_owned()) => Value::String("foo".to_owned()),
            ]);

            let (t, v): (String, Struct<String>) = de(value, "tag", WithTag::new()).unwrap();

            assert_eq!(t, "foo");
            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn enum_internal_unit() {
            use serde_tagged::de::internal::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::Map(map![
                Value::String("tag".to_owned()) => Value::String("foo".to_owned()),
                Value::String("t".to_owned()) => Value::String("Unit".to_owned()),
            ]);

            let (t, v): (String, EnumTaggedInternal) = de(value, "tag", WithTag::new()).unwrap();

            assert_eq!(t, "foo");
            assert_eq!(v, EnumTaggedInternal::Unit);
        }

        #[test]
        fn enum_internal_newtype_nonprimitive() {
            use serde_tagged::de::internal::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::Map(map![
                Value::String("tag".to_owned()) => Value::String("foo".to_owned()),
                Value::String("t".to_owned()) => Value::String("NewtypeC".to_owned()),
                Value::String("foo".to_owned()) => Value::String("bar".to_owned()),
            ]);

            let (t, v): (String, EnumTaggedInternal) = de(value, "tag", WithTag::new()).unwrap();

            assert_eq!(t, "foo");
            assert_eq!(
                v,
                EnumTaggedInternal::NewtypeC(Struct {
                    foo: "bar".to_owned(),
                })
            );
        }

        #[test]
        fn enum_internal_struct() {
            use serde_tagged::de::internal::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::Map(map![
                Value::String("tag".to_owned()) => Value::String("foo".to_owned()),
                Value::String("t".to_owned()) => Value::String("Struct".to_owned()),
                Value::String("foo".to_owned()) => Value::String("bar".to_owned()),
            ]);

            let (t, v): (String, EnumTaggedInternal) = de(value, "tag", WithTag::new()).unwrap();

            assert_eq!(t, "foo");
            assert_eq!(
                v,
                EnumTaggedInternal::Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn enum_adjacent_unit() {
            use serde_tagged::de::internal::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::Map(map![
                Value::String("tag".to_owned()) => Value::String("foo".to_owned()),
                Value::String("t".to_owned()) => Value::String("Unit".to_owned()),
            ]);

            let (t, v): (String, EnumTaggedAdjacent) = de(value, "tag", WithTag::new()).unwrap();

            assert_eq!(t, "foo");
            assert_eq!(v, EnumTaggedAdjacent::Unit);
        }

        #[test]
        fn enum_adjacent_newtype_primitive() {
            use serde_tagged::de::internal::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::Map(map![
                Value::String("tag".to_owned()) => Value::String("foo".to_owned()),
                Value::String("t".to_owned()) => Value::String("NewtypeP".to_owned()),
                Value::String("c".to_owned()) => Value::I32(42),
            ]);

            let (t, v): (String, EnumTaggedAdjacent) = de(value, "tag", WithTag::new()).unwrap();

            assert_eq!(t, "foo");
            assert_eq!(v, EnumTaggedAdjacent::NewtypeP(42));
        }

        #[test]
        fn enum_adjacent_newtype_nonprimitive() {
            use serde_tagged::de::internal::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::Map(map![
                Value::String("tag".to_owned()) => Value::String("foo".to_owned()),
                Value::String("t".to_owned()) => Value::String("NewtypeC".to_owned()),
                Value::String("c".to_owned()) => Value::Map(map![
                    Value::String("foo".to_owned()) => Value::String("bar".to_owned()),
                ]),
            ]);

            let (t, v): (String, EnumTaggedAdjacent) = de(value, "tag", WithTag::new()).unwrap();

            assert_eq!(t, "foo");
            assert_eq!(
                v,
                EnumTaggedAdjacent::NewtypeC(Struct {
                    foo: "bar".to_owned(),
                })
            );
        }

        #[test]
        fn enum_adjacent_adjacent_tuple() {
            use serde_tagged::de::internal::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::Map(map![
                Value::String("tag".to_owned()) => Value::String("foo".to_owned()),
                Value::String("t".to_owned()) => Value::String("Tuple".to_owned()),
                Value::String("c".to_owned()) => Value::Seq(vec![
                    Value::I32(3),
                    Value::I32(2),
                    Value::I32(1),
                ]),
            ]);

            let (t, v): (String, EnumTaggedAdjacent) = de(value, "tag", WithTag::new()).unwrap();

            assert_eq!(t, "foo");
            assert_eq!(v, EnumTaggedAdjacent::Tuple(3, 2, 1));
        }

        #[test]
        fn enum_adjacent_adjacent_struct() {
            use serde_tagged::de::internal::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::Map(map![
                Value::String("tag".to_owned()) => Value::String("foo".to_owned()),
                Value::String("t".to_owned()) => Value::String("Struct".to_owned()),
                Value::String("c".to_owned()) => Value::Map(map![
                    Value::String("foo".to_owned()) => Value::String("bar".to_owned()),
                ]),
            ]);

            let (t, v): (String, EnumTaggedAdjacent) = de(value, "tag", WithTag::new()).unwrap();

            assert_eq!(t, "foo");
            assert_eq!(
                v,
                EnumTaggedAdjacent::Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn enum_untagged_newtype_nonprimitive() {
            use serde_tagged::de::internal::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::Map(map![
                Value::String("tag".to_owned()) => Value::String("foo".to_owned()),
                Value::String("foo".to_owned()) => Value::String("bar".to_owned()),
            ]);

            let (t, v): (String, EnumUntagged) = de(value, "tag", WithTag::new()).unwrap();

            assert_eq!(t, "foo");
            assert_eq!(
                v,
                EnumUntagged::NewtypeC(Struct {
                    foo: "bar".to_owned(),
                })
            );
        }

        #[test]
        fn enum_untagged_tuple() {
            use serde_tagged::de::internal::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::Seq(vec![
                Value::String("foo".to_owned()),
                Value::I32(3),
                Value::I32(2),
                Value::I32(1),
            ]);

            let (t, v): (String, EnumUntagged) = de(value, "tag", WithTag::new()).unwrap();

            assert_eq!(t, "foo");
            assert_eq!(v, EnumUntagged::Tuple(3, 2, 1));
        }

        #[test]
        fn enum_untagged_newtype_struct() {
            use serde_tagged::de::internal::deserialize as de;
            use serde_tagged::de::WithTag;

            let value = Value::Map(map![
                Value::String("tag".to_owned()) => Value::String("foo".to_owned()),
                Value::String("baz".to_owned()) => Value::String("bar".to_owned()),
            ]);

            let (t, v): (String, EnumUntagged) = de(value, "tag", WithTag::new()).unwrap();

            assert_eq!(t, "foo");
            assert_eq!(
                v,
                EnumUntagged::Struct {
                    baz: "bar".to_owned(),
                }
            );
        }
    }
}
