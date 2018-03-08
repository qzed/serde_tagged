//! Tests using `serde_json`.

#![cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]

extern crate serde;
extern crate serde_bytes;
extern crate serde_tagged;
extern crate serde_value;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;


#[macro_use]
mod common;


/// Tests for serialization of tagged values.
mod ser {

    /// Tests for serialization of externally-tagged values.
    mod external {

        /// Tests for serialization of externally-tagged using the
        /// wrapper-struct.
        mod wrapped {
            use common::types::*;
            use serde_bytes::Bytes;

            generate_tests_ser_1! {
                using: ::common::formats::json::ser::external::serialize_wrapped,

                { case: bool, tag: "<tag>", value: true, expect: json!({"<tag>": true}) },

                { case: i8,  tag: "<tag>", value:     -56_i8,  expect: json!({"<tag>":     -56}) },
                { case: i16, tag: "<tag>", value:    -197_i16, expect: json!({"<tag>":    -197}) },
                { case: i32, tag: "<tag>", value:   49206_i32, expect: json!({"<tag>":   49206}) },
                { case: i64, tag: "<tag>", value: -817696_i64, expect: json!({"<tag>": -817696}) },

                { case: u8,  tag: "<tag>", value:     234_u8,  expect: json!({"<tag>":     234}) },
                { case: u16, tag: "<tag>", value:   25507_u16, expect: json!({"<tag>":   25507}) },
                { case: u32, tag: "<tag>", value: 2051984_u32, expect: json!({"<tag>": 2051984}) },
                { case: u64, tag: "<tag>", value: 3331520_u64, expect: json!({"<tag>": 3331520}) },

                { case: f32, tag: "<tag>", value: 2.0_f32, expect: json!({"<tag>": 2.0}) },
                { case: f64, tag: "<tag>", value: 2.0_f64, expect: json!({"<tag>": 2.0}) },

                { case: char, tag: "<tag>", value: 'c',      expect: json!({"<tag>": 'c'})      },
                { case: str,  tag: "<tag>", value: "foobar", expect: json!({"<tag>": "foobar"}) },

                {
                    case:   bytes,
                    tag:    "<tag>",
                    value:  Bytes::new(&[0, 1, 2, 3]),
                    expect: json!({"<tag>": [0, 1, 2, 3]}),
                },

                {
                    case:   none,
                    tag:    "<tag>",
                    value:  None as Option<i32>,
                    expect: json!({"<tag>": null}),
                },{
                    case:   some,
                    tag:    "<tag>",
                    value:  Some(361) as Option<i32>,
                    expect: json!({"<tag>": 361}),
                },{
                    case:   unit,
                    tag:    "<tag>",
                    value:  (),
                    expect: json!({"<tag>": null}),
                },

                {
                    case:   tuple,
                    tag:    "<tag>",
                    value:  (1_i32, 2_i32, 3_i32),
                    expect: json!({"<tag>": [1, 2, 3]}),
                },{
                    case:   seq,
                    tag:    "<tag>",
                    value:  SerializeSeq(&vec![1_i32, 2_i32, 3_i32]),
                    expect: json!({"<tag>": [1, 2, 3]}),
                },{
                    case:   seq_len_hidden,
                    tag:    "<tag>",
                    value:  SerializeSeqLenHidden(&vec![1_i32, 2_i32, 3_i32]),
                    expect: json!({"<tag>": [1, 2, 3]}),
                },{
                    case:   map,
                    tag:    "<tag>",
                    value:  SerializeMap(&map!["a" => 1_i32, "b" => 2_i32, "c" => 3_i32]),
                    expect: json!({"<tag>": {"a": 1, "b": 2, "c": 3}}),
                },{
                    case:   map_len_hidden,
                    tag:    "<tag>",
                    value:  SerializeMapLenHidden(&map!["a" => 1_i32, "b" => 2_i32, "c" => 3_i32]),
                    expect: json!({"<tag>": {"a": 1, "b": 2, "c": 3}}),
                },

                {
                    case:   struct_unit,
                    tag:    "<tag>",
                    value:  UnitStruct,
                    expect: json!({"<tag>": null}),
                },{
                    case:   struct_newtype_primitive,
                    tag:    "<tag>",
                    value:  NewtypeStruct(42),
                    expect: json!({"<tag>": 42}),
                },{
                    case:   struct_newtype_nonprimitive,
                    tag:    "<tag>",
                    value:  NewtypeStruct(Struct { foo: "bar" }),
                    expect: json!({"<tag>": { "foo": "bar" }}),
                },{
                    case:   struct_tuple,
                    tag:    "<tag>",
                    value:  TupleStruct(1, 2, 3, 4),
                    expect: json!({"<tag>": [1, 2, 3, 4]}),
                },{
                    case:   struct_normal,
                    tag:    "<tag>",
                    value:  Struct { foo: "bar" },
                    expect: json!({"<tag>": { "foo": "bar" }}),
                },

                {
                    case:   enum_external_unit,
                    tag:    "<tag>",
                    value:  EnumTaggedExternal::Unit,
                    expect: json!({"<tag>": "Unit"}),
                },{
                    case:   enum_external_newtype_primitive,
                    tag:    "<tag>",
                    value:  EnumTaggedExternal::NewtypeP(42),
                    expect: json!({"<tag>": { "NewtypeP": 42 }}),
                },{
                    case:   enum_external_newtype_nonprimitive,
                    tag:    "<tag>",
                    value:  EnumTaggedExternal::NewtypeC(Struct { foo: "bar" }),
                    expect: json!({"<tag>": { "NewtypeC": { "foo": "bar" }}}),
                },{
                    case:   enum_external_tuple,
                    tag:    "<tag>",
                    value:  EnumTaggedExternal::Tuple(3, 2, 1),
                    expect: json!({"<tag>": { "Tuple": [3, 2, 1]}}),
                },{
                    case:   enum_external_struct,
                    tag:    "<tag>",
                    value:  EnumTaggedExternal::Struct{ foo: "bar" },
                    expect: json!({"<tag>": { "Struct": { "foo": "bar" }}}),
                },

                {
                    case:   enum_internal_unit,
                    tag:    "<tag>",
                    value:  EnumTaggedInternal::Unit,
                    expect: json!({"<tag>": { "t": "Unit" }}),
                },{
                    case:   enum_internal_newtype_nonprimitive,
                    tag:    "<tag>",
                    value:  EnumTaggedInternal::NewtypeC(Struct { foo: "bar" }),
                    expect: json!({"<tag>": { "t": "NewtypeC", "foo": "bar" }}),
                },{
                    case:   enum_internal_struct,
                    tag:    "<tag>",
                    value:  EnumTaggedInternal::Struct{ foo: "bar" },
                    expect: json!({"<tag>": { "t": "Struct", "foo": "bar" }}),
                },

                {
                    case:   enum_adjacent_unit,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::Unit,
                    expect: json!({"<tag>": { "t": "Unit" }}),
                },{
                    case:   enum_adjacent_newtype_primitive,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::NewtypeP(42),
                    expect: json!({"<tag>": { "t": "NewtypeP", "c": 42 }}),
                },{
                    case:   enum_adjacent_newtype_nonprimitive,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::NewtypeC(Struct { foo: "bar" }),
                    expect: json!({"<tag>": { "t": "NewtypeC", "c": { "foo": "bar" }}}),
                },{
                    case:   enum_adjacent_tuple,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::Tuple(3, 2, 1),
                    expect: json!({"<tag>": { "t": "Tuple", "c": [3, 2, 1]}}),
                },{
                    case:   enum_adjacent_struct,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::Struct{ foo: "bar" },
                    expect: json!({"<tag>": { "t": "Struct", "c": { "foo": "bar" }}}),
                },

                {
                    case:   enum_untagged_unit,
                    tag:    "<tag>",
                    value:  EnumUntagged::Unit,
                    expect: json!({"<tag>": null}),
                },{
                    case:   enum_untagged_newtype_primitive,
                    tag:    "<tag>",
                    value:  EnumUntagged::NewtypeP(42),
                    expect: json!({"<tag>": 42}),
                },{
                    case:   enum_untagged_newtype_nonprimitive,
                    tag:    "<tag>",
                    value:  EnumUntagged::NewtypeC(Struct { foo: "bar" }),
                    expect: json!({"<tag>": { "foo": "bar" }}),
                },{
                    case:   enum_untagged_tuple,
                    tag:    "<tag>",
                    value:  EnumUntagged::Tuple(3, 2, 1),
                    expect: json!({"<tag>": [3, 2, 1]}),
                },{
                    case:   enum_untagged_struct,
                    tag:    "<tag>",
                    value:  EnumUntagged::Struct{ foo: "bar" },
                    expect: json!({"<tag>": { "foo": "bar" }}),
                },

                {
                    case:   collect_seq,
                    tag:    "<tag>",
                    value:  CollectSeq(vec![1, 2, 3, 4]),
                    expect: json!({"<tag>": [1, 2, 3, 4]}),
                },{
                    case:   collect_map,
                    tag:    "<tag>",
                    value:  CollectMap(map!["a" => 1, "b" => 2, "c" => 3]),
                    expect: json!({"<tag>": {"a": 1, "b": 2, "c": 3}}),
                },{
                    case:   collect_str,
                    tag:    "<tag>",
                    value:  CollectStr("foobar"),
                    expect: json!({"<tag>": "foobar"}),
                },
            }
        }

        /// Tests for serialization of externally-tagged using the Serializer.
        mod with_serializer {
            use common::types::*;
            use serde_bytes::Bytes;

            generate_tests_ser_1! {
                using: ::common::formats::json::ser::external::serialize_with_serializer,

                { case: bool, tag: "<tag>", value: true, expect: json!({"<tag>": true}) },

                { case: i8,  tag: "<tag>", value:     -56_i8,  expect: json!({"<tag>":     -56}) },
                { case: i16, tag: "<tag>", value:    -197_i16, expect: json!({"<tag>":    -197}) },
                { case: i32, tag: "<tag>", value:   49206_i32, expect: json!({"<tag>":   49206}) },
                { case: i64, tag: "<tag>", value: -817696_i64, expect: json!({"<tag>": -817696}) },

                { case: u8,  tag: "<tag>", value:     234_u8,  expect: json!({"<tag>":     234}) },
                { case: u16, tag: "<tag>", value:   25507_u16, expect: json!({"<tag>":   25507}) },
                { case: u32, tag: "<tag>", value: 2051984_u32, expect: json!({"<tag>": 2051984}) },
                { case: u64, tag: "<tag>", value: 3331520_u64, expect: json!({"<tag>": 3331520}) },

                { case: f32, tag: "<tag>", value: 2.0_f32, expect: json!({"<tag>": 2.0}) },
                { case: f64, tag: "<tag>", value: 2.0_f64, expect: json!({"<tag>": 2.0}) },

                { case: char, tag: "<tag>", value: 'c',      expect: json!({"<tag>": 'c'})      },
                { case: str,  tag: "<tag>", value: "foobar", expect: json!({"<tag>": "foobar"}) },

                {
                    case:   bytes,
                    tag:    "<tag>",
                    value:  Bytes::new(&[0, 1, 2, 3]),
                    expect: json!({"<tag>": [0, 1, 2, 3]}),
                },

                {
                    case:   none,
                    tag:    "<tag>",
                    value:  None as Option<i32>,
                    expect: json!({"<tag>": null}),
                },{
                    case:   some,
                    tag:    "<tag>",
                    value:  Some(361) as Option<i32>,
                    expect: json!({"<tag>": 361}),
                },{
                    case:   unit,
                    tag:    "<tag>",
                    value:  (),
                    expect: json!({"<tag>": null}),
                },

                {
                    case:   tuple,
                    tag:    "<tag>",
                    value:  (1_i32, 2_i32, 3_i32),
                    expect: json!({"<tag>": [1, 2, 3]}),
                },{
                    case:   seq,
                    tag:    "<tag>",
                    value:  SerializeSeq(&vec![1_i32, 2_i32, 3_i32]),
                    expect: json!({"<tag>": [1, 2, 3]}),
                },{
                    case:   seq_len_hidden,
                    tag:    "<tag>",
                    value:  SerializeSeqLenHidden(&vec![1_i32, 2_i32, 3_i32]),
                    expect: json!({"<tag>": [1, 2, 3]}),
                },{
                    case:   map,
                    tag:    "<tag>",
                    value:  SerializeMap(&map!["a" => 1_i32, "b" => 2_i32, "c" => 3_i32]),
                    expect: json!({"<tag>": {"a": 1, "b": 2, "c": 3}}),
                },{
                    case:   map_len_hidden,
                    tag:    "<tag>",
                    value:  SerializeMapLenHidden(&map!["a" => 1_i32, "b" => 2_i32, "c" => 3_i32]),
                    expect: json!({"<tag>": {"a": 1, "b": 2, "c": 3}}),
                },

                {
                    case:   struct_unit,
                    tag:    "<tag>",
                    value:  UnitStruct,
                    expect: json!({"<tag>": null}),
                },{
                    case:   struct_newtype_primitive,
                    tag:    "<tag>",
                    value:  NewtypeStruct(42),
                    expect: json!({"<tag>": 42}),
                },{
                    case:   struct_newtype_nonprimitive,
                    tag:    "<tag>",
                    value:  NewtypeStruct(Struct { foo: "bar" }),
                    expect: json!({"<tag>": { "foo": "bar" }}),
                },{
                    case:   struct_tuple,
                    tag:    "<tag>",
                    value:  TupleStruct(1, 2, 3, 4),
                    expect: json!({"<tag>": [1, 2, 3, 4]}),
                },{
                    case:   struct_normal,
                    tag:    "<tag>",
                    value:  Struct { foo: "bar" },
                    expect: json!({"<tag>": { "foo": "bar" }}),
                },

                {
                    case:   enum_external_unit,
                    tag:    "<tag>",
                    value:  EnumTaggedExternal::Unit,
                    expect: json!({"<tag>": "Unit"}),
                },{
                    case:   enum_external_newtype_primitive,
                    tag:    "<tag>",
                    value:  EnumTaggedExternal::NewtypeP(42),
                    expect: json!({"<tag>": { "NewtypeP": 42 }}),
                },{
                    case:   enum_external_newtype_nonprimitive,
                    tag:    "<tag>",
                    value:  EnumTaggedExternal::NewtypeC(Struct { foo: "bar" }),
                    expect: json!({"<tag>": { "NewtypeC": { "foo": "bar" }}}),
                },{
                    case:   enum_external_tuple,
                    tag:    "<tag>",
                    value:  EnumTaggedExternal::Tuple(3, 2, 1),
                    expect: json!({"<tag>": { "Tuple": [3, 2, 1]}}),
                },{
                    case:   enum_external_struct,
                    tag:    "<tag>",
                    value:  EnumTaggedExternal::Struct{ foo: "bar" },
                    expect: json!({"<tag>": { "Struct": { "foo": "bar" }}}),
                },

                {
                    case:   enum_internal_unit,
                    tag:    "<tag>",
                    value:  EnumTaggedInternal::Unit,
                    expect: json!({"<tag>": { "t": "Unit" }}),
                },{
                    case:   enum_internal_newtype_nonprimitive,
                    tag:    "<tag>",
                    value:  EnumTaggedInternal::NewtypeC(Struct { foo: "bar" }),
                    expect: json!({"<tag>": { "t": "NewtypeC", "foo": "bar" }}),
                },{
                    case:   enum_internal_struct,
                    tag:    "<tag>",
                    value:  EnumTaggedInternal::Struct{ foo: "bar" },
                    expect: json!({"<tag>": { "t": "Struct", "foo": "bar" }}),
                },

                {
                    case:   enum_adjacent_unit,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::Unit,
                    expect: json!({"<tag>": { "t": "Unit" }}),
                },{
                    case:   enum_adjacent_newtype_primitive,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::NewtypeP(42),
                    expect: json!({"<tag>": { "t": "NewtypeP", "c": 42 }}),
                },{
                    case:   enum_adjacent_newtype_nonprimitive,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::NewtypeC(Struct { foo: "bar" }),
                    expect: json!({"<tag>": { "t": "NewtypeC", "c": { "foo": "bar" }}}),
                },{
                    case:   enum_adjacent_tuple,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::Tuple(3, 2, 1),
                    expect: json!({"<tag>": { "t": "Tuple", "c": [3, 2, 1]}}),
                },{
                    case:   enum_adjacent_struct,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::Struct{ foo: "bar" },
                    expect: json!({"<tag>": { "t": "Struct", "c": { "foo": "bar" }}}),
                },

                {
                    case:   enum_untagged_unit,
                    tag:    "<tag>",
                    value:  EnumUntagged::Unit,
                    expect: json!({"<tag>": null}),
                },{
                    case:   enum_untagged_newtype_primitive,
                    tag:    "<tag>",
                    value:  EnumUntagged::NewtypeP(42),
                    expect: json!({"<tag>": 42}),
                },{
                    case:   enum_untagged_newtype_nonprimitive,
                    tag:    "<tag>",
                    value:  EnumUntagged::NewtypeC(Struct { foo: "bar" }),
                    expect: json!({"<tag>": { "foo": "bar" }}),
                },{
                    case:   enum_untagged_tuple,
                    tag:    "<tag>",
                    value:  EnumUntagged::Tuple(3, 2, 1),
                    expect: json!({"<tag>": [3, 2, 1]}),
                },{
                    case:   enum_untagged_struct,
                    tag:    "<tag>",
                    value:  EnumUntagged::Struct{ foo: "bar" },
                    expect: json!({"<tag>": { "foo": "bar" }}),
                },

                {
                    case:   collect_seq,
                    tag:    "<tag>",
                    value:  CollectSeq(vec![1, 2, 3, 4]),
                    expect: json!({"<tag>": [1, 2, 3, 4]}),
                },{
                    case:   collect_map,
                    tag:    "<tag>",
                    value:  CollectMap(map!["a" => 1, "b" => 2, "c" => 3]),
                    expect: json!({"<tag>": {"a": 1, "b": 2, "c": 3}}),
                },{
                    case:   collect_str,
                    tag:    "<tag>",
                    value:  CollectStr("foobar"),
                    expect: json!({"<tag>": "foobar"}),
                },
            }
        }
    }

    /// Tests for serialization of tuple-based adjacently-tagged values.
    mod adj_tuple {

        /// Tests for serialization of tuple-based adjacently-tagged values
        /// using the wrapper-struct.
        mod wrapped {
            use common::types::*;
            use serde_bytes::Bytes;

            generate_tests_ser_1! {
                using: ::common::formats::json::ser::adj_tuple::serialize_wrapped,

                { case: bool, tag: "<tag>", value: true, expect: json!(["<tag>", true]) },

                { case: i8,  tag: "<tag>", value:     -56_i8,  expect: json!(["<tag>",     -56]) },
                { case: i16, tag: "<tag>", value:    -197_i16, expect: json!(["<tag>",    -197]) },
                { case: i32, tag: "<tag>", value:   49206_i32, expect: json!(["<tag>",   49206]) },
                { case: i64, tag: "<tag>", value: -817696_i64, expect: json!(["<tag>", -817696]) },

                { case: u8,  tag: "<tag>", value:     234_u8,  expect: json!(["<tag>",     234]) },
                { case: u16, tag: "<tag>", value:   25507_u16, expect: json!(["<tag>",   25507]) },
                { case: u32, tag: "<tag>", value: 2051984_u32, expect: json!(["<tag>", 2051984]) },
                { case: u64, tag: "<tag>", value: 3331520_u64, expect: json!(["<tag>", 3331520]) },

                { case: f32, tag: "<tag>", value: 2.0_f32, expect: json!(["<tag>", 2.0]) },
                { case: f64, tag: "<tag>", value: 2.0_f64, expect: json!(["<tag>", 2.0]) },

                { case: char, tag: "<tag>", value: 'c',      expect: json!(["<tag>", 'c'])      },
                { case: str,  tag: "<tag>", value: "foobar", expect: json!(["<tag>", "foobar"]) },

                {
                    case:   bytes,
                    tag:    "<tag>",
                    value:  Bytes::new(&[0, 1, 2, 3]),
                    expect: json!(["<tag>", [0, 1, 2, 3]]),
                },

                {
                    case:   none,
                    tag:    "<tag>",
                    value:  None as Option<i32>,
                    expect: json!(["<tag>", null]),
                },{
                    case:   some,
                    tag:    "<tag>",
                    value:  Some(361) as Option<i32>,
                    expect: json!(["<tag>", 361]),
                },{
                    case:   unit,
                    tag:    "<tag>",
                    value:  (),
                    expect: json!(["<tag>", null]),
                },

                {
                    case:   tuple,
                    tag:    "<tag>",
                    value:  (1_i32, 2_i32, 3_i32),
                    expect: json!(["<tag>", [1, 2, 3]]),
                },{
                    case:   seq,
                    tag:    "<tag>",
                    value:  SerializeSeq(&vec![1_i32, 2_i32, 3_i32]),
                    expect: json!(["<tag>", [1, 2, 3]]),
                },{
                    case:   seq_len_hidden,
                    tag:    "<tag>",
                    value:  SerializeSeqLenHidden(&vec![1_i32, 2_i32, 3_i32]),
                    expect: json!(["<tag>", [1, 2, 3]]),
                },{
                    case:   map,
                    tag:    "<tag>",
                    value:  SerializeMap(&map!["a" => 1_i32, "b" => 2_i32, "c" => 3_i32]),
                    expect: json!(["<tag>", {"a": 1, "b": 2, "c": 3}]),
                },{
                    case:   map_len_hidden,
                    tag:    "<tag>",
                    value:  SerializeMapLenHidden(&map!["a" => 1_i32, "b" => 2_i32, "c" => 3_i32]),
                    expect: json!(["<tag>", {"a": 1, "b": 2, "c": 3}]),
                },

                {
                    case:   struct_unit,
                    tag:    "<tag>",
                    value:  UnitStruct,
                    expect: json!(["<tag>", null]),
                },{
                    case:   struct_newtype_primitive,
                    tag:    "<tag>",
                    value:  NewtypeStruct(42),
                    expect: json!(["<tag>", 42]),
                },{
                    case:   struct_newtype_nonprimitive,
                    tag:    "<tag>",
                    value:  NewtypeStruct(Struct { foo: "bar" }),
                    expect: json!(["<tag>", { "foo": "bar" }]),
                },{
                    case:   struct_tuple,
                    tag:    "<tag>",
                    value:  TupleStruct(1, 2, 3, 4),
                    expect: json!(["<tag>", [1, 2, 3, 4]]),
                },{
                    case:   struct_normal,
                    tag:    "<tag>",
                    value:  Struct { foo: "bar" },
                    expect: json!(["<tag>", { "foo": "bar" }]),
                },

                {
                    case:   enum_external_unit,
                    tag:    "<tag>",
                    value:  EnumTaggedExternal::Unit,
                    expect: json!(["<tag>", "Unit"]),
                },{
                    case:   enum_external_newtype_primitive,
                    tag:    "<tag>",
                    value:  EnumTaggedExternal::NewtypeP(42),
                    expect: json!(["<tag>", { "NewtypeP": 42 }]),
                },{
                    case:   enum_external_newtype_nonprimitive,
                    tag:    "<tag>",
                    value:  EnumTaggedExternal::NewtypeC(Struct { foo: "bar" }),
                    expect: json!(["<tag>", { "NewtypeC": { "foo": "bar" }}]),
                },{
                    case:   enum_external_tuple,
                    tag:    "<tag>",
                    value:  EnumTaggedExternal::Tuple(3, 2, 1),
                    expect: json!(["<tag>", { "Tuple": [3, 2, 1]}]),
                },{
                    case:   enum_external_struct,
                    tag:    "<tag>",
                    value:  EnumTaggedExternal::Struct{ foo: "bar" },
                    expect: json!(["<tag>", { "Struct": { "foo": "bar" }}]),
                },

                {
                    case:   enum_internal_unit,
                    tag:    "<tag>",
                    value:  EnumTaggedInternal::Unit,
                    expect: json!(["<tag>", { "t": "Unit" }]),
                },{
                    case:   enum_internal_newtype_nonprimitive,
                    tag:    "<tag>",
                    value:  EnumTaggedInternal::NewtypeC(Struct { foo: "bar" }),
                    expect: json!(["<tag>", { "t": "NewtypeC", "foo": "bar" }]),
                },{
                    case:   enum_internal_struct,
                    tag:    "<tag>",
                    value:  EnumTaggedInternal::Struct{ foo: "bar" },
                    expect: json!(["<tag>", { "t": "Struct", "foo": "bar" }]),
                },

                {
                    case:   enum_adjacent_unit,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::Unit,
                    expect: json!(["<tag>", { "t": "Unit" }]),
                },{
                    case:   enum_adjacent_newtype_primitive,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::NewtypeP(42),
                    expect: json!(["<tag>", { "t": "NewtypeP", "c": 42 }]),
                },{
                    case:   enum_adjacent_newtype_nonprimitive,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::NewtypeC(Struct { foo: "bar" }),
                    expect: json!(["<tag>", { "t": "NewtypeC", "c": { "foo": "bar" }}]),
                },{
                    case:   enum_adjacent_tuple,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::Tuple(3, 2, 1),
                    expect: json!(["<tag>", { "t": "Tuple", "c": [3, 2, 1]}]),
                },{
                    case:   enum_adjacent_struct,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::Struct{ foo: "bar" },
                    expect: json!(["<tag>", { "t": "Struct", "c": { "foo": "bar" }}]),
                },

                {
                    case:   enum_untagged_unit,
                    tag:    "<tag>",
                    value:  EnumUntagged::Unit,
                    expect: json!(["<tag>", null]),
                },{
                    case:   enum_untagged_newtype_primitive,
                    tag:    "<tag>",
                    value:  EnumUntagged::NewtypeP(42),
                    expect: json!(["<tag>", 42]),
                },{
                    case:   enum_untagged_newtype_nonprimitive,
                    tag:    "<tag>",
                    value:  EnumUntagged::NewtypeC(Struct { foo: "bar" }),
                    expect: json!(["<tag>", { "foo": "bar" }]),
                },{
                    case:   enum_untagged_tuple,
                    tag:    "<tag>",
                    value:  EnumUntagged::Tuple(3, 2, 1),
                    expect: json!(["<tag>", [3, 2, 1]]),
                },{
                    case:   enum_untagged_struct,
                    tag:    "<tag>",
                    value:  EnumUntagged::Struct{ foo: "bar" },
                    expect: json!(["<tag>", { "foo": "bar" }]),
                },

                {
                    case:   collect_seq,
                    tag:    "<tag>",
                    value:  CollectSeq(vec![1, 2, 3, 4]),
                    expect: json!(["<tag>", [1, 2, 3, 4]]),
                },{
                    case:   collect_map,
                    tag:    "<tag>",
                    value:  CollectMap(map!["a" => 1, "b" => 2, "c" => 3]),
                    expect: json!(["<tag>", {"a": 1, "b": 2, "c": 3}]),
                },{
                    case:   collect_str,
                    tag:    "<tag>",
                    value:  CollectStr("foobar"),
                    expect: json!(["<tag>", "foobar"]),
                },
            }
        }

        /// Tests for serialization of tuple-based adjacently-tagged values
        /// using the Serializer.
        mod with_serializer {
            use common::types::*;
            use serde_bytes::Bytes;

            generate_tests_ser_1! {
                using: ::common::formats::json::ser::adj_tuple::serialize_with_serializer,

                { case: bool, tag: "<tag>", value: true, expect: json!(["<tag>", true]) },

                { case: i8,  tag: "<tag>", value:     -56_i8,  expect: json!(["<tag>",     -56]) },
                { case: i16, tag: "<tag>", value:    -197_i16, expect: json!(["<tag>",    -197]) },
                { case: i32, tag: "<tag>", value:   49206_i32, expect: json!(["<tag>",   49206]) },
                { case: i64, tag: "<tag>", value: -817696_i64, expect: json!(["<tag>", -817696]) },

                { case: u8,  tag: "<tag>", value:     234_u8,  expect: json!(["<tag>",     234]) },
                { case: u16, tag: "<tag>", value:   25507_u16, expect: json!(["<tag>",   25507]) },
                { case: u32, tag: "<tag>", value: 2051984_u32, expect: json!(["<tag>", 2051984]) },
                { case: u64, tag: "<tag>", value: 3331520_u64, expect: json!(["<tag>", 3331520]) },

                { case: f32, tag: "<tag>", value: 2.0_f32, expect: json!(["<tag>", 2.0]) },
                { case: f64, tag: "<tag>", value: 2.0_f64, expect: json!(["<tag>", 2.0]) },

                { case: char, tag: "<tag>", value: 'c',      expect: json!(["<tag>", 'c'])      },
                { case: str,  tag: "<tag>", value: "foobar", expect: json!(["<tag>", "foobar"]) },

                {
                    case:   bytes,
                    tag:    "<tag>",
                    value:  Bytes::new(&[0, 1, 2, 3]),
                    expect: json!(["<tag>", [0, 1, 2, 3]]),
                },

                {
                    case:   none,
                    tag:    "<tag>",
                    value:  None as Option<i32>,
                    expect: json!(["<tag>", null]),
                },{
                    case:   some,
                    tag:    "<tag>",
                    value:  Some(361) as Option<i32>,
                    expect: json!(["<tag>", 361]),
                },{
                    case:   unit,
                    tag:    "<tag>",
                    value:  (),
                    expect: json!(["<tag>", null]),
                },

                {
                    case:   tuple,
                    tag:    "<tag>",
                    value:  (1_i32, 2_i32, 3_i32),
                    expect: json!(["<tag>", [1, 2, 3]]),
                },{
                    case:   seq,
                    tag:    "<tag>",
                    value:  SerializeSeq(&vec![1_i32, 2_i32, 3_i32]),
                    expect: json!(["<tag>", [1, 2, 3]]),
                },{
                    case:   seq_len_hidden,
                    tag:    "<tag>",
                    value:  SerializeSeqLenHidden(&vec![1_i32, 2_i32, 3_i32]),
                    expect: json!(["<tag>", [1, 2, 3]]),
                },{
                    case:   map,
                    tag:    "<tag>",
                    value:  SerializeMap(&map!["a" => 1_i32, "b" => 2_i32, "c" => 3_i32]),
                    expect: json!(["<tag>", {"a": 1, "b": 2, "c": 3}]),
                },{
                    case:   map_len_hidden,
                    tag:    "<tag>",
                    value:  SerializeMapLenHidden(&map!["a" => 1_i32, "b" => 2_i32, "c" => 3_i32]),
                    expect: json!(["<tag>", {"a": 1, "b": 2, "c": 3}]),
                },

                {
                    case:   struct_unit,
                    tag:    "<tag>",
                    value:  UnitStruct,
                    expect: json!(["<tag>", null]),
                },{
                    case:   struct_newtype_primitive,
                    tag:    "<tag>",
                    value:  NewtypeStruct(42),
                    expect: json!(["<tag>", 42]),
                },{
                    case:   struct_newtype_nonprimitive,
                    tag:    "<tag>",
                    value:  NewtypeStruct(Struct { foo: "bar" }),
                    expect: json!(["<tag>", { "foo": "bar" }]),
                },{
                    case:   struct_tuple,
                    tag:    "<tag>",
                    value:  TupleStruct(1, 2, 3, 4),
                    expect: json!(["<tag>", [1, 2, 3, 4]]),
                },{
                    case:   struct_normal,
                    tag:    "<tag>",
                    value:  Struct { foo: "bar" },
                    expect: json!(["<tag>", { "foo": "bar" }]),
                },

                {
                    case:   enum_external_unit,
                    tag:    "<tag>",
                    value:  EnumTaggedExternal::Unit,
                    expect: json!(["<tag>", "Unit"]),
                },{
                    case:   enum_external_newtype_primitive,
                    tag:    "<tag>",
                    value:  EnumTaggedExternal::NewtypeP(42),
                    expect: json!(["<tag>", { "NewtypeP": 42 }]),
                },{
                    case:   enum_external_newtype_nonprimitive,
                    tag:    "<tag>",
                    value:  EnumTaggedExternal::NewtypeC(Struct { foo: "bar" }),
                    expect: json!(["<tag>", { "NewtypeC": { "foo": "bar" }}]),
                },{
                    case:   enum_external_tuple,
                    tag:    "<tag>",
                    value:  EnumTaggedExternal::Tuple(3, 2, 1),
                    expect: json!(["<tag>", { "Tuple": [3, 2, 1]}]),
                },{
                    case:   enum_external_struct,
                    tag:    "<tag>",
                    value:  EnumTaggedExternal::Struct{ foo: "bar" },
                    expect: json!(["<tag>", { "Struct": { "foo": "bar" }}]),
                },

                {
                    case:   enum_internal_unit,
                    tag:    "<tag>",
                    value:  EnumTaggedInternal::Unit,
                    expect: json!(["<tag>", { "t": "Unit" }]),
                },{
                    case:   enum_internal_newtype_nonprimitive,
                    tag:    "<tag>",
                    value:  EnumTaggedInternal::NewtypeC(Struct { foo: "bar" }),
                    expect: json!(["<tag>", { "t": "NewtypeC", "foo": "bar" }]),
                },{
                    case:   enum_internal_struct,
                    tag:    "<tag>",
                    value:  EnumTaggedInternal::Struct{ foo: "bar" },
                    expect: json!(["<tag>", { "t": "Struct", "foo": "bar" }]),
                },

                {
                    case:   enum_adjacent_unit,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::Unit,
                    expect: json!(["<tag>", { "t": "Unit" }]),
                },{
                    case:   enum_adjacent_newtype_primitive,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::NewtypeP(42),
                    expect: json!(["<tag>", { "t": "NewtypeP", "c": 42 }]),
                },{
                    case:   enum_adjacent_newtype_nonprimitive,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::NewtypeC(Struct { foo: "bar" }),
                    expect: json!(["<tag>", { "t": "NewtypeC", "c": { "foo": "bar" }}]),
                },{
                    case:   enum_adjacent_tuple,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::Tuple(3, 2, 1),
                    expect: json!(["<tag>", { "t": "Tuple", "c": [3, 2, 1]}]),
                },{
                    case:   enum_adjacent_struct,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::Struct{ foo: "bar" },
                    expect: json!(["<tag>", { "t": "Struct", "c": { "foo": "bar" }}]),
                },

                {
                    case:   enum_untagged_unit,
                    tag:    "<tag>",
                    value:  EnumUntagged::Unit,
                    expect: json!(["<tag>", null]),
                },{
                    case:   enum_untagged_newtype_primitive,
                    tag:    "<tag>",
                    value:  EnumUntagged::NewtypeP(42),
                    expect: json!(["<tag>", 42]),
                },{
                    case:   enum_untagged_newtype_nonprimitive,
                    tag:    "<tag>",
                    value:  EnumUntagged::NewtypeC(Struct { foo: "bar" }),
                    expect: json!(["<tag>", { "foo": "bar" }]),
                },{
                    case:   enum_untagged_tuple,
                    tag:    "<tag>",
                    value:  EnumUntagged::Tuple(3, 2, 1),
                    expect: json!(["<tag>", [3, 2, 1]]),
                },{
                    case:   enum_untagged_struct,
                    tag:    "<tag>",
                    value:  EnumUntagged::Struct{ foo: "bar" },
                    expect: json!(["<tag>", { "foo": "bar" }]),
                },

                {
                    case:   collect_seq,
                    tag:    "<tag>",
                    value:  CollectSeq(vec![1, 2, 3, 4]),
                    expect: json!(["<tag>", [1, 2, 3, 4]]),
                },{
                    case:   collect_map,
                    tag:    "<tag>",
                    value:  CollectMap(map!["a" => 1, "b" => 2, "c" => 3]),
                    expect: json!(["<tag>", {"a": 1, "b": 2, "c": 3}]),
                },{
                    case:   collect_str,
                    tag:    "<tag>",
                    value:  CollectStr("foobar"),
                    expect: json!(["<tag>", "foobar"]),
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
        use serde_json;

        #[test]
        fn without_tag_phantom() {
            use serde_tagged::de::external::deserialize as de;
            use std::marker::PhantomData;

            let json = r###"
            {
                "tag": {
                    "foo": "bar"
                }
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let v = de::<&str, Struct<String>, _, _>(&mut jde, PhantomData).unwrap();

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

            let json = r###"
            {
                "tag": {
                    "foo": "bar"
                }
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let v = de::<&str, Struct<String>, _, _>(&mut jde, WithoutTag::new()).unwrap();

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

            let json = r###"
            {
                "tag": {
                    "foo": "bar"
                }
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (t, v) = de::<_, (&str, Struct<String>), _, _>(&mut jde, WithTag::new()).unwrap();

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

            let json = "{}";

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v) = de::<_, (&str, &str), _, _>(&mut jde, WithTag::new()).unwrap();
        }

        #[test]
        #[should_panic]
        fn error_map_len() {
            use serde_tagged::de::WithTag;
            use serde_tagged::de::external::deserialize as de;

            let json = r###"{
                "a": "b",
                "c": "d",
            }"###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v) = de::<_, (&str, &str), _, _>(&mut jde, WithTag::new()).unwrap();
        }

        #[test]
        #[should_panic]
        fn error_type() {
            use serde_tagged::de::WithTag;
            use serde_tagged::de::external::deserialize as de;

            let json = "[]";

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v) = de::<_, (&str, &str), _, _>(&mut jde, WithTag::new()).unwrap();
        }
    }
}
