# Contributing to Rust Fest

Open an issue describing the application problem before proposing a new utility.
Include a small example of the API you would like to use and explain why Rust's
standard types or an existing dependency do not already fit it.

New types should have a clear invariant, documented conversion behavior, and an
API that preserves that invariant. Avoid adding TypeScript-shaped aliases for
concepts already represented by `Option`, `Result`, references, or arrays.

For code changes, include public API tests, runnable documentation, and
`compile_fail` examples when the guarantee depends on rejecting a program.
Serialization must use checked constructors wherever validation is needed.

Run these checks before submitting:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --locked
cargo test --all-features --locked
cargo +1.85.0 test --all-features --locked
cargo doc --all-features --no-deps
```

Keep changes compatible with Rust 1.85 and `no_std` + `alloc`. Update the
changelog for user-visible changes. Contributions are provided under the
project's MIT OR CC0-1.0 licensing terms.
