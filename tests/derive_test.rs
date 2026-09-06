use jumo_derive::{Jumo, JumoItem};

#[allow(dead_code)]
#[derive(Debug, Jumo)]
#[jumo(kind = "struct", domain = "Business")]
struct Order {
    #[jumo(unique)]
    id: String,
    user: String,
    status: String,
}

#[allow(dead_code)]
#[derive(Debug, Jumo)]
#[jumo(kind = "message", role = "command", domain = "Business")]
struct PlaceOrder {
    items: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Jumo)]
#[jumo(kind = "state", domain = "Business")]
enum OrderStatus {
    Created,
    Paid,
    Shipped,
    Cancelled,
}

#[allow(dead_code)]
#[derive(Debug, Jumo)]
#[jumo(kind = "failure", identity = "payment.timeout", tag = "payment")]
enum PaymentError {
    ProviderUnavailable,
    Timeout,
}

#[test]
fn test_struct_kind_and_domain() {
    assert_eq!(Order::jumo_kind(), "struct");
    assert_eq!(Order::jumo_domain(), "Business");
    assert_eq!(Order::jumo_role(), None);
    assert_eq!(Order::jumo_unique_fields(), &["id"]);
}

#[test]
fn test_message_role() {
    assert_eq!(PlaceOrder::jumo_kind(), "message");
    assert_eq!(PlaceOrder::jumo_role(), Some("command"));
    assert_eq!(PlaceOrder::jumo_unique_fields(), &[] as &[&str]);
}

#[test]
fn test_state_enum() {
    assert_eq!(OrderStatus::jumo_kind(), "state");
    assert_eq!(OrderStatus::jumo_domain(), "Business");
}

#[test]
fn test_failure_identity() {
    assert_eq!(PaymentError::jumo_kind(), "failure");
    assert_eq!(PaymentError::jumo_identity(), Some("payment.timeout"));
    assert_eq!(PaymentError::jumo_tag(), Some("payment"));
}

#[allow(dead_code)]
#[derive(Debug, Jumo)]
#[jumo(
    kind = "storage",
    domain = "Business",
    storage_kind = "table",
    durability = "persistent"
)]
struct OrderStore;

#[allow(dead_code)]
#[derive(Debug, Jumo)]
#[jumo(kind = "actor", domain = "Business", parent = "User")]
struct CustomerActor;

#[allow(dead_code)]
#[derive(Debug, Jumo)]
#[jumo(
    kind = "failure",
    identity = "db.timeout",
    tag = "db",
    description = "database operation timed out"
)]
enum DbError {
    Timeout,
}

#[test]
fn test_storage_kind_and_durability() {
    assert_eq!(OrderStore::jumo_kind(), "storage");
    assert_eq!(OrderStore::jumo_domain(), "Business");
    assert_eq!(OrderStore::jumo_storage_kind(), Some("table"));
    assert_eq!(OrderStore::jumo_durability(), Some("persistent"));
    assert_eq!(OrderStore::jumo_role(), None);
    assert_eq!(OrderStore::jumo_identity(), None);
}

#[test]
fn test_actor_parent() {
    assert_eq!(CustomerActor::jumo_kind(), "actor");
    assert_eq!(CustomerActor::jumo_domain(), "Business");
    assert_eq!(CustomerActor::jumo_parent(), Some("User"));
}

#[test]
fn test_failure_description() {
    assert_eq!(DbError::jumo_kind(), "failure");
    assert_eq!(DbError::jumo_identity(), Some("db.timeout"));
    assert_eq!(DbError::jumo_tag(), Some("db"));
    assert_eq!(
        DbError::jumo_description(),
        Some("database operation timed out")
    );
}
