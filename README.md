# `tagged-id`

A zero-cost wrapper adding type-safety to resource identifiers.

[![crates.io](https://img.shields.io/crates/v/tagged-id.svg)](https://crates.io/crates/tagged-id)

## Why ?
This solves two problems:
- If two resources are identified by the same type (`Foo` and `Bar` both being identified by a `i32` for instance), they might be swapped by mistake in application code. The compiler cannot help you with this, but it is of course incorrect.
- When writting code that needs to manipulate resource identifiers, you often need to lookup what the concrete type of the ID is in order to write function signatures and type annotations. In most cases, the nature of the ID does not matter, so it is more convenient to just treat it as an opaque type.
  
## Examples
```rust compile_fail
use tagged_id::{Id, Identify};

#[derive(Id)]     // note the derive macro
#[tagged_id(i32)] // this attribute specifies the underlying type
struct Foo {
    id: Id<Foo>,  // id is a i32
    some_field: String,
}

struct Bar {
    id: Id<Bar>,  // id is also a i32, see impl below
    some_value: i32,
}

// This is what the derive macro generates
impl Identify for Bar {
    type InnerId = i32;
}

fn main() {
    let foo_id: Id<Foo> = Id::new(42);
    let bar_id: Id<Bar> = Id::new(42);

    // This does not compile since the tags are different.
    assert_eq!(foo_id, bar_id);
}
```

`Id<T>` inherits core trait implementations of the inner type. For instance, if `InnerId` is `Copy`, then `Id<T>` is also `Copy`. \
`Id<T>` is just a newtype wrapper of the inner type with a trait bound, which makes it a zero cost abstraction.

## Cargo Feature Flags
- `derive`: Enable the derive macro, this feature is enabled by default.
- `smartstring`: Enable `From<&str>` instance converting to a [`CompactString`](https://docs.rs/smartstring/latest/smartstring/alias/type.CompactString.html). When disabled, an instance for `String` is enabled instead.
- `uuid`: Enable `From<Uuid>` instance for convenience.
- `serde`: Enable serde support for `tagged-id` and dependencies that support it like `smartstring`.
- `sqlx-{postgres,mysql,sqlite}`: Enable [`Encode`](https://docs.rs/sqlx/latest/sqlx/trait.Encode.html) and [`Decode`](https://docs.rs/sqlx/latest/sqlx/trait.Decode.html) instances for transparent use with the corresponding `sqlx` backend.

