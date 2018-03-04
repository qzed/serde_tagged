# Serde Tagged

Tag values during serialization, retreive tags during deserialization.

[Serde][serde] is a powerful, efficient and generic serialization framework for the rust language.
It does, however, not (directly) support de-/serialization of trait-objects.
Especially deserialization of a trait-object whose type cannot be determined beforehand requires an additional layer of code to retreive said type based on information that can be stored in the data format.

This library aims to provide a framework to store tags that can contain type information in combination with a value during serialization, and retreive them during deserialization.
To this end, multiple tagging-formats are provided, which are independent<sup>[1](#format-restrictions)</sup> of the data format being used.
The tagging-formats are largely similar to the [enum tagging-formats already used in serde][serde-enums].

<a name="myfootnote1">[1]</a>:
Some data formats may however impose restrictions, e.g. JSON objects can only contain strings as keys, which in turn restricts the tag type that can be used with the external tagging-format to strings for a JSON backend.

## Currently under development

This crate is currently under development, thus it is not on `crates.io` yet.
Further documentation and examples will follow.


## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[serde]: https://github.com/serde-rs/serde
[serde-enums]: https://serde.rs/enum-representations.html
