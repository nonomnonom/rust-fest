use std::{
    collections::{BTreeSet, HashMap, HashSet},
    mem::{align_of, size_of},
    panic::{AssertUnwindSafe, catch_unwind},
    rc::Rc,
};

use rust_fest::{Either, EmptyError, NonEmptyString, NonEmptyVec, Tagged};

// Deliberately has no trait implementations: bounds must only apply to the value.
enum User {}
type UserId = Tagged<u64, User>;

#[test]
fn tagged_ids_work_as_keys_without_marker_trait_bounds() {
    let id = UserId::new(42);
    let copied = id;
    assert_eq!(id, copied);
    assert!(HashSet::from([id]).contains(&copied));
    let ordered = BTreeSet::from([UserId::new(7), id]);
    assert_eq!(ordered.first().unwrap().get(), &7);
    assert_eq!(format!("{id}"), "42");
    assert!(format!("{id:?}").contains("42"));
}

#[test]
fn tagged_values_clone_without_copy_or_clone_tags() {
    let original = Tagged::<String, User>::new(String::from("ferris"));
    let cloned = original.clone();
    assert_eq!(original.get(), cloned.get());
    assert_eq!(cloned.into_inner(), "ferris");
}

#[test]
fn tags_add_no_storage_or_alignment() {
    assert_eq!(size_of::<Tagged<u64, User>>(), size_of::<u64>());
    assert_eq!(align_of::<Tagged<u64, User>>(), align_of::<u64>());
    assert_eq!(size_of::<Tagged<String, User>>(), size_of::<String>());
    assert_eq!(size_of::<Tagged<(), User>>(), 0);
}

#[test]
fn thread_safety_does_not_require_the_marker_to_be_send_or_sync() {
    fn require_send_sync<T: Send + Sync>() {}
    require_send_sync::<Tagged<u64, Rc<()>>>();
}

#[test]
fn empty_inputs_are_rejected_by_every_checked_constructor() {
    assert_eq!(NonEmptyVec::<u8>::try_from(vec![]), Err(EmptyError));
    assert_eq!(NonEmptyString::try_from(""), Err(EmptyError));
    assert_eq!(NonEmptyString::try_from(String::new()), Err(EmptyError));
    assert_eq!("".parse::<NonEmptyString>(), Err(EmptyError));
}

#[test]
fn vector_mutations_preserve_a_final_element() {
    let mut values = NonEmptyVec::new(3);
    values.extend([1, 2]);
    values.push(4);
    values.as_mut_slice().sort();
    for value in &mut values {
        *value *= 2;
    }
    assert_eq!(values.first(), &2);
    assert_eq!(values.last(), &8);
    assert_eq!((&values).into_iter().sum::<i32>(), 20);
    assert_eq!(values.pop(), Some(8));
    assert_eq!(values.pop(), Some(6));
    assert_eq!(values.pop(), Some(4));
    assert_eq!(values.pop(), None);
    assert!(!values.is_empty());
    assert_eq!(values.as_slice(), &[2]);
    assert_eq!(values.into_iter().collect::<Vec<_>>(), [2]);
}

#[test]
fn an_iterator_panic_cannot_empty_an_existing_vector() {
    let mut values = NonEmptyVec::new(7);
    let result = catch_unwind(AssertUnwindSafe(|| {
        values.extend((0..3).inspect(|&index| {
            assert!(index < 1, "iterator failed");
        }));
    }));
    assert!(result.is_err());
    assert_eq!(values.first(), &7);
    assert!(!values.is_empty());
}

#[test]
fn wrapping_and_unwrapping_owned_values_preserves_their_allocations() {
    let vector = vec![String::from("ferris")];
    let original_pointer = vector.as_ptr();
    let wrapped = NonEmptyVec::try_from(vector).unwrap();
    assert_eq!(wrapped.as_ptr(), original_pointer);
    let unwrapped: Vec<_> = wrapped.into();
    assert_eq!(unwrapped.as_ptr(), original_pointer);

    let text = String::from("ferris");
    let original_pointer = text.as_ptr();
    let wrapped = NonEmptyString::try_from(text).unwrap();
    assert_eq!(wrapped.as_ptr(), original_pointer);
    let unwrapped: String = wrapped.into();
    assert_eq!(unwrapped.as_ptr(), original_pointer);
}

#[test]
fn non_empty_text_accepts_unicode_and_whitespace_without_normalizing() {
    for text in [" ", "\n", "🦀", "e\u{301}", "\0"] {
        let mut wrapped: NonEmptyString = text.parse().unwrap();
        assert_eq!(wrapped.as_str(), text);
        wrapped.push_str("");
        wrapped.push_str("!");
        assert_eq!(wrapped.to_string(), format!("{text}!"));
    }
}

#[test]
fn non_empty_keys_support_borrowed_lookups() {
    let text_keys = HashMap::from([(NonEmptyString::try_from("rust").unwrap(), 1)]);
    assert_eq!(text_keys.get("rust"), Some(&1));
    let vector_keys = HashMap::from([(NonEmptyVec::try_from(vec![1, 2]).unwrap(), 2)]);
    assert_eq!(vector_keys.get(&[1, 2][..]), Some(&2));
}

#[test]
fn either_mapping_only_evaluates_the_matching_branch() {
    let left = Either::<_, String>::Left(7)
        .map_right(|_| -> usize { panic!("wrong branch") })
        .map_left(|number| number.to_string());
    assert_eq!(left, Either::Left(String::from("7")));
    let right = Either::<String, _>::Right(9)
        .map_left(|_| -> usize { panic!("wrong branch") })
        .map_right(|number| number.to_string());
    assert_eq!(right, Either::Right(String::from("9")));
}

#[test]
fn either_borrowing_keeps_the_variant_and_owned_value() {
    for mut choice in [
        Either::Left(String::from("L")),
        Either::Right(String::from("R")),
    ] {
        match choice.as_mut() {
            Either::Left(text) | Either::Right(text) => text.push('!'),
        }
        assert!(choice.as_ref().into_inner().ends_with('!'));
        assert!(choice.into_inner().ends_with('!'));
    }
}

#[test]
fn empty_errors_work_with_standard_error_propagation() {
    fn parse_name() -> Result<NonEmptyString, Box<dyn std::error::Error>> {
        Ok(NonEmptyString::try_from("")?)
    }
    let error = parse_name().unwrap_err();
    assert!(error.downcast_ref::<EmptyError>().is_some());
    assert_eq!(error.to_string(), "expected a non-empty value");
}
