# Rust Fest 🦀

A collection of essential Rust utility types.

Rust Fest packages small, reusable types for expressing intent and keeping invalid states out of application code. It supports Rust 1.85+, works with `no_std` + `alloc`, has no dependencies by default, and forbids unsafe code.

## Install

```sh
cargo add rust-fest
```

Or add it to your manifest:

```toml
[dependencies]
rust-fest = "0.1"
```

The Cargo package is named `rust-fest`; the Rust import is `rust_fest`.

```rust
use rust_fest::{Either, NonEmptyString, NonEmptyVec, Tagged};

enum User {}
type UserId = Tagged<u64, User>;

fn notify(recipients: NonEmptyVec<UserId>, message: NonEmptyString) {
    // There is always a first recipient, with no Option to unwrap.
    println!("First recipient: {}", recipients.first());
    println!("Message: {message}");
}

fn main() -> Result<(), rust_fest::EmptyError> {
    let recipients = NonEmptyVec::new(UserId::new(42));
    let message = NonEmptyString::try_from("Welcome to Rust Fest!")?;
    notify(recipients, message);

    let lookup: Either<UserId, &str> = Either::Right("ferris");
    match lookup {
        Either::Left(id) => println!("Look up ID {id}"),
        Either::Right(name) => println!("Look up name {name}"),
    }
    Ok(())
}
```

## Types

| Type | Purpose | Guarantee |
| --- | --- | --- |
| `Tagged<T, Tag>` | Give values a distinct identity, such as user IDs and order IDs | Different tags cannot be mixed implicitly; same layout as `T` |
| `NonEmptyVec<T>` | Represent one or more values | Always contains at least one element |
| `NonEmptyString` | Represent non-empty text | Always contains at least one UTF-8 byte |
| `Either<L, R>` | Represent one of two valid alternatives | Exactly one variant is present |
| `EmptyError` | Report failed non-empty construction | Implements `core::error::Error` (also available through `std`) |

### Tagged values

```rust
use rust_fest::Tagged;

enum User {}
enum Order {}
type UserId = Tagged<u64, User>;
type OrderId = Tagged<u64, Order>;

let user = UserId::new(42);
let order = OrderId::new(42);
assert_eq!(user.into_inner(), order.into_inner());
// let wrong: OrderId = user; // Does not compile: the tags differ.
```

Tags are compile-time labels. `Tagged::new` accepts any value of `T`; it does not validate IDs or establish trust. Use a domain-specific newtype with a private field and checked constructor when creation itself must be restricted. Tag marker types do not need to implement `Clone`, `Copy`, or comparison traits.

### Non-empty values

```rust
use rust_fest::{NonEmptyString, NonEmptyVec};

fn main() -> Result<(), rust_fest::EmptyError> {
    let mut values = NonEmptyVec::try_from(vec![3, 1, 2])?;
    values.as_mut_slice().sort();
    assert_eq!(values.first(), &1);
    assert_eq!(values.last(), &3);

    while values.pop().is_some() {}
    assert_eq!(values.as_slice(), &[1]); // The final element cannot be popped.

    assert!(NonEmptyVec::<i32>::try_from(vec![]).is_err());
    assert!(NonEmptyString::try_from("").is_err());
    assert!(NonEmptyString::try_from(" ").is_ok()); // Non-empty is not non-blank.
    Ok(())
}
```

Constructors validate external values at runtime. Private fields and restricted mutation preserve the invariant afterward. Slice and string methods are available through read-only dereferencing. `NonEmptyVec` also supports owned, borrowed, and mutable iteration. `into_vec` and `into_string` explicitly remove the guarantee by consuming the wrapper.

## Serde support

Enable the optional integration when using these types in serialized data:

```toml
[dependencies]
rust-fest = { version = "0.1", features = ["serde"] }
```

- `Tagged` serializes as its inner value. The tag is not stored or authenticated.
- `NonEmptyVec` serializes as a sequence and rejects empty sequences when deserializing.
- `NonEmptyString` serializes as a string and rejects empty strings when deserializing.
- `Either` uses explicit variant names, for example `{"Left":42}` or `{"Right":"ferris"}` in JSON.

```rust
#[cfg(feature = "serde")]
{
    use rust_fest::NonEmptyVec;

    let recipients: NonEmptyVec<u64> = serde_json::from_str("[42,7]")?;
    assert_eq!(recipients.first(), &42);
    assert!(serde_json::from_str::<NonEmptyVec<u64>>("[]").is_err());
}
# Ok::<(), serde_json::Error>(())
```

Serde is the only optional runtime dependency; its derive support is enabled with this feature. `serde_json` in this example is a separate application dependency. Serialization round trips and rejection of invalid inputs are tested.

## Platform support

The library uses `core` and `alloc`, without relying on `std`. `Tagged` and `Either` do not allocate by themselves. The non-empty owned types need an allocator. No feature flag is required to use the crate in a `no_std` application; Serde support also works without `std`.

## When to use these types

- **Domain IDs:** distinguish `UserId` from `OrderId` even when both are stored as `u64`.
- **Batches and workflows:** require at least one recipient, task, or processing step with `NonEmptyVec`.
- **API payloads and configuration:** reject empty values during Serde deserialization before application logic runs.
- **Alternative inputs:** accept an ID or a name using `Either`, with exhaustive handling of each case.

Use `Option` for optional values and `Result` for failures. Use your own enum when variants deserve domain-specific names. Non-empty values only guarantee presence; bounds, email syntax, whitespace rules, authorization, and other domain checks belong in your application.

For a complete typed notification example, run `cargo run --example usage`. For checked JSON input and round-trip serialization, run `cargo run --example api_payload --features serde`.

## Development

Clone the repository and use Rust 1.85 or newer. For local integration in another project, use `rust-fest = { path = "../rust-fest" }`.

```sh
cargo run --example usage
cargo run --example api_payload --features serde
cargo test
cargo test --all-features
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo doc --all-features --no-deps
cargo package --offline
```

`cargo test` includes documentation tests that prove mismatched tags and operations that empty a non-empty value are rejected by the compiler. Integration tests exercise the crate through its public API. `cargo package` builds and verifies a local archive; it does not publish anything. See [CONTRIBUTING.md](https://github.com/nonomnonom/rust-fest/blob/main/CONTRIBUTING.md) for contribution guidelines.

## License

Available under either the [MIT license](https://github.com/nonomnonom/rust-fest/blob/main/LICENSE-MIT) or [CC0 1.0 Universal](https://github.com/nonomnonom/rust-fest/blob/main/LICENSE-CC0), at your choice.
