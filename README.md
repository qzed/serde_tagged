# Serde Tagged

[![Build Status]][github_actions] [![Coverage]][codecov] [![crates.io]][crates-io] [![docs.rs]][docs-rs]

[Build Status]: https://github.com/qzed/serde_tagged/workflows/CI/badge.svg
[github_actions]: https://github.com/qzed/serde_tagged/actions/workflows/ci.yml
[Coverage]: https://codecov.io/gh/qzed/serde_tagged/branch/master/graph/badge.svg
[codecov]: https://codecov.io/gh/qzed/serde_tagged
[crates.io]: https://img.shields.io/crates/v/serde_tagged.svg
[crates-io]: https://crates.io/crates/serde_tagged
[docs.rs]: https://docs.rs/serde_tagged/badge.svg
[docs-rs]: https://docs.rs/serde_tagged

Tag values during serialization, retrieve tags during deserialization.

[Serde][serde] is a powerful, efficient and generic serialization framework for the rust language.
It does, however, not (directly) support de-/serialization of trait-objects.
Especially deserialization of a trait-object whose type cannot be determined beforehand requires an additional layer of code to retrieve said type based on information that can be stored in the data format.

This library aims to provide a framework to store tags that can contain type information in combination with a value during serialization, and retrieve them during deserialization.
To this end, multiple tagging-formats are provided, which are independent<sup>[1](#format-restrictions)</sup> of the data format being used.
The tagging-formats are largely similar to the [enum tagging-formats already used in serde][serde-enums].

<a name="myfootnote1">[1]</a>:
Some data formats may however impose restrictions, e.g. JSON objects can only contain strings as keys, which in turn restricts the tag type that can be used with the external tagging-format to strings for a JSON backend.

## Tagging formats

`serde_tagged` supports multiple tagging formats.
Here is a short overview:

### External tagging

The external tagging format applies tags using a map with a single entry, where a tag is the key and the value the value of the entry. In a somewhat illustrative form, this would yield

```text
{ <tag> => <value> }
```

where `value` can be any de-/serializable value, `tag` however may be limited by the format being used (e.g. JSON would only allow strings).
A benefit of this format is that it is somewhat readable when serialized to a human-readable data format but can also be compact when serialized to a binary format.
Furthermore, due to the clear order (tag before value), deserialization can be faster than with some of the other formats (such as internal and non-tuple-based adjacent tagging).
However, for configuration files and primarily text based data formats you might want to look at the internal tagging format.

### Internal tagging

This format tags values internally, meaning that the tag is embedded into the value.
Embedding a tag does however not work with all value types (e.g. primitives such as `i32`).

A big benefit of this format is that it is (subjectively) more readable in configuration files.
A TOML configuration file using this tagging scheme could look somewhat like this:

```toml
[log]
type = "terminal"   # this is the tag
level = "trace"     # this is a value-specific entry
color = "auto"      # this is another value-specific entry
```

Parsing this format, however, requires allocations so you might want to choose another format when the data format you are using is binary and/or you care about performance.

### Adjacent tagging using tuples

The tuple-based adjacent format is similar to the external format compact, easy to deserialize due to its predefined tag value order, however, arguably less readable.
Tag and value pairs are stored as tuples, i.e.

```text
( <tag>, <value> )
```

### Adjacent tagging using maps

The map-based adjacent tagging format applies tags using two map entries, where one entry contains a mapping from tag-key to tag and the other entry a mapping from value-key to value.
Illustrated, this yields

```text
{ <tag-key> => <tag>, <value-key> => <value> }
```

This format again makes more sense when used in a human-readable data format, however also requires potential heap allocations for deserialization due to the order of tag and value being undefined.

### Adjacent tagging using structs

The struct-based adjacent tagging format is similar to the map-based adjacent tagging format, however, here the tagged value is serialized as struct where the keys are the names of the struct fields.
Illustrated, this yields

```text
{ <tag-key>: <tag>, <value-key>: <value> }
```

The representation of this tagging format in the data format largely depends on the latter, thus it can be either compact (msgpack, bincode) or verbose (JSON).

## Usage

Have a look at the [examples][examples] directory.
A good starting point would be the [trait-object example][examples-trait_obj].
This example explains all the relevant details regarding de-/serialization of trait-objects using the external tagging format (other formats can be used quite similar).
Of course you can not only serialize and deserialize trait-objects, but any serializable and deserializable value with a tag.
Also, have a look at the [API documentation][api-doc].

[examples]: https://github.com/qzed/serde_tagged/tree/master/examples
[examples-trait_obj]: https://github.com/qzed/serde_tagged/blob/master/examples/trait_objects.rs
[api-doc]: https://docs.rs/serde_tagged

## Optional features

By default, this crate is built with the `erased` feature enabled (which requires `erased-serde` as dependency).
This feature is intended to simplify the deserialization of type-erased trait objects by providing types and traits to simplify interactions with `erased-serde`.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[serde]: https://github.com/serde-rs/serde
[serde-enums]: https://serde.rs/enum-representations.html
