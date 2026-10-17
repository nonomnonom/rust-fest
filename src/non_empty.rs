use alloc::{borrow::ToOwned, string::String, vec, vec::Vec};
use core::{borrow::Borrow, error::Error, fmt, ops::Deref, str::FromStr};

/// An empty collection or string was supplied where a non-empty value is required.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmptyError;

impl fmt::Display for EmptyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("expected a non-empty value")
    }
}

impl Error for EmptyError {}

/// An owned vector guaranteed to contain at least one element.
///
/// Construction checks the invariant once. Mutation can append elements, edit
/// existing elements, or pop while more than one remains. No mutable `Vec<T>`
/// access is exposed, so safe callers cannot empty the collection in place.
///
/// ```
/// use rust_fest::NonEmptyVec;
///
/// let mut recipients = NonEmptyVec::new("ferris");
/// recipients.push("corro");
/// assert_eq!(recipients.first(), &"ferris");
/// assert_eq!(recipients.pop(), Some("corro"));
/// assert_eq!(recipients.pop(), None);
/// assert_eq!(recipients.len(), 1);
/// ```
///
/// ```compile_fail,E0599
/// use rust_fest::NonEmptyVec;
/// let mut values = NonEmptyVec::new(1);
/// values.clear();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct NonEmptyVec<T>(Vec<T>);

impl<T> NonEmptyVec<T> {
    /// Creates a vector containing one element.
    pub fn new(first: T) -> Self {
        Self(vec![first])
    }

    /// Borrows all elements as a slice.
    pub fn as_slice(&self) -> &[T] {
        &self.0
    }

    /// Borrows elements mutably without allowing the vector's length to change.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.0
    }

    /// Borrows the first element, which always exists.
    pub fn first(&self) -> &T {
        &self.0[0]
    }

    /// Borrows the last element, which always exists.
    pub fn last(&self) -> &T {
        &self.0[self.0.len() - 1]
    }

    /// Appends an element.
    pub fn push(&mut self, value: T) {
        self.0.push(value);
    }

    /// Removes the last element if at least two elements remain before removal.
    ///
    /// Returns `None` without changing the vector when only one element remains.
    pub fn pop(&mut self) -> Option<T> {
        if self.0.len() > 1 { self.0.pop() } else { None }
    }

    /// Consumes the wrapper and returns a vector with unrestricted mutation.
    pub fn into_vec(self) -> Vec<T> {
        self.0
    }
}

impl<T> TryFrom<Vec<T>> for NonEmptyVec<T> {
    type Error = EmptyError;

    fn try_from(values: Vec<T>) -> Result<Self, Self::Error> {
        if values.is_empty() {
            Err(EmptyError)
        } else {
            Ok(Self(values))
        }
    }
}

impl<T> From<NonEmptyVec<T>> for Vec<T> {
    fn from(values: NonEmptyVec<T>) -> Self {
        values.into_vec()
    }
}

impl<T> Deref for NonEmptyVec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T> AsRef<[T]> for NonEmptyVec<T> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> AsMut<[T]> for NonEmptyVec<T> {
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T> Borrow<[T]> for NonEmptyVec<T> {
    fn borrow(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> Extend<T> for NonEmptyVec<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.0.extend(iter);
    }
}

impl<T> IntoIterator for NonEmptyVec<T> {
    type Item = T;
    type IntoIter = alloc::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a NonEmptyVec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut NonEmptyVec<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

/// An owned UTF-8 string guaranteed to contain at least one byte.
///
/// Whitespace is accepted. This checks emptiness, not whether text is trimmed,
/// visible, or valid for a particular domain. Read-only `str` methods are
/// available through [`Deref`]. No mutable `String` access is exposed.
///
/// ```
/// use rust_fest::NonEmptyString;
///
/// let name: NonEmptyString = "Ferris".parse()?;
/// assert_eq!(name.as_str(), "Ferris");
/// assert!(NonEmptyString::try_from("").is_err());
/// # Ok::<(), rust_fest::EmptyError>(())
/// ```
///
/// ```compile_fail,E0599
/// use rust_fest::NonEmptyString;
/// let mut name = NonEmptyString::try_from("Ferris").unwrap();
/// name.clear();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct NonEmptyString(String);

impl NonEmptyString {
    /// Borrows the text.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Appends text, preserving the non-empty invariant.
    pub fn push_str(&mut self, text: &str) {
        self.0.push_str(text);
    }

    /// Consumes the wrapper and returns a string with unrestricted mutation.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl TryFrom<String> for NonEmptyString {
    type Error = EmptyError;

    fn try_from(text: String) -> Result<Self, Self::Error> {
        if text.is_empty() {
            Err(EmptyError)
        } else {
            Ok(Self(text))
        }
    }
}

impl TryFrom<&str> for NonEmptyString {
    type Error = EmptyError;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        if text.is_empty() {
            Err(EmptyError)
        } else {
            Ok(Self(text.to_owned()))
        }
    }
}

impl FromStr for NonEmptyString {
    type Err = EmptyError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::try_from(text)
    }
}

impl From<NonEmptyString> for String {
    fn from(text: NonEmptyString) -> Self {
        text.into_string()
    }
}

impl Deref for NonEmptyString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl AsRef<str> for NonEmptyString {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Borrow<str> for NonEmptyString {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for NonEmptyString {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}
