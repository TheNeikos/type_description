# type_description

![Bors enabled](https://bors.tech/images/badge_small.svg)

This crate provides machine-readable descriptions for types.

The idea is to make types discoverable for users by explaining them in a way
that a _user_ can understand without knowing implementation details (a `u16` is
an "integer with 16 bit")

## Example

One could make configuration types explained with this crate and
show the explanation (in a GUI, web interface, some special config-editor) to
the user.

```rust
use type_description::AsTypeDescription;
use type_description::TypeDescription;
use type_description::TypeKind;
use type_description::Sign;

/// A configuration
#[derive(TypeDescription)]
struct Config {
    /// The bind address
    addr: std::net::SocketAddr,

    /// The Port
    port: u16,
}

let desc = Config::as_type_description();

assert_eq!(desc.name(), "Config");
assert_eq!(desc.doc(), Some("A configuration"));
assert!(std::matches!(desc.kind(), TypeKind::Struct(_)));

match desc.kind() {
    TypeKind::Struct(v) => {
        let first_field = &v[0];
        assert_eq!(first_field.name(), "addr");
        assert_eq!(first_field.doc(), Some("The bind address"));
        assert_eq!(first_field.kind().name(), "String");
        assert_eq!(first_field.kind().doc(), Some("A socket address"));
        assert_eq!(*first_field.kind().kind(), type_description::TypeKind::String);

        let second_field = &v[1];
        assert_eq!(second_field.name(), "port");
        assert_eq!(second_field.doc(), Some("The Port"));
        assert_eq!(second_field.kind().name(), "Integer");
        assert_eq!(second_field.kind().doc(), Some("An unsigned integer with 16 bits"));
        assert_eq!(*second_field.kind().kind(), type_description::TypeKind::Integer { size: 16, sign: Sign::Unsigned });
    }
    _ => unreachable!()
}
```

## Goals

* Follow the `serde` model and be compatible with all
  `serde::{Deserialize, Serialize}` types
* Produce machine-readable description for types implementing `AsTypeDescription`

## Non-Goals

* Allow constructing values through `TypeDescription` (`serde::Deserialize`
  should be preferred for that)
* Any form of reflection

## License

MPL-2.0

