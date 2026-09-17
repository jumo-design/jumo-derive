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
#[jumo(
    kind = "failure",
    domain = "Business",
    identity = "payment.timeout",
    tag = "payment"
)]
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
    domain = "Business",
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

// ---------------------------------------------------------------------------
// stable identity (`id`) and `module`
// ---------------------------------------------------------------------------

#[allow(dead_code)]
#[derive(Debug, Jumo)]
#[jumo(
    id = "Business.Order.Root",
    kind = "struct",
    domain = "Business",
    module = "Business.Order"
)]
struct IdentifiedOrder {
    #[jumo(unique)]
    id: String,
    total: u64,
}

#[allow(dead_code)]
#[derive(Debug, Jumo)]
#[jumo(kind = "struct", domain = "Business")]
struct UnidentifiedOrder {
    id: String,
}

#[allow(dead_code)]
#[derive(Debug, Jumo)]
#[jumo(kind = "state", domain = "Business", module = "Business.Order")]
enum ModuleOnlyState {
    Created,
}

#[test]
fn test_id_and_module_are_reported() {
    assert_eq!(IdentifiedOrder::jumo_id(), Some("Business.Order.Root"));
    assert_eq!(IdentifiedOrder::jumo_module(), Some("Business.Order"));
    assert_eq!(IdentifiedOrder::jumo_unique_fields(), &["id"]);
}

#[test]
fn test_id_defaults_to_none_without_module() {
    assert_eq!(UnidentifiedOrder::jumo_id(), None);
    assert_eq!(UnidentifiedOrder::jumo_module(), None);
}

#[test]
fn test_module_without_id() {
    assert_eq!(ModuleOnlyState::jumo_id(), None);
    assert_eq!(ModuleOnlyState::jumo_module(), Some("Business.Order"));
    assert_eq!(ModuleOnlyState::jumo_kind(), "state");
}

// ---------------------------------------------------------------------------
// generics, lifetimes, where clauses
// ---------------------------------------------------------------------------

#[allow(dead_code)]
#[derive(Debug, Jumo)]
#[jumo(kind = "struct", domain = "Business", module = "Design.Generic")]
struct GenericWrapper<T> {
    item: T,
}

#[allow(dead_code)]
#[derive(Debug, Jumo)]
#[jumo(kind = "actor", domain = "Business", parent = "User")]
struct GenericHolder<'a, T>
where
    T: Clone,
{
    borrowed: &'a T,
}

#[test]
fn test_generic_type_parameter() {
    assert_eq!(GenericWrapper::<u8>::jumo_kind(), "struct");
    assert_eq!(GenericWrapper::<u8>::jumo_domain(), "Business");
    assert_eq!(GenericWrapper::<u8>::jumo_module(), Some("Design.Generic"));
}

#[test]
fn test_generic_lifetime_and_where_clause() {
    assert_eq!(GenericHolder::<'static, u8>::jumo_kind(), "actor");
    assert_eq!(GenericHolder::<'static, u8>::jumo_parent(), Some("User"));
}

// ---------------------------------------------------------------------------
// type shapes without named fields
// ---------------------------------------------------------------------------

#[allow(dead_code)]
#[derive(Debug, Jumo)]
#[jumo(kind = "struct", domain = "Business")]
struct TupleRecord(String, u32);

#[allow(dead_code)]
#[derive(Jumo)]
#[jumo(kind = "struct", domain = "Business")]
union RawBits {
    as_u32: u32,
    as_bytes: [u8; 4],
}

#[test]
fn test_tuple_struct_has_no_unique_fields() {
    assert_eq!(TupleRecord::jumo_kind(), "struct");
    assert_eq!(TupleRecord::jumo_domain(), "Business");
    assert_eq!(TupleRecord::jumo_unique_fields(), &[] as &[&str]);
}

#[test]
fn test_union_is_supported() {
    assert_eq!(RawBits::jumo_kind(), "struct");
    assert_eq!(RawBits::jumo_domain(), "Business");
    assert_eq!(RawBits::jumo_unique_fields(), &[] as &[&str]);
}

// ---------------------------------------------------------------------------
// multiple unique fields
// ---------------------------------------------------------------------------

#[allow(dead_code)]
#[derive(Debug, Jumo)]
#[jumo(kind = "struct", domain = "Business")]
struct CompositeKey {
    #[jumo(unique)]
    tenant: String,
    #[jumo(unique)]
    code: String,
    label: String,
}

#[test]
fn test_multiple_unique_fields_keep_declaration_order() {
    assert_eq!(CompositeKey::jumo_unique_fields(), &["tenant", "code"]);
}

// ---------------------------------------------------------------------------
// enums never carry field-level unique
// ---------------------------------------------------------------------------

#[test]
fn test_enum_unique_fields_is_empty() {
    assert_eq!(OrderStatus::jumo_unique_fields(), &[] as &[&str]);
    assert_eq!(PaymentError::jumo_unique_fields(), &[] as &[&str]);
    assert_eq!(DbError::jumo_unique_fields(), &[] as &[&str]);
}

#[test]
fn test_enum_optional_metadata_defaults_to_none() {
    assert_eq!(OrderStatus::jumo_id(), None);
    assert_eq!(OrderStatus::jumo_module(), None);
    assert_eq!(OrderStatus::jumo_role(), None);
    assert_eq!(OrderStatus::jumo_identity(), None);
    assert_eq!(OrderStatus::jumo_tag(), None);
    assert_eq!(OrderStatus::jumo_storage_kind(), None);
    assert_eq!(OrderStatus::jumo_durability(), None);
    assert_eq!(OrderStatus::jumo_parent(), None);
    assert_eq!(OrderStatus::jumo_description(), None);
}

// ---------------------------------------------------------------------------
// attribute-list edge cases
// ---------------------------------------------------------------------------

#[allow(dead_code)]
#[derive(Debug, Jumo)]
#[jumo(kind = "struct", kind = "message", domain = "Business")]
struct DuplicateKey;

#[test]
fn test_duplicate_key_last_one_wins() {
    assert_eq!(DuplicateKey::jumo_kind(), "message");
    assert_eq!(DuplicateKey::jumo_domain(), "Business");
}

#[allow(dead_code)]
#[derive(Debug, Jumo)]
#[jumo(kind = "struct")]
#[jumo(domain = "Business", role = "query")]
struct SplitAttributes;

#[test]
fn test_multiple_attributes_accumulate() {
    assert_eq!(SplitAttributes::jumo_kind(), "struct");
    assert_eq!(SplitAttributes::jumo_domain(), "Business");
    assert_eq!(SplitAttributes::jumo_role(), Some("query"));
}

#[allow(dead_code)]
#[derive(Debug, Jumo)]
#[jumo(kind = "struct", domain = "Business")]
struct ToleratedFieldFlag {
    /// jumo-core annotates ignored fields as `#[jumo(skip)]`; the derive macro
    /// must tolerate field flags it does not interpret instead of failing.
    #[jumo(skip)]
    internal: String,
    #[jumo(unique)]
    id: String,
}

#[test]
fn test_uninterpreted_field_flag_is_tolerated() {
    assert_eq!(ToleratedFieldFlag::jumo_kind(), "struct");
    assert_eq!(ToleratedFieldFlag::jumo_unique_fields(), &["id"]);
}
