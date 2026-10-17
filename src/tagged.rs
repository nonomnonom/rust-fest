use core::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

/// A value distinguished by a compile-time tag, with the same layout as `T`.
///
/// Tags prevent accidentally mixing values that share a representation, such as
/// user IDs and order IDs. The tag is invariant and is never stored at runtime.
/// Traits and thread safety depend on `T`, not on `Tag`.
///
/// This type labels a value; it does not validate it. Anyone can call [`Self::new`].
/// Use a private newtype with a checked constructor for domain validation.
///
/// ```
/// use rust_fest::Tagged;
///
/// enum User {}
/// type UserId = Tagged<u64, User>;
///
/// let id = UserId::new(42);
/// assert_eq!(*id.get(), 42);
/// assert_eq!(id.into_inner(), 42);
/// ```
///
/// Different tags cannot be assigned to one another:
///
/// ```compile_fail,E0308
/// use rust_fest::Tagged;
/// enum User {}
/// enum Order {}
/// let user = Tagged::<u64, User>::new(42);
/// let order: Tagged<u64, Order> = user;
/// ```
///
/// A type alias is only a new name, so use distinct marker types for distinct tags.
///
/// Enabling `serde` serializes only the wrapped value. Tags are not recorded in
/// the data, so deserialization does not verify a tag's identity or semantics.
///
/// Thread safety still follows the wrapped value:
///
/// ```compile_fail,E0277
/// use rust_fest::Tagged;
/// use std::rc::Rc;
/// fn require_send<T: Send>() {}
/// require_send::<Tagged<Rc<u64>, ()>>();
/// ```
///
/// Even lifetime-based tags cannot be weakened:
///
/// ```compile_fail
/// use rust_fest::Tagged;
/// fn shorten<'a>(id: Tagged<u64, &'static str>, _: &'a str) -> Tagged<u64, &'a str> {
///     id
/// }
/// ```
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(
        transparent,
        bound(
            serialize = "T: serde::Serialize",
            deserialize = "T: serde::Deserialize<'de>"
        )
    )
)]
pub struct Tagged<T, Tag> {
    value: T,
    // Function input + output makes Tag invariant without owning a Tag value.
    #[cfg_attr(feature = "serde", serde(skip))]
    tag: PhantomData<fn(Tag) -> Tag>,
}

impl<T, Tag> Tagged<T, Tag> {
    /// Labels a value without validation or additional allocation.
    pub const fn new(value: T) -> Self {
        Self {
            value,
            tag: PhantomData,
        }
    }

    /// Borrows the underlying value without removing its tag.
    pub const fn get(&self) -> &T {
        &self.value
    }

    /// Consumes the wrapper and removes the tag.
    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T: Clone, Tag> Clone for Tagged<T, Tag> {
    fn clone(&self) -> Self {
        Self::new(self.value.clone())
    }
}

impl<T: Copy, Tag> Copy for Tagged<T, Tag> {}

impl<T: fmt::Debug, Tag> fmt::Debug for Tagged<T, Tag> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_tuple("Tagged").field(&self.value).finish()
    }
}

impl<T: fmt::Display, Tag> fmt::Display for Tagged<T, Tag> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(formatter)
    }
}

impl<T: PartialEq, Tag> PartialEq for Tagged<T, Tag> {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl<T: Eq, Tag> Eq for Tagged<T, Tag> {}

impl<T: PartialOrd, Tag> PartialOrd for Tagged<T, Tag> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<T: Ord, Tag> Ord for Tagged<T, Tag> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl<T: Hash, Tag> Hash for Tagged<T, Tag> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}
