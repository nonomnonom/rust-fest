use alloc::{borrow::Cow, format, string::String, vec::Vec};
use schemars::{JsonSchema, Schema, SchemaGenerator};

use crate::{NonEmptyString, NonEmptyVec, Tagged};

impl<T: JsonSchema, Tag> JsonSchema for Tagged<T, Tag> {
    fn inline_schema() -> bool {
        T::inline_schema()
    }

    fn schema_name() -> Cow<'static, str> {
        T::schema_name()
    }

    fn schema_id() -> Cow<'static, str> {
        T::schema_id()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        T::json_schema(generator)
    }

    // Schemars uses these hooks for optional fields and flattened schemas.
    // Forward them just as its transparent derive does for Option wrappers.
    fn _schemars_private_is_option() -> bool {
        T::_schemars_private_is_option()
    }

    fn _schemars_private_non_optional_json_schema(generator: &mut SchemaGenerator) -> Schema {
        T::_schemars_private_non_optional_json_schema(generator)
    }
}

impl<T: JsonSchema> JsonSchema for NonEmptyVec<T> {
    fn schema_name() -> Cow<'static, str> {
        format!("NonEmptyVec_of_{}", T::schema_name()).into()
    }

    fn schema_id() -> Cow<'static, str> {
        format!("rust_fest::NonEmptyVec<{}>", T::schema_id()).into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let mut schema = Vec::<T>::json_schema(generator);
        schema.insert("minItems".into(), 1.into());
        schema
    }
}

impl JsonSchema for NonEmptyString {
    fn schema_name() -> Cow<'static, str> {
        "NonEmptyString".into()
    }

    fn schema_id() -> Cow<'static, str> {
        "rust_fest::NonEmptyString".into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let mut schema = String::json_schema(generator);
        schema.insert("minLength".into(), 1.into());
        schema
    }
}
