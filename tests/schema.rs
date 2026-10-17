#![cfg(feature = "schemars")]

use rust_fest::{Either, NonEmptyString, NonEmptyVec, Tagged};
use schemars::{JsonSchema, schema_for};
use serde_json::{Value, json};

enum User {}

#[test]
fn non_empty_schemas_preserve_inner_types_and_require_presence() {
    let schema = serde_json::to_value(schema_for!(NonEmptyVec<NonEmptyString>)).unwrap();
    assert_eq!(schema["type"], "array");
    assert_eq!(schema["minItems"], 1);
    assert_eq!(schema["items"]["$ref"], "#/$defs/NonEmptyString");
    assert_eq!(schema["$defs"]["NonEmptyString"]["type"], "string");
    assert_eq!(schema["$defs"]["NonEmptyString"]["minLength"], 1);
    assert_ne!(
        NonEmptyVec::<String>::schema_id(),
        NonEmptyVec::<u64>::schema_id()
    );
    assert_ne!(
        NonEmptyVec::<String>::schema_id(),
        Vec::<String>::schema_id()
    );
}

#[test]
fn tagged_schemas_are_transparent_without_marker_bounds() {
    assert_eq!(schema_for!(Tagged<u64, User>), schema_for!(u64));
    assert_eq!(
        schema_for!(Tagged<NonEmptyString, User>),
        schema_for!(NonEmptyString)
    );
    assert_eq!(
        schema_for!(Tagged<Option<String>, User>),
        schema_for!(Option<String>)
    );
}

#[test]
fn schema_derives_on_requests_and_keeps_tagged_option_optional() {
    #[derive(JsonSchema)]
    #[allow(dead_code)]
    struct Request {
        steps: NonEmptyVec<NonEmptyString>,
        owner: Tagged<Option<u64>, User>,
    }
    let schema = serde_json::to_value(schema_for!(Request)).unwrap();
    assert_eq!(schema["required"], json!(["steps"]));
    assert_eq!(
        schema["properties"]["steps"]["$ref"],
        "#/$defs/NonEmptyVec_of_NonEmptyString"
    );
}

#[cfg(feature = "serde")]
#[test]
fn tagged_optional_fields_match_serde_deserialization() {
    #[derive(JsonSchema, serde::Deserialize)]
    struct Request {
        owner: Tagged<Option<u64>, User>,
    }
    let value: Request = serde_json::from_str("{}").unwrap();
    assert_eq!(value.owner.into_inner(), None);
    let schema = serde_json::to_value(schema_for!(Request)).unwrap();
    assert!(schema.get("required").is_none());
}

#[test]
fn recursive_tagged_values_use_resolvable_references() {
    #[derive(JsonSchema)]
    #[allow(dead_code)]
    struct Node {
        children: Option<NonEmptyVec<Tagged<Box<Node>, User>>>,
    }
    let schema = serde_json::to_value(schema_for!(Node)).unwrap();
    fn check_refs(value: &Value, root: &Value) {
        match value {
            Value::Object(fields) => {
                if let Some(Value::String(reference)) = fields.get("$ref") {
                    assert!(
                        root.pointer(reference.strip_prefix('#').unwrap()).is_some(),
                        "{reference}"
                    );
                }
                for child in fields.values() {
                    check_refs(child, root);
                }
            }
            Value::Array(values) => {
                for child in values {
                    check_refs(child, root);
                }
            }
            _ => {}
        }
    }
    check_refs(&schema, &schema);
}

#[test]
fn either_schema_uses_the_existing_external_variant_names() {
    let schema = serde_json::to_value(schema_for!(Either<u64, NonEmptyString>)).unwrap();
    let variants = schema["oneOf"].as_array().unwrap();
    assert_eq!(variants.len(), 2);
    assert_eq!(variants[0]["required"], json!(["Left"]));
    assert_eq!(variants[1]["required"], json!(["Right"]));
    assert_eq!(variants[0]["additionalProperties"], false);
    assert_eq!(variants[1]["additionalProperties"], false);
}
