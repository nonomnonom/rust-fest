//! Generate a schema for the same types used in an API's Serde payloads.
use rust_fest::{Either, NonEmptyString, NonEmptyVec, Tagged};
use schemars::JsonSchema;

enum Project {}

#[derive(JsonSchema)]
#[allow(dead_code)]
struct Job {
    project: Tagged<u64, Project>,
    steps: NonEmptyVec<NonEmptyString>,
    destination: Either<u64, NonEmptyString>,
}

fn main() -> Result<(), serde_json::Error> {
    let schema = schemars::schema_for!(Job);
    println!("{}", serde_json::to_string_pretty(&schema)?);
    Ok(())
}
