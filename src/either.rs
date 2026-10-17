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
///
/// When both alternatives are iterators with the same item type, this type is
/// also an iterator. Enable `either` for conversions to and from the `either`
/// crate's type via [`Into::into`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
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

impl<L, R> Iterator for Either<L, R>
where
    L: Iterator,
    R: Iterator<Item = L::Item>,
{
    type Item = L::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Left(iter) => iter.next(),
            Self::Right(iter) => iter.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Self::Left(iter) => iter.size_hint(),
            Self::Right(iter) => iter.size_hint(),
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        match self {
            Self::Left(iter) => iter.nth(n),
            Self::Right(iter) => iter.nth(n),
        }
    }

    fn fold<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        match self {
            Self::Left(iter) => iter.fold(init, f),
            Self::Right(iter) => iter.fold(init, f),
        }
    }
}

impl<L, R> DoubleEndedIterator for Either<L, R>
where
    L: DoubleEndedIterator,
    R: DoubleEndedIterator<Item = L::Item>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        match self {
            Self::Left(iter) => iter.next_back(),
            Self::Right(iter) => iter.next_back(),
        }
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        match self {
            Self::Left(iter) => iter.nth_back(n),
            Self::Right(iter) => iter.nth_back(n),
        }
    }
}

impl<L, R> ExactSizeIterator for Either<L, R>
where
    L: ExactSizeIterator,
    R: ExactSizeIterator<Item = L::Item>,
{
    fn len(&self) -> usize {
        match self {
            Self::Left(iter) => iter.len(),
            Self::Right(iter) => iter.len(),
        }
    }
}

impl<L, R> core::iter::FusedIterator for Either<L, R>
where
    L: core::iter::FusedIterator,
    R: core::iter::FusedIterator<Item = L::Item>,
{
}

#[cfg(feature = "either")]
impl<L, R> From<Either<L, R>> for ::either::Either<L, R> {
    fn from(value: Either<L, R>) -> Self {
        match value {
            Either::Left(left) => Self::Left(left),
            Either::Right(right) => Self::Right(right),
        }
    }
}

#[cfg(feature = "either")]
impl<L, R> From<::either::Either<L, R>> for Either<L, R> {
    fn from(value: ::either::Either<L, R>) -> Self {
        match value {
            ::either::Either::Left(left) => Self::Left(left),
            ::either::Either::Right(right) => Self::Right(right),
        }
    }
}
