# Changelog

## 0.1.0

Initial release of Rust Fest, a collection of utility types inspired by TypeFest.

- `Tagged<T, Tag>` gives values distinct compile-time identities without storing a tag.
- `NonEmptyVec<T>` guarantees at least one element, including after mutation.
- `NonEmptyString` validates and preserves non-empty UTF-8 text.
- `Either<L, R>` represents two valid alternatives with borrowing and mapping helpers.
- Optional `serde` support preserves non-empty validation during deserialization.
- Supports `no_std` with `alloc`, stable Rust 1.85+, and zero dependencies by default.
- Licensed under MIT OR CC0-1.0.
