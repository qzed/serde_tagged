//! Tests using `serde_json`.

#![allow(clippy::unreadable_literal)]

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
        use crate::common::types::*;
        use serde_bytes::Bytes;

        generate_tests_ser_1! {
            use crate::common::formats::json::ser::external::serialize_wrapped : wrapped,
            use crate::common::formats::json::ser::external::serialize_with_serializer : with_serializer,

            with {
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
                    value:  NewtypeStruct(Struct { foo: "bar".to_owned() }),
                    expect: json!({"<tag>": { "foo": "bar" }}),
                },{
                    case:   struct_tuple,
                    tag:    "<tag>",
                    value:  TupleStruct(1, 2, 3, 4),
                    expect: json!({"<tag>": [1, 2, 3, 4]}),
                },{
                    case:   struct_normal,
                    tag:    "<tag>",
                    value:  Struct { foo: "bar".to_owned() },
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
                    value:  EnumTaggedExternal::NewtypeC(Struct { foo: "bar".to_owned() }),
                    expect: json!({"<tag>": { "NewtypeC": { "foo": "bar" }}}),
                },{
                    case:   enum_external_tuple,
                    tag:    "<tag>",
                    value:  EnumTaggedExternal::Tuple(3, 2, 1),
                    expect: json!({"<tag>": { "Tuple": [3, 2, 1]}}),
                },{
                    case:   enum_external_struct,
                    tag:    "<tag>",
                    value:  EnumTaggedExternal::Struct { foo: "bar".to_owned() },
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
                    value:  EnumTaggedInternal::NewtypeC(Struct { foo: "bar".to_owned() }),
                    expect: json!({"<tag>": { "t": "NewtypeC", "foo": "bar" }}),
                },{
                    case:   enum_internal_struct,
                    tag:    "<tag>",
                    value:  EnumTaggedInternal::Struct { foo: "bar".to_owned() },
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
                    value:  EnumTaggedAdjacent::NewtypeC(Struct { foo: "bar".to_owned() }),
                    expect: json!({"<tag>": { "t": "NewtypeC", "c": { "foo": "bar" }}}),
                },{
                    case:   enum_adjacent_tuple,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::Tuple(3, 2, 1),
                    expect: json!({"<tag>": { "t": "Tuple", "c": [3, 2, 1]}}),
                },{
                    case:   enum_adjacent_struct,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::Struct { foo: "bar".to_owned() },
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
                    value:  EnumUntagged::NewtypeC(Struct { foo: "bar".to_owned() }),
                    expect: json!({"<tag>": { "foo": "bar" }}),
                },{
                    case:   enum_untagged_tuple,
                    tag:    "<tag>",
                    value:  EnumUntagged::Tuple(3, 2, 1),
                    expect: json!({"<tag>": [3, 2, 1]}),
                },{
                    case:   enum_untagged_struct,
                    tag:    "<tag>",
                    value:  EnumUntagged::Struct { baz: "bar".to_owned() },
                    expect: json!({"<tag>": { "baz": "bar" }}),
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
        use crate::common::types::*;
        use serde_bytes::Bytes;

        generate_tests_ser_1! {
            use crate::common::formats::json::ser::adj_tuple::serialize_wrapped : wrapped,
            use crate::common::formats::json::ser::adj_tuple::serialize_with_serializer : with_serializer,

            with {
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
                    value:  NewtypeStruct(Struct { foo: "bar".to_owned() }),
                    expect: json!(["<tag>", { "foo": "bar" }]),
                },{
                    case:   struct_tuple,
                    tag:    "<tag>",
                    value:  TupleStruct(1, 2, 3, 4),
                    expect: json!(["<tag>", [1, 2, 3, 4]]),
                },{
                    case:   struct_normal,
                    tag:    "<tag>",
                    value:  Struct { foo: "bar".to_owned() },
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
                    value:  EnumTaggedExternal::NewtypeC(Struct { foo: "bar".to_owned() }),
                    expect: json!(["<tag>", { "NewtypeC": { "foo": "bar" }}]),
                },{
                    case:   enum_external_tuple,
                    tag:    "<tag>",
                    value:  EnumTaggedExternal::Tuple(3, 2, 1),
                    expect: json!(["<tag>", { "Tuple": [3, 2, 1]}]),
                },{
                    case:   enum_external_struct,
                    tag:    "<tag>",
                    value:  EnumTaggedExternal::Struct { foo: "bar".to_owned() },
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
                    value:  EnumTaggedInternal::NewtypeC(Struct { foo: "bar".to_owned() }),
                    expect: json!(["<tag>", { "t": "NewtypeC", "foo": "bar" }]),
                },{
                    case:   enum_internal_struct,
                    tag:    "<tag>",
                    value:  EnumTaggedInternal::Struct { foo: "bar".to_owned() },
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
                    value:  EnumTaggedAdjacent::NewtypeC(Struct { foo: "bar".to_owned() }),
                    expect: json!(["<tag>", { "t": "NewtypeC", "c": { "foo": "bar" }}]),
                },{
                    case:   enum_adjacent_tuple,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::Tuple(3, 2, 1),
                    expect: json!(["<tag>", { "t": "Tuple", "c": [3, 2, 1]}]),
                },{
                    case:   enum_adjacent_struct,
                    tag:    "<tag>",
                    value:  EnumTaggedAdjacent::Struct { foo: "bar".to_owned() },
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
                    value:  EnumUntagged::NewtypeC(Struct { foo: "bar".to_owned() }),
                    expect: json!(["<tag>", { "foo": "bar" }]),
                },{
                    case:   enum_untagged_tuple,
                    tag:    "<tag>",
                    value:  EnumUntagged::Tuple(3, 2, 1),
                    expect: json!(["<tag>", [3, 2, 1]]),
                },{
                    case:   enum_untagged_struct,
                    tag:    "<tag>",
                    value:  EnumUntagged::Struct { baz: "bar".to_owned() },
                    expect: json!(["<tag>", { "baz": "bar" }]),
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

    /// Tests for serialization of map-based adjacently-tagged values.
    mod adj_map {
        use crate::common::types::*;
        use serde_bytes::Bytes;

        generate_tests_ser_3! {
            use crate::common::formats::json::ser::adj_map::serialize_wrapped : wrapped,
            use crate::common::formats::json::ser::adj_map::serialize_with_serializer : with_serializer,

            with {
                {
                    case:   bool,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  true,
                    expect: json!({
                        "t": "<tag>",
                        "c": true
                    }),
                },

                {
                    case:   i8,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  -56_i8,
                    expect: json!({
                        "t": "<tag>",
                        "c": -56
                    }),
                },{
                    case:   i16,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  -197_i16,
                    expect: json!({
                        "t": "<tag>",
                        "c": -197
                    }),
                },{
                    case:   i32,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  49206_i32,
                    expect: json!({
                        "t": "<tag>",
                        "c": 49206
                    }),
                },{
                    case:   i64,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  -817696_i64,
                    expect: json!({
                        "t": "<tag>",
                        "c":  -817696
                    }),
                },

                {
                    case:   u8,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  234_u8,
                    expect: json!({
                        "t": "<tag>",
                        "c": 234
                    }),
                },{
                    case:   u16,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  25507_u16,
                    expect: json!({
                        "t": "<tag>",
                        "c": 25507
                    }),
                },{
                    case:   u32,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  2051984_u32,
                    expect: json!({
                        "t": "<tag>",
                        "c": 2051984
                    }),
                },{
                    case:   u64,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  3331520_u64,
                    expect: json!({
                        "t": "<tag>",
                        "c": 3331520
                    }),
                },

                {
                    case:   f32,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  2.0_f32,
                    expect: json!({
                        "t": "<tag>",
                        "c": 2.0
                    }),
                },{
                    case:   f64,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  2.0_f64,
                    expect: json!({
                        "t": "<tag>",
                        "c": 2.0
                    }),
                },

                {
                    case:   char,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  'c',
                    expect: json!({
                        "t": "<tag>",
                        "c": 'c'
                    }),
                },{
                    case:   str,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  "foobar",
                    expect: json!({
                        "t": "<tag>",
                        "c": "foobar"
                    }),
                },

                {
                    case:   bytes,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  Bytes::new(&[0, 1, 2, 3]),
                    expect: json!({
                        "t": "<tag>",
                        "c": [0, 1, 2, 3]
                    }),
                },

                {
                    case:   none,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  None as Option<i32>,
                    expect: json!({
                        "t": "<tag>",
                        "c": null
                    }),
                },{
                    case:   some,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  Some(361) as Option<i32>,
                    expect: json!({
                        "t": "<tag>",
                        "c": 361
                    }),
                },{
                    case:   unit,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  (),
                    expect: json!({
                        "t": "<tag>",
                        "c": null
                    }),
                },

                {
                    case:   tuple,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  (1_i32, 2_i32, 3_i32),
                    expect: json!({
                        "t": "<tag>",
                        "c": [1, 2, 3]
                    }),
                },{
                    case:   seq,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  SerializeSeq(&vec![1_i32, 2_i32, 3_i32]),
                    expect: json!({
                        "t": "<tag>",
                        "c": [1, 2, 3]
                    }),
                },{
                    case:   seq_len_hidden,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  SerializeSeqLenHidden(&vec![1_i32, 2_i32, 3_i32]),
                    expect: json!({
                        "t": "<tag>",
                        "c": [1, 2, 3]
                    }),
                },{
                    case:   map,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  SerializeMap(&map!["a" => 1_i32, "b" => 2_i32, "c" => 3_i32]),
                    expect: json!({
                        "t": "<tag>",
                        "c": {"a": 1, "b": 2, "c": 3}
                    }),
                },{
                    case:   map_len_hidden,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  SerializeMapLenHidden(&map!["a" => 1_i32, "b" => 2_i32, "c" => 3_i32]),
                    expect: json!({
                        "t": "<tag>",
                        "c": {"a": 1, "b": 2, "c": 3}
                    }),
                },

                {
                    case:   struct_unit,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  UnitStruct,
                    expect: json!({
                        "t": "<tag>",
                        "c": null
                    }),
                },{
                    case:   struct_newtype_primitive,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  NewtypeStruct(42),
                    expect: json!({
                        "t": "<tag>",
                        "c": 42
                    }),
                },{
                    case:   struct_newtype_nonprimitive,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  NewtypeStruct(Struct { foo: "bar".to_owned() }),
                    expect: json!({
                        "t": "<tag>",
                        "c": { "foo": "bar" }
                    }),
                },{
                    case:   struct_tuple,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  TupleStruct(1, 2, 3, 4),
                    expect: json!({
                        "t": "<tag>",
                        "c": [1, 2, 3, 4]
                    }),
                },{
                    case:   struct_normal,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  Struct { foo: "bar".to_owned() },
                    expect: json!({
                        "t": "<tag>",
                        "c": { "foo": "bar" }
                    }),
                },

                {
                    case:   enum_external_unit,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedExternal::Unit,
                    expect: json!({
                        "t": "<tag>",
                        "c": "Unit"
                    }),
                },{
                    case:   enum_external_newtype_primitive,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedExternal::NewtypeP(42),
                    expect: json!({
                        "t": "<tag>",
                        "c": { "NewtypeP": 42 }
                    }),
                },{
                    case:   enum_external_newtype_nonprimitive,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedExternal::NewtypeC(Struct { foo: "bar".to_owned() }),
                    expect: json!({
                        "t": "<tag>",
                        "c": { "NewtypeC": { "foo": "bar" }}
                    }),
                },{
                    case:   enum_external_tuple,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedExternal::Tuple(3, 2, 1),
                    expect: json!({
                        "t": "<tag>",
                        "c": { "Tuple": [3, 2, 1]}
                    }),
                },{
                    case:   enum_external_struct,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedExternal::Struct { foo: "bar".to_owned() },
                    expect: json!({
                        "t": "<tag>",
                        "c": { "Struct": { "foo": "bar" }}
                    }),
                },

                {
                    case:   enum_internal_unit,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedInternal::Unit,
                    expect: json!({
                        "t": "<tag>",
                        "c": { "t": "Unit" }
                    }),
                },{
                    case:   enum_internal_newtype_nonprimitive,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedInternal::NewtypeC(Struct { foo: "bar".to_owned() }),
                    expect: json!({
                        "t": "<tag>",
                        "c": { "t": "NewtypeC", "foo": "bar" }
                    }),
                },{
                    case:   enum_internal_struct,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedInternal::Struct { foo: "bar".to_owned() },
                    expect: json!({
                        "t": "<tag>",
                        "c": { "t": "Struct", "foo": "bar" }
                    }),
                },

                {
                    case:   enum_adjacent_unit,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedAdjacent::Unit,
                    expect: json!({
                        "t": "<tag>",
                        "c": { "t": "Unit" }
                    }),
                },{
                    case:   enum_adjacent_newtype_primitive,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedAdjacent::NewtypeP(42),
                    expect: json!({
                        "t": "<tag>",
                        "c": { "t": "NewtypeP", "c": 42 }
                    }),
                },{
                    case:   enum_adjacent_newtype_nonprimitive,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedAdjacent::NewtypeC(Struct { foo: "bar".to_owned() }),
                    expect: json!({
                        "t": "<tag>",
                        "c": { "t": "NewtypeC", "c": { "foo": "bar" }}
                    }),
                },{
                    case:   enum_adjacent_tuple,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedAdjacent::Tuple(3, 2, 1),
                    expect: json!({
                        "t": "<tag>",
                        "c": { "t": "Tuple", "c": [3, 2, 1]}
                    }),
                },{
                    case:   enum_adjacent_struct,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedAdjacent::Struct { foo: "bar".to_owned() },
                    expect: json!({
                        "t": "<tag>",
                        "c": { "t": "Struct", "c": { "foo": "bar" }}
                    }),
                },

                {
                    case:   enum_untagged_unit,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumUntagged::Unit,
                    expect: json!({
                        "t": "<tag>",
                        "c": null
                    }),
                },{
                    case:   enum_untagged_newtype_primitive,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumUntagged::NewtypeP(42),
                    expect: json!({
                        "t": "<tag>",
                        "c": 42
                    }),
                },{
                    case:   enum_untagged_newtype_nonprimitive,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumUntagged::NewtypeC(Struct { foo: "bar".to_owned() }),
                    expect: json!({
                        "t": "<tag>",
                        "c": { "foo": "bar" }
                    }),
                },{
                    case:   enum_untagged_tuple,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumUntagged::Tuple(3, 2, 1),
                    expect: json!({
                        "t": "<tag>",
                        "c": [3, 2, 1]
                    }),
                },{
                    case:   enum_untagged_struct,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumUntagged::Struct { baz: "bar".to_owned() },
                    expect: json!({
                        "t": "<tag>",
                        "c": { "baz": "bar" }
                    }),
                },

                {
                    case:   collect_seq,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  CollectSeq(vec![1, 2, 3, 4]),
                    expect: json!({
                        "t": "<tag>",
                        "c": [1, 2, 3, 4]
                    }),
                },{
                    case:   collect_map,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  CollectMap(map!["a" => 1, "b" => 2, "c" => 3]),
                    expect: json!({
                        "t": "<tag>",
                        "c": {"a": 1, "b": 2, "c": 3}
                    }),
                },{
                    case:   collect_str,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  CollectStr("foobar"),
                    expect: json!({
                        "t": "<tag>",
                        "c": "foobar"
                    }),
                },
            }
        }
    }

    /// Tests for serialization of struct-based adjacently-tagged values.
    mod adj_struc {
        use crate::common::types::*;
        use serde_bytes::Bytes;

        generate_tests_ser_3! {
            use crate::common::formats::json::ser::adj_struc::serialize_wrapped : wrapped,
            use crate::common::formats::json::ser::adj_struc::serialize_with_serializer : with_serializer,

            with {
                {
                    case:   bool,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  true,
                    expect: json!({
                        "t": "<tag>",
                        "c": true
                    }),
                },

                {
                    case:   i8,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  -56_i8,
                    expect: json!({
                        "t": "<tag>",
                        "c": -56
                    }),
                },{
                    case:   i16,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  -197_i16,
                    expect: json!({
                        "t": "<tag>",
                        "c": -197
                    }),
                },{
                    case:   i32,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  49206_i32,
                    expect: json!({
                        "t": "<tag>",
                        "c": 49206
                    }),
                },{
                    case:   i64,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  -817696_i64,
                    expect: json!({
                        "t": "<tag>",
                        "c":  -817696
                    }),
                },

                {
                    case:   u8,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  234_u8,
                    expect: json!({
                        "t": "<tag>",
                        "c": 234
                    }),
                },{
                    case:   u16,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  25507_u16,
                    expect: json!({
                        "t": "<tag>",
                        "c": 25507
                    }),
                },{
                    case:   u32,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  2051984_u32,
                    expect: json!({
                        "t": "<tag>",
                        "c": 2051984
                    }),
                },{
                    case:   u64,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  3331520_u64,
                    expect: json!({
                        "t": "<tag>",
                        "c": 3331520
                    }),
                },

                {
                    case:   f32,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  2.0_f32,
                    expect: json!({
                        "t": "<tag>",
                        "c": 2.0
                    }),
                },{
                    case:   f64,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  2.0_f64,
                    expect: json!({
                        "t": "<tag>",
                        "c": 2.0
                    }),
                },

                {
                    case:   char,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  'c',
                    expect: json!({
                        "t": "<tag>",
                        "c": 'c'
                    }),
                },{
                    case:   str,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  "foobar",
                    expect: json!({
                        "t": "<tag>",
                        "c": "foobar"
                    }),
                },

                {
                    case:   bytes,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  Bytes::new(&[0, 1, 2, 3]),
                    expect: json!({
                        "t": "<tag>",
                        "c": [0, 1, 2, 3]
                    }),
                },

                {
                    case:   none,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  None as Option<i32>,
                    expect: json!({
                        "t": "<tag>",
                        "c": null
                    }),
                },{
                    case:   some,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  Some(361) as Option<i32>,
                    expect: json!({
                        "t": "<tag>",
                        "c": 361
                    }),
                },{
                    case:   unit,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  (),
                    expect: json!({
                        "t": "<tag>",
                        "c": null
                    }),
                },

                {
                    case:   tuple,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  (1_i32, 2_i32, 3_i32),
                    expect: json!({
                        "t": "<tag>",
                        "c": [1, 2, 3]
                    }),
                },{
                    case:   seq,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  SerializeSeq(&vec![1_i32, 2_i32, 3_i32]),
                    expect: json!({
                        "t": "<tag>",
                        "c": [1, 2, 3]
                    }),
                },{
                    case:   seq_len_hidden,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  SerializeSeqLenHidden(&vec![1_i32, 2_i32, 3_i32]),
                    expect: json!({
                        "t": "<tag>",
                        "c": [1, 2, 3]
                    }),
                },{
                    case:   map,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  SerializeMap(&map!["a" => 1_i32, "b" => 2_i32, "c" => 3_i32]),
                    expect: json!({
                        "t": "<tag>",
                        "c": {"a": 1, "b": 2, "c": 3}
                    }),
                },{
                    case:   map_len_hidden,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  SerializeMapLenHidden(&map!["a" => 1_i32, "b" => 2_i32, "c" => 3_i32]),
                    expect: json!({
                        "t": "<tag>",
                        "c": {"a": 1, "b": 2, "c": 3}
                    }),
                },

                {
                    case:   struct_unit,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  UnitStruct,
                    expect: json!({
                        "t": "<tag>",
                        "c": null
                    }),
                },{
                    case:   struct_newtype_primitive,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  NewtypeStruct(42),
                    expect: json!({
                        "t": "<tag>",
                        "c": 42
                    }),
                },{
                    case:   struct_newtype_nonprimitive,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  NewtypeStruct(Struct { foo: "bar".to_owned() }),
                    expect: json!({
                        "t": "<tag>",
                        "c": { "foo": "bar" }
                    }),
                },{
                    case:   struct_tuple,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  TupleStruct(1, 2, 3, 4),
                    expect: json!({
                        "t": "<tag>",
                        "c": [1, 2, 3, 4]
                    }),
                },{
                    case:   struct_normal,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  Struct { foo: "bar".to_owned() },
                    expect: json!({
                        "t": "<tag>",
                        "c": { "foo": "bar" }
                    }),
                },

                {
                    case:   enum_external_unit,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedExternal::Unit,
                    expect: json!({
                        "t": "<tag>",
                        "c": "Unit"
                    }),
                },{
                    case:   enum_external_newtype_primitive,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedExternal::NewtypeP(42),
                    expect: json!({
                        "t": "<tag>",
                        "c": { "NewtypeP": 42 }
                    }),
                },{
                    case:   enum_external_newtype_nonprimitive,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedExternal::NewtypeC(Struct { foo: "bar".to_owned() }),
                    expect: json!({
                        "t": "<tag>",
                        "c": { "NewtypeC": { "foo": "bar" }}
                    }),
                },{
                    case:   enum_external_tuple,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedExternal::Tuple(3, 2, 1),
                    expect: json!({
                        "t": "<tag>",
                        "c": { "Tuple": [3, 2, 1]}
                    }),
                },{
                    case:   enum_external_struct,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedExternal::Struct { foo: "bar".to_owned() },
                    expect: json!({
                        "t": "<tag>",
                        "c": { "Struct": { "foo": "bar" }}
                    }),
                },

                {
                    case:   enum_internal_unit,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedInternal::Unit,
                    expect: json!({
                        "t": "<tag>",
                        "c": { "t": "Unit" }
                    }),
                },{
                    case:   enum_internal_newtype_nonprimitive,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedInternal::NewtypeC(Struct { foo: "bar".to_owned() }),
                    expect: json!({
                        "t": "<tag>",
                        "c": { "t": "NewtypeC", "foo": "bar" }
                    }),
                },{
                    case:   enum_internal_struct,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedInternal::Struct { foo: "bar".to_owned() },
                    expect: json!({
                        "t": "<tag>",
                        "c": { "t": "Struct", "foo": "bar" }
                    }),
                },

                {
                    case:   enum_adjacent_unit,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedAdjacent::Unit,
                    expect: json!({
                        "t": "<tag>",
                        "c": { "t": "Unit" }
                    }),
                },{
                    case:   enum_adjacent_newtype_primitive,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedAdjacent::NewtypeP(42),
                    expect: json!({
                        "t": "<tag>",
                        "c": { "t": "NewtypeP", "c": 42 }
                    }),
                },{
                    case:   enum_adjacent_newtype_nonprimitive,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedAdjacent::NewtypeC(Struct { foo: "bar".to_owned() }),
                    expect: json!({
                        "t": "<tag>",
                        "c": { "t": "NewtypeC", "c": { "foo": "bar" }}
                    }),
                },{
                    case:   enum_adjacent_tuple,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedAdjacent::Tuple(3, 2, 1),
                    expect: json!({
                        "t": "<tag>",
                        "c": { "t": "Tuple", "c": [3, 2, 1]}
                    }),
                },{
                    case:   enum_adjacent_struct,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedAdjacent::Struct { foo: "bar".to_owned() },
                    expect: json!({
                        "t": "<tag>",
                        "c": { "t": "Struct", "c": { "foo": "bar" }}
                    }),
                },

                {
                    case:   enum_untagged_unit,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumUntagged::Unit,
                    expect: json!({
                        "t": "<tag>",
                        "c": null
                    }),
                },{
                    case:   enum_untagged_newtype_primitive,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumUntagged::NewtypeP(42),
                    expect: json!({
                        "t": "<tag>",
                        "c": 42
                    }),
                },{
                    case:   enum_untagged_newtype_nonprimitive,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumUntagged::NewtypeC(Struct { foo: "bar".to_owned() }),
                    expect: json!({
                        "t": "<tag>",
                        "c": { "foo": "bar" }
                    }),
                },{
                    case:   enum_untagged_tuple,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumUntagged::Tuple(3, 2, 1),
                    expect: json!({
                        "t": "<tag>",
                        "c": [3, 2, 1]
                    }),
                },{
                    case:   enum_untagged_struct,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumUntagged::Struct { baz: "bar".to_owned() },
                    expect: json!({
                        "t": "<tag>",
                        "c": { "baz": "bar" }
                    }),
                },

                {
                    case:   collect_seq,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  CollectSeq(vec![1, 2, 3, 4]),
                    expect: json!({
                        "t": "<tag>",
                        "c": [1, 2, 3, 4]
                    }),
                },{
                    case:   collect_map,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  CollectMap(map!["a" => 1, "b" => 2, "c" => 3]),
                    expect: json!({
                        "t": "<tag>",
                        "c": {"a": 1, "b": 2, "c": 3}
                    }),
                },{
                    case:   collect_str,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  CollectStr("foobar"),
                    expect: json!({
                        "t": "<tag>",
                        "c": "foobar"
                    }),
                },
            }
        }
    }

    /// Tests for serialization of internally-tagged values.
    mod internal {
        use crate::common::types::*;
        use serde_bytes::Bytes;

        generate_tests_ser_2! {
            use crate::common::formats::json::ser::internal::serialize: serialize,

            with {
                {
                    case:   tuple,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  (1_i32, 2_i32, 3_i32),
                    expect: json!(["<tag>", 1, 2, 3]),
                },{
                    case:   seq,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  SerializeSeq(&vec![1_i32, 2_i32, 3_i32]),
                    expect: json!(["<tag>", 1, 2, 3]),
                },{
                    case:   seq_len_hidden,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  SerializeSeqLenHidden(&vec![1_i32, 2_i32, 3_i32]),
                    expect: json!(["<tag>", 1, 2, 3]),
                },{
                    case:   map,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  SerializeMap(&map!["a" => 1_i32, "b" => 2_i32, "c" => 3_i32]),
                    expect: json!({
                        "tag": "<tag>",
                        "a": 1,
                        "b": 2,
                        "c": 3
                    }),
                },{
                    case:   map_len_hidden,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  SerializeMapLenHidden(&map!["a" => 1_i32, "b" => 2_i32, "c" => 3_i32]),
                    expect: json!({
                        "tag": "<tag>",
                        "a": 1,
                        "b": 2,
                        "c": 3
                    }),
                },

                {
                    case:   struct_unit,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  UnitStruct,
                    expect: json!({
                        "tag": "<tag>"
                    }),
                },{
                    case:   struct_newtype_nonprimitive,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  NewtypeStruct(Struct { foo: "bar".to_owned() }),
                    expect: json!({
                        "tag": "<tag>",
                        "foo": "bar"
                    }),
                },{
                    case:   struct_tuple,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  TupleStruct(1, 2, 3, 4),
                    expect: json!(["<tag>", 1, 2, 3, 4]),
                },{
                    case:   struct_normal,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  Struct { foo: "bar".to_owned() },
                    expect: json!({
                        "tag": "<tag>",
                        "foo": "bar"
                    }),
                },

                {
                    case:   enum_internal_unit,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  EnumTaggedInternal::Unit,
                    expect: json!({
                        "tag": "<tag>",
                        "t": "Unit"
                    }),
                },{
                    case:   enum_internal_newtype_nonprimitive,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  EnumTaggedInternal::NewtypeC(Struct { foo: "bar".to_owned() }),
                    expect: json!({
                        "tag": "<tag>",
                        "t": "NewtypeC",
                        "foo": "bar"
                    }),
                },{
                    case:   enum_internal_struct,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  EnumTaggedInternal::Struct { foo: "bar".to_owned() },
                    expect: json!({
                        "tag": "<tag>",
                        "t": "Struct",
                        "foo": "bar"
                    }),
                },

                {
                    case:   enum_adjacent_unit,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  EnumTaggedAdjacent::Unit,
                    expect: json!({
                        "tag": "<tag>",
                        "t": "Unit"
                    }),
                },{
                    case:   enum_adjacent_newtype_primitive,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  EnumTaggedAdjacent::NewtypeP(42),
                    expect: json!({
                        "tag": "<tag>",
                        "t": "NewtypeP",
                        "c": 42
                    }),
                },{
                    case:   enum_adjacent_newtype_nonprimitive,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  EnumTaggedAdjacent::NewtypeC(Struct { foo: "bar".to_owned() }),
                    expect: json!({
                        "tag": "<tag>",
                        "t": "NewtypeC",
                        "c": { "foo": "bar" }
                    }),
                },{
                    case:   enum_adjacent_tuple,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  EnumTaggedAdjacent::Tuple(3, 2, 1),
                    expect: json!({
                        "tag": "<tag>",
                        "t": "Tuple",
                        "c": [3, 2, 1]
                    }),
                },{
                    case:   enum_adjacent_struct,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  EnumTaggedAdjacent::Struct { foo: "bar".to_owned() },
                    expect: json!({
                        "tag": "<tag>",
                        "t": "Struct",
                        "c": { "foo": "bar" }
                    }),
                },

                {
                    case:   enum_untagged_newtype_nonprimitive,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  EnumUntagged::NewtypeC(Struct { foo: "bar".to_owned() }),
                    expect: json!({
                        "tag": "<tag>",
                        "foo": "bar"
                    }),
                },{
                    case:   enum_untagged_tuple,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  EnumUntagged::Tuple(3, 2, 1),
                    expect: json!(["<tag>", 3, 2, 1]),
                },{
                    case:   enum_untagged_struct,
                    tag_k:  "tag",
                    tag_v:  "<tag>",
                    value:  EnumUntagged::Struct { baz: "bar".to_owned() },
                    expect: json!({
                        "tag": "<tag>",
                        "baz": "bar"
                    }),
                },
            }
        }

        #[test]
        #[should_panic]
        fn i8() {
            use crate::common::formats::json::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &42_i8).unwrap();
        }

        #[test]
        #[should_panic]
        fn i16() {
            use crate::common::formats::json::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &42_i16).unwrap();
        }

        #[test]
        #[should_panic]
        fn i32() {
            use crate::common::formats::json::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &42_i32).unwrap();
        }

        #[test]
        #[should_panic]
        fn i64() {
            use crate::common::formats::json::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &42_i64).unwrap();
        }

        #[test]
        #[should_panic]
        fn u8() {
            use crate::common::formats::json::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &42_u8).unwrap();
        }

        #[test]
        #[should_panic]
        fn u16() {
            use crate::common::formats::json::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &42_u16).unwrap();
        }

        #[test]
        #[should_panic]
        fn u32() {
            use crate::common::formats::json::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &42_u32).unwrap();
        }

        #[test]
        #[should_panic]
        fn u64() {
            use crate::common::formats::json::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &42_u64).unwrap();
        }

        #[test]
        #[should_panic]
        fn f32() {
            use crate::common::formats::json::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &2.0_f32).unwrap();
        }

        #[test]
        #[should_panic]
        fn f64() {
            use crate::common::formats::json::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &2.0_f64).unwrap();
        }

        #[test]
        #[should_panic]
        fn char() {
            use crate::common::formats::json::ser::internal::serialize;
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
            use crate::common::formats::json::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &Bytes::new(&[0, 1, 2, 3])).unwrap();
        }

        #[test]
        #[should_panic]
        fn none() {
            use crate::common::formats::json::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &None as &Option<i32>).unwrap();
        }

        #[test]
        #[should_panic]
        fn some() {
            use crate::common::formats::json::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &Some(42_i32)).unwrap();
        }

        #[test]
        #[should_panic]
        fn unit() {
            use crate::common::formats::json::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &()).unwrap();
        }

        #[test]
        #[should_panic]
        fn enum_external_unit() {
            use crate::common::formats::json::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &EnumTaggedExternal::Unit).unwrap();
        }

        #[test]
        #[should_panic]
        fn enum_external_newtype_primitive() {
            use crate::common::formats::json::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &EnumTaggedExternal::NewtypeP(42)).unwrap();
        }

        #[test]
        #[should_panic]
        fn enum_external_newtype_nonprimitive() {
            use crate::common::formats::json::ser::internal::serialize;
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
            use crate::common::formats::json::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &EnumTaggedExternal::Tuple(3, 4, 5)).unwrap();
        }

        #[test]
        #[should_panic]
        fn enum_external_struct() {
            use crate::common::formats::json::ser::internal::serialize;
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
            use crate::common::formats::json::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &EnumUntagged::Unit).unwrap();
        }

        #[test]
        #[should_panic]
        fn enum_untagged_newtype_primitive() {
            use crate::common::formats::json::ser::internal::serialize;
            let _v = serialize("tag", "<tag>", &EnumUntagged::NewtypeP(42)).unwrap();
        }
    }
}

/// Tests for deserialization of tagged values.
mod de {

    /// Tests for deserialization of externally-tagged values.
    mod external {
        use crate::common::types::*;
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
            let v: Struct<String> = de::<&str, _, _>(&mut jde, PhantomData).unwrap();

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

            let json = r###"
            {
                "tag": {
                    "foo": "bar"
                }
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let v: Struct<String> = de::<&str, _, _>(&mut jde, WithoutTag::new()).unwrap();

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

            let json = r###"
            {
                "tag": {
                    "foo": "bar"
                }
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (t, v): (&str, Struct<String>) = de(&mut jde, WithTag::new()).unwrap();

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

            let json = "{}";

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v): (&str, &str) = de(&mut jde, WithTag::new()).unwrap();
        }

        #[test]
        #[should_panic]
        fn error_map_len() {
            use serde_tagged::de::external::deserialize as de;
            use serde_tagged::de::WithTag;

            let json = r###"{
                "a": "b",
                "c": "d",
            }"###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v): (&str, &str) = de(&mut jde, WithTag::new()).unwrap();
        }

        #[test]
        #[should_panic]
        fn error_type() {
            use serde_tagged::de::external::deserialize as de;
            use serde_tagged::de::WithTag;

            let json = "[]";

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v): (&str, &str) = de(&mut jde, WithTag::new()).unwrap();
        }
    }

    /// Tests for deserialization of tuple-based adjacently-tagged values.
    mod adj_tuple {
        use crate::common::types::*;
        use serde_json;

        #[test]
        fn without_tag_phantom() {
            use serde_tagged::de::adj::tuple::deserialize as de;
            use std::marker::PhantomData;

            let json = r###"
            [
                "tag",
                {
                    "foo": "bar"
                }
            ]
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let v: Struct<String> = de::<&str, _, _>(&mut jde, PhantomData).unwrap();

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

            let json = r###"
            [
                "tag",
                {
                    "foo": "bar"
                }
            ]
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let v: Struct<String> = de::<&str, _, _>(&mut jde, WithoutTag::new()).unwrap();

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

            let json = r###"
            [
                "tag",
                {
                    "foo": "bar"
                }
            ]
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (t, v): (&str, Struct<String>) = de(&mut jde, WithTag::new()).unwrap();

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

            let json = "[]";

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v): (&str, &str) = de(&mut jde, WithTag::new()).unwrap();
        }

        #[test]
        #[should_panic]
        fn error_tuple_len() {
            use serde_tagged::de::adj::tuple::deserialize as de;
            use serde_tagged::de::WithTag;

            let json = r###"
                [ "a", "b", "c", "d" ]
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v): (&str, &str) = de(&mut jde, WithTag::new()).unwrap();
        }

        #[test]
        #[should_panic]
        fn error_type() {
            use serde_tagged::de::adj::tuple::deserialize as de;
            use serde_tagged::de::WithTag;

            let json = "{}";

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v): (&str, &str) = de(&mut jde, WithTag::new()).unwrap();
        }
    }

    /// Tests for deserialization of map-based adjacently-tagged values.
    mod adj_map {
        use crate::common::types::*;
        use serde_json;

        #[test]
        fn without_tag_phantom_a() {
            use serde_tagged::de::adj::map::deserialize as de;
            use std::marker::PhantomData;

            let json = r###"
            {
                "t": "tag",
                "c": { "foo": "bar" }
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let v: Struct<String> =
                de::<&str, &str, _, _, _>(&mut jde, "t", "c", PhantomData).unwrap();

            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn without_tag_phantom_b() {
            use serde_tagged::de::adj::map::deserialize as de;
            use std::marker::PhantomData;

            let json = r###"
            {
                "c": { "foo": "bar" },
                "t": "tag"
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let v: Struct<String> =
                de::<&str, &str, _, _, _>(&mut jde, "t", "c", PhantomData).unwrap();

            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn without_tag_a() {
            use serde_tagged::de::adj::map::deserialize as de;
            use serde_tagged::de::WithoutTag;

            let json = r###"
            {
                "t": "tag",
                "c": { "foo": "bar" }
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let v: Struct<String> =
                de::<&str, &str, _, _, _>(&mut jde, "t", "c", WithoutTag::new()).unwrap();

            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn without_tag_b() {
            use serde_tagged::de::adj::map::deserialize as de;
            use serde_tagged::de::WithoutTag;

            let json = r###"
            {
                "c": { "foo": "bar" },
                "t": "tag"
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let v: Struct<String> =
                de::<&str, &str, _, _, _>(&mut jde, "t", "c", WithoutTag::new()).unwrap();

            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn with_tag_a() {
            use serde_tagged::de::adj::map::deserialize as de;
            use serde_tagged::de::WithTag;

            let json = r###"
            {
                "t": "tag",
                "c": { "foo": "bar" }
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (t, v): (&str, Struct<String>) =
                de::<_, &str, _, _, _>(&mut jde, "t", "c", WithTag::new()).unwrap();

            assert_eq!(t, "tag");
            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn with_tag_b() {
            use serde_tagged::de::adj::map::deserialize as de;
            use serde_tagged::de::WithTag;

            let json = r###"
            {
                "c": { "foo": "bar" },
                "t": "tag"
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (t, v): (&str, Struct<String>) =
                de::<_, &str, _, _, _>(&mut jde, "t", "c", WithTag::new()).unwrap();

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

            let json = "{}";

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v): (&str, &str) =
                de::<_, &str, _, _, _>(&mut jde, "t", "c", WithTag::new()).unwrap();
        }

        #[test]
        #[should_panic]
        fn error_map_len() {
            use serde_tagged::de::adj::map::deserialize as de;
            use serde_tagged::de::WithTag;

            let json = r###"{
                "t": "b",
                "c": "d",
                "e": "f",
            }"###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v): (&str, &str) =
                de::<_, &str, _, _, _>(&mut jde, "t", "c", WithTag::new()).unwrap();
        }

        #[test]
        #[should_panic]
        fn error_keys() {
            use serde_tagged::de::adj::map::deserialize as de;
            use serde_tagged::de::WithTag;

            let json = r###"{
                "t": "b",
                "c": "d",
            }"###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v): (&str, &str) =
                de::<_, &str, _, _, _>(&mut jde, "a", "b", WithTag::new()).unwrap();
        }

        #[test]
        #[should_panic]
        fn error_type() {
            use serde_tagged::de::adj::map::deserialize as de;
            use serde_tagged::de::WithTag;

            let json = "[]";

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v): (&str, &str) =
                de::<_, &str, _, _, _>(&mut jde, "t", "c", WithTag::new()).unwrap();
        }

        #[test]
        fn known_a() {
            use serde_tagged::de::adj::map::deserialize_known as de;

            let json = r###"
            {
                "t": "tag",
                "c": { "foo": "bar" }
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (t, v) = de::<&str, Struct<String>, &str, _, _>(&mut jde, "t", "c").unwrap();

            assert_eq!(t, "tag");
            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn known_b() {
            use serde_tagged::de::adj::map::deserialize_known as de;

            let json = r###"
            {
                "c": { "foo": "bar" },
                "t": "tag"
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (t, v): (&str, Struct<String>) =
                de::<_, _, &str, _, _>(&mut jde, "t", "c").unwrap();

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

            let json = "{}";

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v): (&str, &str) = de::<_, _, &str, _, _>(&mut jde, "t", "c").unwrap();
        }

        #[test]
        #[should_panic]
        fn known_error_map_len() {
            use serde_tagged::de::adj::map::deserialize_known as de;

            let json = r###"{
                "t": "b",
                "c": "d",
                "e": "f",
            }"###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v): (&str, &str) = de::<_, _, &str, _, _>(&mut jde, "t", "c").unwrap();
        }

        #[test]
        #[should_panic]
        fn known_error_keys() {
            use serde_tagged::de::adj::map::deserialize_known as de;

            let json = r###"{
                "t": "b",
                "c": "d",
            }"###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v): (&str, &str) = de::<_, _, &str, _, _>(&mut jde, "a", "b").unwrap();
        }

        #[test]
        #[should_panic]
        fn known_error_type() {
            use serde_tagged::de::adj::map::deserialize_known as de;

            let json = "[]";

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v): (&str, &str) = de::<_, _, &str, _, _>(&mut jde, "t", "c").unwrap();
        }
    }

    /// Tests for deserialization of struct-based adjacently-tagged values.
    mod adj_struc {
        use crate::common::types::*;
        use serde_json;

        #[test]
        fn without_tag_phantom_a() {
            use serde_tagged::de::adj::struc::deserialize as de;
            use std::marker::PhantomData;

            let json = r###"
            {
                "t": "tag",
                "c": { "foo": "bar" }
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let v: Struct<String> =
                de::<&str, _, _>(&mut jde, "Tagged", "t", "c", PhantomData).unwrap();

            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn without_tag_phantom_b() {
            use serde_tagged::de::adj::struc::deserialize as de;
            use std::marker::PhantomData;

            let json = r###"
            {
                "c": { "foo": "bar" },
                "t": "tag"
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let v: Struct<String> =
                de::<&str, _, _>(&mut jde, "Tagged", "t", "c", PhantomData).unwrap();

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

            let json = r###"
            [ "tag", { "foo": "bar" } ]
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let v: Struct<String> =
                de::<&str, _, _>(&mut jde, "Tagged", "t", "c", PhantomData).unwrap();

            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn without_tag_a() {
            use serde_tagged::de::adj::struc::deserialize as de;
            use serde_tagged::de::WithoutTag;

            let json = r###"
            {
                "t": "tag",
                "c": { "foo": "bar" }
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let v: Struct<String> =
                de::<&str, _, _>(&mut jde, "Tagged", "t", "c", WithoutTag::new()).unwrap();

            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn without_tag_b() {
            use serde_tagged::de::adj::struc::deserialize as de;
            use serde_tagged::de::WithoutTag;

            let json = r###"
            {
                "c": { "foo": "bar" },
                "t": "tag"
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let v: Struct<String> =
                de::<&str, _, _>(&mut jde, "Tagged", "t", "c", WithoutTag::new()).unwrap();

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

            let json = r###"
            [ "tag", { "foo": "bar" } ]
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let v: Struct<String> =
                de::<&str, _, _>(&mut jde, "Tagged", "t", "c", WithoutTag::new()).unwrap();

            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn with_tag_a() {
            use serde_tagged::de::adj::struc::deserialize as de;
            use serde_tagged::de::WithTag;

            let json = r###"
            {
                "t": "tag",
                "c": { "foo": "bar" }
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (t, v): (&str, Struct<String>) =
                de(&mut jde, "Tagged", "t", "c", WithTag::new()).unwrap();

            assert_eq!(t, "tag");
            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn with_tag_b() {
            use serde_tagged::de::adj::struc::deserialize as de;
            use serde_tagged::de::WithTag;

            let json = r###"
            {
                "c": { "foo": "bar" },
                "t": "tag"
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (t, v): (&str, Struct<String>) =
                de(&mut jde, "Tagged", "t", "c", WithTag::new()).unwrap();

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

            let json = r###"
            [ "tag", { "foo": "bar" } ]
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (t, v): (&str, Struct<String>) =
                de(&mut jde, "Tagged", "t", "c", WithTag::new()).unwrap();

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

            let json = "{}";

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v): (&str, &str) = de(&mut jde, "Tagged", "t", "c", WithTag::new()).unwrap();
        }

        #[test]
        #[should_panic]
        fn error_seq_empty() {
            use serde_tagged::de::adj::struc::deserialize as de;
            use serde_tagged::de::WithTag;

            let json = "[]";

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v): (&str, &str) = de(&mut jde, "Tagged", "t", "c", WithTag::new()).unwrap();
        }

        #[test]
        #[should_panic]
        fn error_map_len() {
            use serde_tagged::de::adj::struc::deserialize as de;
            use serde_tagged::de::WithTag;

            let json = r###"{
                "t": "b",
                "c": "d",
                "e": "f",
            }"###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v): (&str, &str) = de(&mut jde, "Tagged", "t", "c", WithTag::new()).unwrap();
        }

        #[test]
        #[should_panic]
        fn error_keys() {
            use serde_tagged::de::adj::struc::deserialize as de;
            use serde_tagged::de::WithTag;

            let json = r###"{
                "t": "b",
                "c": "d",
            }"###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v): (&str, &str) = de(&mut jde, "Tagged", "a", "b", WithTag::new()).unwrap();
        }

        #[test]
        #[should_panic]
        fn error_type() {
            use serde_tagged::de::adj::struc::deserialize as de;
            use serde_tagged::de::WithTag;

            let json = "null";

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v): (&str, &str) = de(&mut jde, "Tagged", "t", "c", WithTag::new()).unwrap();
        }

        #[test]
        fn known_a() {
            use serde_tagged::de::adj::struc::deserialize_known as de;

            let json = r###"
            {
                "t": "tag",
                "c": { "foo": "bar" }
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (t, v): (&str, Struct<String>) = de(&mut jde, "Tagged", "t", "c").unwrap();

            assert_eq!(t, "tag");
            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn known_b() {
            use serde_tagged::de::adj::struc::deserialize_known as de;

            let json = r###"
            {
                "c": { "foo": "bar" },
                "t": "tag"
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (t, v): (&str, Struct<String>) = de(&mut jde, "Tagged", "t", "c").unwrap();

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

            let json = r###"
            [ "tag", { "foo": "bar" } ]
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (t, v): (&str, Struct<String>) = de(&mut jde, "Tagged", "t", "c").unwrap();

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

            let json = "{}";

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v): (&str, Struct<String>) = de(&mut jde, "Tagged", "t", "c").unwrap();
        }

        #[test]
        #[should_panic]
        fn known_error_seq_empty() {
            use serde_tagged::de::adj::struc::deserialize_known as de;

            let json = "[]";

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v): (&str, &str) = de(&mut jde, "Tagged", "t", "c").unwrap();
        }

        #[test]
        #[should_panic]
        fn known_error_map_len() {
            use serde_tagged::de::adj::struc::deserialize_known as de;

            let json = r###"{
                "t": "b",
                "c": "d",
                "e": "f",
            }"###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v): (&str, &str) = de(&mut jde, "Tagged", "t", "c").unwrap();
        }

        #[test]
        #[should_panic]
        fn known_error_keys() {
            use serde_tagged::de::adj::struc::deserialize_known as de;

            let json = r###"{
                "t": "b",
                "c": "d",
            }"###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v): (&str, &str) = de(&mut jde, "Tagged", "t", "c").unwrap();
        }

        #[test]
        #[should_panic]
        fn known_error_type() {
            use serde_tagged::de::adj::struc::deserialize_known as de;

            let json = "null";

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v): (&str, &str) = de(&mut jde, "Tagged", "t", "c").unwrap();
        }
    }

    /// Tests for deserialization of internally-tagged values.
    mod internal {
        use crate::common::types::*;

        use serde_json;
        use std::collections::BTreeMap;

        #[test]
        fn seq() {
            use serde_tagged::de::internal::deserialize as de;
            use serde_tagged::de::WithTag;

            let json = r###"
                ["foo", 1, 2, 3, 4]
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (t, v): (&str, Vec<i32>) = de(&mut jde, "tag", WithTag::new()).unwrap();

            assert_eq!(t, "foo");
            assert_eq!(v, [1, 2, 3, 4]);
        }

        #[test]
        fn map() {
            use serde_tagged::de::internal::deserialize as de;
            use serde_tagged::de::WithTag;

            let json = r###"
            {
                "a": "b",
                "tag": "foo",
                "c": "d"
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (t, v): (&str, BTreeMap<String, String>) =
                de(&mut jde, "tag", WithTag::new()).unwrap();

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

            let json = r###"
            {
                "tag": "foo"
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (t, v): (&str, UnitStruct) = de(&mut jde, "tag", WithTag::new()).unwrap();

            assert_eq!(t, "foo");
            assert_eq!(v, UnitStruct);
        }

        #[test]
        fn struct_unit_seq() {
            use serde_tagged::de::internal::deserialize as de;
            use serde_tagged::de::WithTag;

            let json = r###"
            [ "foo" ]
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (t, v): (&str, UnitStruct) = de(&mut jde, "tag", WithTag::new()).unwrap();

            assert_eq!(t, "foo");
            assert_eq!(v, UnitStruct);
        }

        #[test]
        fn struct_newtype_nonprimitive() {
            use serde_tagged::de::internal::deserialize as de;
            use serde_tagged::de::WithTag;

            let json = r###"
            {
                "tag": "foo",
                "foo": "bar"
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (t, v): (&str, NewtypeStruct<Struct<String>>) =
                de(&mut jde, "tag", WithTag::new()).unwrap();

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

            let json = r###"
            [ "foo", 1, 2, 3, 4 ]
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (t, v): (&str, TupleStruct<i32, i32, i32, i32>) =
                de(&mut jde, "tag", WithTag::new()).unwrap();

            assert_eq!(t, "foo");
            assert_eq!(v, TupleStruct(1, 2, 3, 4));
        }

        #[test]
        fn struct_normal() {
            use serde_tagged::de::internal::deserialize as de;
            use serde_tagged::de::WithTag;

            let json = r###"
            {
                "tag": "foo",
                "foo": "bar"
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (t, v): (&str, Struct<String>) = de(&mut jde, "tag", WithTag::new()).unwrap();

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

            let json = r###"
            {
                "tag": "foo",
                "t": "Unit"
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (t, v): (&str, EnumTaggedInternal) = de(&mut jde, "tag", WithTag::new()).unwrap();

            assert_eq!(t, "foo");
            assert_eq!(v, EnumTaggedInternal::Unit);
        }

        #[test]
        fn enum_internal_newtype_nonprimitive() {
            use serde_tagged::de::internal::deserialize as de;
            use serde_tagged::de::WithTag;

            let json = r###"
            {
                "tag": "foo",
                "t": "NewtypeC",
                "foo": "bar"
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (t, v): (&str, EnumTaggedInternal) = de(&mut jde, "tag", WithTag::new()).unwrap();

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

            let json = r###"
            {
                "tag": "foo",
                "t": "Struct",
                "foo": "bar"
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (t, v): (&str, EnumTaggedInternal) = de(&mut jde, "tag", WithTag::new()).unwrap();

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

            let json = r###"
            {
                "tag": "foo",
                "t": "Unit"
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (t, v): (&str, EnumTaggedAdjacent) = de(&mut jde, "tag", WithTag::new()).unwrap();

            assert_eq!(t, "foo");
            assert_eq!(v, EnumTaggedAdjacent::Unit);
        }

        #[test]
        fn enum_adjacent_newtype_primitive() {
            use serde_tagged::de::internal::deserialize as de;
            use serde_tagged::de::WithTag;

            let json = r###"
            {
                "tag": "foo",
                "t": "NewtypeP",
                "c": 42
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (t, v): (&str, EnumTaggedAdjacent) = de(&mut jde, "tag", WithTag::new()).unwrap();

            assert_eq!(t, "foo");
            assert_eq!(v, EnumTaggedAdjacent::NewtypeP(42));
        }

        #[test]
        fn enum_adjacent_newtype_nonprimitive() {
            use serde_tagged::de::internal::deserialize as de;
            use serde_tagged::de::WithTag;

            let json = r###"
            {
                "tag": "foo",
                "t": "NewtypeC",
                "c": { "foo": "bar" }
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (t, v): (&str, EnumTaggedAdjacent) = de(&mut jde, "tag", WithTag::new()).unwrap();

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

            let json = r###"
            {
                "tag": "foo",
                "t": "Tuple",
                "c": [3, 2, 1]
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (t, v): (&str, EnumTaggedAdjacent) = de(&mut jde, "tag", WithTag::new()).unwrap();

            assert_eq!(t, "foo");
            assert_eq!(v, EnumTaggedAdjacent::Tuple(3, 2, 1));
        }

        #[test]
        fn enum_adjacent_adjacent_struct() {
            use serde_tagged::de::internal::deserialize as de;
            use serde_tagged::de::WithTag;

            let json = r###"
            {
                "tag": "foo",
                "t": "Struct",
                "c": { "foo": "bar" }
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (t, v): (&str, EnumTaggedAdjacent) = de(&mut jde, "tag", WithTag::new()).unwrap();

            assert_eq!(t, "foo");
            assert_eq!(
                v,
                EnumTaggedAdjacent::Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn enum_untagged_tuple() {
            use serde_tagged::de::internal::deserialize as de;
            use serde_tagged::de::WithTag;

            let json = r###"
                [ "foo", 3, 2, 1]
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (t, v): (&str, EnumUntagged) = de(&mut jde, "tag", WithTag::new()).unwrap();

            assert_eq!(t, "foo");
            assert_eq!(v, EnumUntagged::Tuple(3, 2, 1));
        }

        #[test]
        fn enum_untagged_newtype_nonprimitive() {
            use serde_tagged::de::internal::deserialize as de;
            use serde_tagged::de::WithTag;

            let json = r###"
            {
                "tag": "foo",
                "foo": "bar"
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (t, v): (&str, EnumUntagged) = de(&mut jde, "tag", WithTag::new()).unwrap();

            assert_eq!(t, "foo");
            assert_eq!(
                v,
                EnumUntagged::NewtypeC(Struct {
                    foo: "bar".to_owned(),
                })
            );
        }

        #[test]
        fn enum_untagged_newtype_struct() {
            use serde_tagged::de::internal::deserialize as de;
            use serde_tagged::de::WithTag;

            let json = r###"
            {
                "tag": "foo",
                "baz": "bar"
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (t, v): (&str, EnumUntagged) = de(&mut jde, "tag", WithTag::new()).unwrap();

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
