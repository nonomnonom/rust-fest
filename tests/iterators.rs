use rust_fest::{Either, NonEmptyVec};

#[test]
fn map_preserves_order_cardinality_and_accepts_owned_non_copy_values() {
    let values = NonEmptyVec::try_from(vec![String::from("a"), String::from("bbb")]).unwrap();
    let mut seen = Vec::new();
    let mut lengths = values.map(|value| {
        seen.push(value.clone());
        value.len()
    });
    assert_eq!(seen, ["a", "bbb"]);
    assert_eq!(lengths.as_slice(), &[1, 3]);
    assert_eq!(lengths.pop(), Some(3));
    assert_eq!(lengths.pop(), None);
    assert_eq!(*NonEmptyVec::new(7).map(|_| ()).first(), ());
}

#[test]
fn either_iterators_support_both_branches_and_standard_adapters() {
    for iter in [Either::Left(1..5), Either::Right([1, 2, 3, 4].into_iter())] {
        assert_eq!(iter.clone().sum::<i32>(), 10);
        assert_eq!(iter.clone().rev().collect::<Vec<_>>(), [4, 3, 2, 1]);
        assert_eq!(iter.filter(|n| n % 2 == 0).collect::<Vec<_>>(), [2, 4]);
    }
}

#[test]
fn either_mixed_direction_iteration_tracks_length_and_exhaustion() {
    fn require_fused<T: core::iter::FusedIterator>(_: &T) {}
    for mut iter in [
        Either::Left(0..6),
        Either::Right([0, 1, 2, 3, 4, 5].into_iter()),
    ] {
        require_fused(&iter);
        assert_eq!(iter.len(), 6);
        assert_eq!(iter.size_hint(), (6, Some(6)));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next_back(), Some(5));
        assert_eq!(iter.nth(1), Some(2));
        assert_eq!(iter.nth_back(1), Some(3));
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }
}

#[test]
fn either_does_not_require_exact_size_or_fused_iterators() {
    let mut next = 0;
    let source = std::iter::from_fn(move || {
        next += 1;
        (next % 2 == 1).then_some(next)
    });
    let mut iter = Either::<_, std::iter::Empty<i32>>::Left(source);
    assert_eq!(iter.size_hint(), (0, None));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), Some(3));
}

#[cfg(feature = "either")]
#[test]
fn upstream_conversion_roundtrips_owned_values_and_both_variants() {
    for value in [
        Either::Left(String::from("left")),
        Either::Right(String::from("right")),
    ] {
        let upstream: either::Either<String, String> = value.clone().into();
        let roundtrip: Either<String, String> = upstream.into();
        assert_eq!(roundtrip, value);
    }
}

#[cfg(feature = "either")]
#[test]
fn conversion_connects_to_itertools_partition_map() {
    use itertools::Itertools;
    let (even, odd): (Vec<_>, Vec<_>) = (0..6).partition_map(|n| {
        let branch = if n % 2 == 0 {
            Either::Left(n)
        } else {
            Either::Right(n)
        };
        branch.into()
    });
    assert_eq!(even, [0, 2, 4]);
    assert_eq!(odd, [1, 3, 5]);
}
