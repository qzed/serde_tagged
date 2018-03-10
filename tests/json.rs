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
        use common::types::*;
        use serde_bytes::Bytes;

        generate_tests_ser_1! {
            use ::common::formats::json::ser::external::serialize_wrapped : wrapped,
            use ::common::formats::json::ser::external::serialize_with_serializer : with_serializer,

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
        use common::types::*;
        use serde_bytes::Bytes;

        generate_tests_ser_1! {
            use ::common::formats::json::ser::adj_tuple::serialize_wrapped : wrapped,
            use ::common::formats::json::ser::adj_tuple::serialize_with_serializer : with_serializer,

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

    /// Tests for serialization of map-based adjacently-tagged values.
    mod adj_map {
        use common::types::*;
        use serde_bytes::Bytes;

        generate_tests_ser_3! {
            use ::common::formats::json::ser::adj_map::serialize_wrapped : wrapped,
            use ::common::formats::json::ser::adj_map::serialize_with_serializer : with_serializer,

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
                    value:  NewtypeStruct(Struct { foo: "bar" }),
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
                    value:  Struct { foo: "bar" },
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
                    value:  EnumTaggedExternal::NewtypeC(Struct { foo: "bar" }),
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
                    value:  EnumTaggedExternal::Struct{ foo: "bar" },
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
                    value:  EnumTaggedInternal::NewtypeC(Struct { foo: "bar" }),
                    expect: json!({
                        "t": "<tag>",
                        "c": { "t": "NewtypeC", "foo": "bar" }
                    }),
                },{
                    case:   enum_internal_struct,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedInternal::Struct{ foo: "bar" },
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
                    value:  EnumTaggedAdjacent::NewtypeC(Struct { foo: "bar" }),
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
                    value:  EnumTaggedAdjacent::Struct{ foo: "bar" },
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
                    value:  EnumUntagged::NewtypeC(Struct { foo: "bar" }),
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
                    value:  EnumUntagged::Struct{ foo: "bar" },
                    expect: json!({
                        "t": "<tag>",
                        "c": { "foo": "bar" }
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
        use common::types::*;
        use serde_bytes::Bytes;

        generate_tests_ser_3! {
            use ::common::formats::json::ser::adj_struc::serialize_wrapped : wrapped,
            use ::common::formats::json::ser::adj_struc::serialize_with_serializer : with_serializer,

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
                    value:  NewtypeStruct(Struct { foo: "bar" }),
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
                    value:  Struct { foo: "bar" },
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
                    value:  EnumTaggedExternal::NewtypeC(Struct { foo: "bar" }),
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
                    value:  EnumTaggedExternal::Struct{ foo: "bar" },
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
                    value:  EnumTaggedInternal::NewtypeC(Struct { foo: "bar" }),
                    expect: json!({
                        "t": "<tag>",
                        "c": { "t": "NewtypeC", "foo": "bar" }
                    }),
                },{
                    case:   enum_internal_struct,
                    tag_k:  "t",
                    tag_v:  "<tag>",
                    key:    "c",
                    value:  EnumTaggedInternal::Struct{ foo: "bar" },
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
                    value:  EnumTaggedAdjacent::NewtypeC(Struct { foo: "bar" }),
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
                    value:  EnumTaggedAdjacent::Struct{ foo: "bar" },
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
                    value:  EnumUntagged::NewtypeC(Struct { foo: "bar" }),
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
                    value:  EnumUntagged::Struct{ foo: "bar" },
                    expect: json!({
                        "t": "<tag>",
                        "c": { "foo": "bar" }
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

    /// Tests for deserialization of tuple-based adjacently-tagged values.
    mod adj_tuple {
        use common::types::*;
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
            use serde_tagged::de::adj::tuple::deserialize as de;

            let json = r###"
            [
                "tag",
                {
                    "foo": "bar"
                }
            ]
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
            use serde_tagged::de::adj::tuple::deserialize as de;

            let json = r###"
            [
                "tag",
                {
                    "foo": "bar"
                }
            ]
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
        fn error_tuple_empty() {
            use serde_tagged::de::WithTag;
            use serde_tagged::de::adj::tuple::deserialize as de;

            let json = "[]";

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v) = de::<_, (&str, &str), _, _>(&mut jde, WithTag::new()).unwrap();
        }

        #[test]
        #[should_panic]
        fn error_tuple_len() {
            use serde_tagged::de::WithTag;
            use serde_tagged::de::adj::tuple::deserialize as de;

            let json = r###"
                [ "a", "b", "c", "d" ]
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v) = de::<_, (&str, &str), _, _>(&mut jde, WithTag::new()).unwrap();
        }

        #[test]
        #[should_panic]
        fn error_type() {
            use serde_tagged::de::WithTag;
            use serde_tagged::de::adj::tuple::deserialize as de;

            let json = "{}";

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v) = de::<_, (&str, &str), _, _>(&mut jde, WithTag::new()).unwrap();
        }
    }

    /// Tests for deserialization of map-based adjacently-tagged values.
    mod adj_map {
        use common::types::*;
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
            let v =
                de::<&str, Struct<String>, &str, _, _, _>(&mut jde, "t", "c", PhantomData).unwrap();

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
            let v =
                de::<&str, Struct<String>, &str, _, _, _>(&mut jde, "t", "c", PhantomData).unwrap();

            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn without_tag_a() {
            use serde_tagged::de::WithoutTag;
            use serde_tagged::de::adj::map::deserialize as de;

            let json = r###"
            {
                "t": "tag",
                "c": { "foo": "bar" }
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let v =
                de::<&str, Struct<String>, &str, _, _, _>(&mut jde, "t", "c", WithoutTag::new())
                    .unwrap();

            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn without_tag_b() {
            use serde_tagged::de::WithoutTag;
            use serde_tagged::de::adj::map::deserialize as de;

            let json = r###"
            {
                "c": { "foo": "bar" },
                "t": "tag"
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let v =
                de::<&str, Struct<String>, &str, _, _, _>(&mut jde, "t", "c", WithoutTag::new())
                    .unwrap();

            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn with_tag_a() {
            use serde_tagged::de::WithTag;
            use serde_tagged::de::adj::map::deserialize as de;

            let json = r###"
            {
                "t": "tag",
                "c": { "foo": "bar" }
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (t, v) =
                de::<_, (&str, Struct<String>), &str, _, _, _>(&mut jde, "t", "c", WithTag::new())
                    .unwrap();

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
            use serde_tagged::de::WithTag;
            use serde_tagged::de::adj::map::deserialize as de;

            let json = r###"
            {
                "c": { "foo": "bar" },
                "t": "tag"
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (t, v) =
                de::<_, (&str, Struct<String>), &str, _, _, _>(&mut jde, "t", "c", WithTag::new())
                    .unwrap();

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
            use serde_tagged::de::adj::map::deserialize as de;

            let json = "{}";

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v) =
                de::<_, (&str, &str), &str, _, _, _>(&mut jde, "t", "c", WithTag::new()).unwrap();
        }

        #[test]
        #[should_panic]
        fn error_map_len() {
            use serde_tagged::de::WithTag;
            use serde_tagged::de::adj::map::deserialize as de;

            let json = r###"{
                "t": "b",
                "c": "d",
                "e": "f",
            }"###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v) =
                de::<_, (&str, &str), &str, _, _, _>(&mut jde, "t", "c", WithTag::new()).unwrap();
        }

        #[test]
        #[should_panic]
        fn error_keys() {
            use serde_tagged::de::WithTag;
            use serde_tagged::de::adj::map::deserialize as de;

            let json = r###"{
                "t": "b",
                "c": "d",
            }"###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v) =
                de::<_, (&str, &str), &str, _, _, _>(&mut jde, "a", "b", WithTag::new()).unwrap();
        }

        #[test]
        #[should_panic]
        fn error_type() {
            use serde_tagged::de::WithTag;
            use serde_tagged::de::adj::map::deserialize as de;

            let json = "[]";

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v) =
                de::<_, (&str, &str), &str, _, _, _>(&mut jde, "t", "c", WithTag::new()).unwrap();
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
        #[should_panic]
        fn known_error_map_empty() {
            use serde_tagged::de::adj::map::deserialize_known as de;

            let json = "{}";

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v) = de::<&str, &str, &str, _, _>(&mut jde, "t", "c").unwrap();
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
            let (_t, _v) = de::<&str, &str, &str, _, _>(&mut jde, "t", "c").unwrap();
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
            let (_t, _v) = de::<&str, &str, &str, _, _>(&mut jde, "a", "b").unwrap();
        }

        #[test]
        #[should_panic]
        fn known_error_type() {
            use serde_tagged::de::adj::map::deserialize_known as de;

            let json = "[]";

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v) = de::<&str, &str, &str, _, _>(&mut jde, "t", "c").unwrap();
        }
    }

    /// Tests for deserialization of struct-based adjacently-tagged values.
    mod adj_struc {
        use common::types::*;
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
            let v = de::<&str, Struct<String>, _, _>(&mut jde, "Tagged", "t", "c", PhantomData)
                .unwrap();

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
            let v = de::<&str, Struct<String>, _, _>(&mut jde, "Tagged", "t", "c", PhantomData)
                .unwrap();

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
            let v = de::<&str, Struct<String>, _, _>(&mut jde, "Tagged", "t", "c", PhantomData)
                .unwrap();

            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn without_tag_a() {
            use serde_tagged::de::WithoutTag;
            use serde_tagged::de::adj::struc::deserialize as de;

            let json = r###"
            {
                "t": "tag",
                "c": { "foo": "bar" }
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let v =
                de::<&str, Struct<String>, _, _>(&mut jde, "Tagged", "t", "c", WithoutTag::new())
                    .unwrap();

            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn without_tag_b() {
            use serde_tagged::de::WithoutTag;
            use serde_tagged::de::adj::struc::deserialize as de;

            let json = r###"
            {
                "c": { "foo": "bar" },
                "t": "tag"
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let v =
                de::<&str, Struct<String>, _, _>(&mut jde, "Tagged", "t", "c", WithoutTag::new())
                    .unwrap();

            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn without_tag_seq() {
            use serde_tagged::de::WithoutTag;
            use serde_tagged::de::adj::struc::deserialize as de;

            let json = r###"
            [ "tag", { "foo": "bar" } ]
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let v =
                de::<&str, Struct<String>, _, _>(&mut jde, "Tagged", "t", "c", WithoutTag::new())
                    .unwrap();

            assert_eq!(
                v,
                Struct {
                    foo: "bar".to_owned(),
                }
            );
        }

        #[test]
        fn with_tag_a() {
            use serde_tagged::de::WithTag;
            use serde_tagged::de::adj::struc::deserialize as de;

            let json = r###"
            {
                "t": "tag",
                "c": { "foo": "bar" }
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (t, v) =
                de::<_, (&str, Struct<String>), _, _>(&mut jde, "Tagged", "t", "c", WithTag::new())
                    .unwrap();

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
            use serde_tagged::de::WithTag;
            use serde_tagged::de::adj::struc::deserialize as de;

            let json = r###"
            {
                "c": { "foo": "bar" },
                "t": "tag"
            }
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (t, v) =
                de::<_, (&str, Struct<String>), _, _>(&mut jde, "Tagged", "t", "c", WithTag::new())
                    .unwrap();

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
            use serde_tagged::de::WithTag;
            use serde_tagged::de::adj::struc::deserialize as de;

            let json = r###"
            [ "tag", { "foo": "bar" } ]
            "###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (t, v) =
                de::<_, (&str, Struct<String>), _, _>(&mut jde, "Tagged", "t", "c", WithTag::new())
                    .unwrap();

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
            use serde_tagged::de::adj::struc::deserialize as de;

            let json = "{}";

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v) =
                de::<_, (&str, &str), _, _>(&mut jde, "Tagged", "t", "c", WithTag::new()).unwrap();
        }

        #[test]
        #[should_panic]
        fn error_seq_empty() {
            use serde_tagged::de::WithTag;
            use serde_tagged::de::adj::struc::deserialize as de;

            let json = "[]";

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v) =
                de::<_, (&str, &str), _, _>(&mut jde, "Tagged", "t", "c", WithTag::new()).unwrap();
        }

        #[test]
        #[should_panic]
        fn error_map_len() {
            use serde_tagged::de::WithTag;
            use serde_tagged::de::adj::struc::deserialize as de;

            let json = r###"{
                "t": "b",
                "c": "d",
                "e": "f",
            }"###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v) =
                de::<_, (&str, &str), _, _>(&mut jde, "Tagged", "t", "c", WithTag::new()).unwrap();
        }

        #[test]
        #[should_panic]
        fn error_keys() {
            use serde_tagged::de::WithTag;
            use serde_tagged::de::adj::struc::deserialize as de;

            let json = r###"{
                "t": "b",
                "c": "d",
            }"###;

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v) =
                de::<_, (&str, &str), _, _>(&mut jde, "Tagged", "a", "b", WithTag::new()).unwrap();
        }

        #[test]
        #[should_panic]
        fn error_type() {
            use serde_tagged::de::WithTag;
            use serde_tagged::de::adj::struc::deserialize as de;

            let json = "null";

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v) =
                de::<_, (&str, &str), _, _>(&mut jde, "Tagged", "t", "c", WithTag::new()).unwrap();
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
            let (t, v) = de::<&str, Struct<String>, _>(&mut jde, "Tagged", "t", "c").unwrap();

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
            let (t, v) = de::<&str, Struct<String>, _>(&mut jde, "Tagged", "t", "c").unwrap();

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
            let (t, v) = de::<&str, Struct<String>, _>(&mut jde, "Tagged", "t", "c").unwrap();

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
            let (_t, _v) = de::<&str, &str, _>(&mut jde, "Tagged", "t", "c").unwrap();
        }

        #[test]
        #[should_panic]
        fn known_error_seq_empty() {
            use serde_tagged::de::adj::struc::deserialize_known as de;

            let json = "[]";

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v) = de::<&str, &str, _>(&mut jde, "Tagged", "t", "c").unwrap();
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
            let (_t, _v) = de::<&str, &str, _>(&mut jde, "Tagged", "t", "c").unwrap();
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
            let (_t, _v) = de::<&str, &str, _>(&mut jde, "Tagged", "a", "b").unwrap();
        }

        #[test]
        #[should_panic]
        fn known_error_type() {
            use serde_tagged::de::adj::struc::deserialize_known as de;

            let json = "null";

            let mut jde = serde_json::Deserializer::from_str(json);
            let (_t, _v) = de::<&str, &str, _>(&mut jde, "Tagged", "t", "c").unwrap();
        }
    }
}
