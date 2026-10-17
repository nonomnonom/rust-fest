# Changelog

## 0.1.1

- `Either` supports standard iteration, reverse iteration, exact lengths, and fused iteration when its branches do.
- Optional `either` integration converts to and from the ecosystem type, including use with itertools.
- `NonEmptyVec::map` preserves the non-empty guarantee while transforming owned elements.
- Optional `schemars` integration describes all four types, including non-empty constraints, recursive payloads, and transparent tags.
- Default dependencies remain empty; Rust 1.85, `no_std` + `alloc`, and existing Serde formats are preserved.

See [the release changeset](changesets/0.1.1.md) for integration and compatibility details.

## 0.1.0

Initial release of Rust Fest, a collection of essential Rust utility types.

- `Tagged<T, Tag>` gives values distinct compile-time identities without storing a tag.
- `NonEmptyVec<T>` guarantees at least one element, including after mutation.
- `NonEmptyString` validates and preserves non-empty UTF-8 text.
- `Either<L, R>` represents two valid alternatives with borrowing and mapping helpers.
- Optional `serde` support preserves non-empty validation during deserialization.
- Supports `no_std` with `alloc`, stable Rust 1.85+, and zero dependencies by default.
- Licensed under MIT OR CC0-1.0.

See [the release changeset](changesets/0.1.0.md) for use cases, validation, and compatibility details.
