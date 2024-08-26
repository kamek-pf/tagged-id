# `tagged-id`

A zero-cost wrapper adding type-safety to ressource identifiers.

[![crates.io](https://img.shields.io/crates/v/tagged-id.svg)](https://crates.io/crates/tagged-id)

## Why ?
This solves two problems:
- If two ressources are identified by the same type (a `User` and a `Device` both being identified by a `i32` for instance), they might be swapped by mistake in application code. The compiler cannot help you with this, but it is of course incorrect.
- When writting code that needs to manipulate ressource identifiers, you often need to lookup what the concrete type of the ID is in order to write function signatures and type annotations. In most cases, the nature of the ID does not matter, so it is more convenient to just treat it as an opaque type.
  
## Examples
```rust compile_fail
use tagged_id::{Id, Identify};

struct User {
  id: Id<User>, // id is a i32
  some_field: String
}

impl Identifiable for User {
  type InnerId = i32;
}

struct Device {
  id: Id<Device>, // id is also a i32
  some_value: i32
}

impl Identifiable for Device {
  type InnerId = i32;
}

fn main() {
  let user_id: Id<User> = Id::new(42);
  let device_id: Id<Device> = Id::new(42);

  // This does not compile since the tags are different.
  assert_eq!(user_id, device_id);
}
```


`Id<T>` inherits core trait implementations of the inner type. For instance, if `InnerId` is `Copy`, then `Id<T>` is also `Copy`. \
`Id<T>` is just a newtype wrapper of the inner type with a trait bound, which makes it a zero cost abstraction.

## Cargo Feature Flags
- `smartstring`: Enable `From<&str>` instance converting to a [`CompactString`](https://docs.rs/smartstring/latest/smartstring/alias/type.CompactString.html). When disabled, an instance for `String` is enabled instead.
- `serde`: Enable serde support for `tagged-id` and dependencies that support it like `smartstring`.
- `sqlx-{postgres,mysql,sqlite}`: Enable [`Encode`](https://docs.rs/sqlx/latest/sqlx/trait.Encode.html) and [`Decode`](https://docs.rs/sqlx/latest/sqlx/trait.Decode.html) instances for transparent use with the corresponding `sqlx` backend.

