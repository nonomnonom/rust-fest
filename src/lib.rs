#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

extern crate alloc;

mod either;
mod non_empty;
#[cfg(feature = "serde")]
mod serde_impl;
mod tagged;

pub use either::Either;
pub use non_empty::{EmptyError, NonEmptyString, NonEmptyVec};
pub use tagged::Tagged;
