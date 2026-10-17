/// Exactly one of two possible value types.
///
/// Both variants describe valid alternatives. Use [`Result`] when the variants
/// represent success and failure, or a domain-specific enum for named choices.
///
/// ```
/// use rust_fest::Either;
///
/// let destination: Either<u64, &str> = Either::Right("ferris");
/// let label = match destination {
///     Either::Left(id) => format!("user #{id}"),
///     Either::Right(name) => format!("user @{name}"),
/// };
/// assert_eq!(label, "user @ferris");
/// ```
///
/// With the `serde` feature, the representation is externally tagged, for
/// example `{"Left": 42}` or `{"Right": "ferris"}` in JSON.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Either<L, R> {
    /// A value of the first alternative.
    Left(L),
    /// A value of the second alternative.
    Right(R),
}

impl<L, R> Either<L, R> {
    /// Borrows the contained value, preserving the variant.
    pub const fn as_ref(&self) -> Either<&L, &R> {
        match self {
            Self::Left(value) => Either::Left(value),
            Self::Right(value) => Either::Right(value),
        }
    }

    /// Mutably borrows the contained value, preserving the variant.
    pub fn as_mut(&mut self) -> Either<&mut L, &mut R> {
        match self {
            Self::Left(value) => Either::Left(value),
            Self::Right(value) => Either::Right(value),
        }
    }

    /// Transforms a left value, leaving a right value unchanged.
    pub fn map_left<U>(self, map: impl FnOnce(L) -> U) -> Either<U, R> {
        match self {
            Self::Left(value) => Either::Left(map(value)),
            Self::Right(value) => Either::Right(value),
        }
    }

    /// Transforms a right value, leaving a left value unchanged.
    pub fn map_right<U>(self, map: impl FnOnce(R) -> U) -> Either<L, U> {
        match self {
            Self::Left(value) => Either::Left(value),
            Self::Right(value) => Either::Right(map(value)),
        }
    }
}

impl<T> Either<T, T> {
    /// Removes the variant when both alternatives have the same type.
    pub fn into_inner(self) -> T {
        match self {
            Self::Left(value) | Self::Right(value) => value,
        }
    }
}
