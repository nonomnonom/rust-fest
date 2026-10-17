use rust_fest::{NonEmptyString, NonEmptyVec, Tagged};
use serde::{Deserialize, Serialize};

enum Project {}
type ProjectId = Tagged<u64, Project>;

#[derive(Debug, Serialize, Deserialize)]
struct Job {
    project_id: ProjectId,
    steps: NonEmptyVec<NonEmptyString>,
}

fn main() -> Result<(), serde_json::Error> {
    let request = r#"{"project_id":42,"steps":["build","test"]}"#;
    let job: Job = serde_json::from_str(request)?;

    println!(
        "Project {} starts with: {}",
        job.project_id,
        job.steps.first()
    );
    println!("Serialized: {}", serde_json::to_string(&job)?);

    for invalid in [
        r#"{"project_id":42,"steps":[]}"#,
        r#"{"project_id":42,"steps":[""]}"#,
    ] {
        match serde_json::from_str::<Job>(invalid) {
            Err(error) => println!("Rejected invalid job: {error}"),
            Ok(_) => panic!("non-empty validation was bypassed"),
        }
    }

    Ok(())
}
