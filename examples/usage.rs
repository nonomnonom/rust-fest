use rust_fest::{Either, EmptyError, NonEmptyString, NonEmptyVec, Tagged};

enum User {}
type UserId = Tagged<u64, User>;

struct Notification {
    recipients: NonEmptyVec<UserId>,
    message: NonEmptyString,
}

fn main() -> Result<(), EmptyError> {
    let mut recipients = NonEmptyVec::new(UserId::new(42));
    recipients.push(UserId::new(7));

    let notification = Notification {
        recipients,
        message: "Welcome to Rust Fest!".parse()?,
    };

    println!("First recipient: {}", notification.recipients.first());
    for recipient in &notification.recipients {
        println!("To {recipient}: {}", notification.message);
    }

    let lookup: Either<UserId, &str> = Either::Right("ferris");
    let label = lookup
        .map_left(|id| format!("ID {id}"))
        .map_right(|name| format!("@{name}"))
        .into_inner();
    println!("Lookup: {label}");

    Ok(())
}
