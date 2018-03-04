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
}
