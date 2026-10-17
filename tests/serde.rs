#![cfg(feature = "serde")]

use rust_fest::{Either, NonEmptyString, NonEmptyVec, Tagged};

#[test]
fn non_empty_values_roundtrip_with_plain_json_shapes() {
    let list = NonEmptyVec::try_from(vec![1, 2]).unwrap();
    let json = serde_json::to_string(&list).unwrap();
    assert_eq!(json, "[1,2]");
    assert_eq!(
        serde_json::from_str::<NonEmptyVec<i32>>(&json).unwrap(),
        list
    );

    let text = NonEmptyString::try_from("🦀").unwrap();
    let json = serde_json::to_string(&text).unwrap();
    assert_eq!(json, "\"🦀\"");
    assert_eq!(serde_json::from_str::<NonEmptyString>(&json).unwrap(), text);
}

#[test]
fn deserialization_cannot_bypass_non_empty_validation() {
    assert!(serde_json::from_str::<NonEmptyVec<u8>>("[]").is_err());
    assert!(serde_json::from_str::<NonEmptyString>("\"\"").is_err());
    assert!(serde_json::from_str::<NonEmptyVec<NonEmptyString>>("[\"\"]").is_err());
    assert!(serde_json::from_str::<NonEmptyString>("null").is_err());
    assert!(serde_json::from_str::<NonEmptyVec<u8>>("null").is_err());
}

#[test]
fn tags_do_not_need_serialization_traits() {
    enum User {}
    type Id = Tagged<u64, User>;
    let id = Id::new(42);
    assert_eq!(serde_json::to_string(&id).unwrap(), "42");
    assert_eq!(serde_json::from_str::<Id>("42").unwrap(), id);
}

#[test]
fn tagged_payload_validation_is_preserved() {
    enum Name {}
    type NameValue = Tagged<NonEmptyString, Name>;
    assert!(serde_json::from_str::<NameValue>("\"\"").is_err());
    assert_eq!(
        serde_json::from_str::<NameValue>("\"rust\"")
            .unwrap()
            .get()
            .as_str(),
        "rust"
    );
}

#[test]
fn either_roundtrips_using_explicit_variant_names() {
    for (value, json) in [
        (Either::<u64, String>::Left(42), "{\"Left\":42}"),
        (Either::Right(String::from("rust")), "{\"Right\":\"rust\"}"),
    ] {
        assert_eq!(serde_json::to_string(&value).unwrap(), json);
        assert_eq!(
            serde_json::from_str::<Either<u64, String>>(json).unwrap(),
            value
        );
    }
    assert!(
        serde_json::from_str::<Either<u64, String>>("{\"Left\":42,\"Right\":\"rust\"}").is_err()
    );
}
