// Re-export the derive macro from the proc-macro sub-crate.
pub use jumo_derive_macros::Jumo;

/// The trait that `#[derive(Jumo)]` implements.
///
/// Generated code depends on this trait to carry .mju metadata at runtime.
/// Extraction tools can also use it for compiled-type reflection.
pub trait JumoItem {
    /// 稳定身份：模型条目的全名（如 `"Control.Gateway.Management.GatewayHealth"`）。
    ///
    /// 这是给工具用的**指针**：有了它，代码类型与模型条目的对应关系不再依赖
    /// 类型名猜测（原名匹配对字母大小写、前后缀差异都无能为力）。缺省 None 表示
    /// 该类型未声明 id，工具回落到名字匹配。
    fn jumo_id() -> Option<&'static str> {
        None
    }

    /// .mju item kind: `"struct"`, `"state"`, `"event"`, `"message"`,
    /// `"failure"`, `"cap"`, `"actor"`, `"module"`, `"interface"`,
    /// `"storage"`, `"command"`, `"dataflow"`, `"lifecycle"`, `"layer"`,
    /// `"dependency_rule"`, `"decision"`, `"failure_policy"`, `"flow"`,
    /// `"verify"`, `"target"`.
    fn jumo_kind() -> &'static str;

    /// Domain name, e.g. `"Business"`, `"Storage"`.
    fn jumo_domain() -> &'static str;

    /// Module name, e.g. `"Design.JumoCore"`.
    fn jumo_module() -> Option<&'static str> {
        None
    }

    /// Message role, if kind is `"message"`: `"command"`, `"query"`, `"response"`.
    fn jumo_role() -> Option<&'static str> {
        None
    }

    /// Failure identity path, if kind is `"failure"`, e.g. `"payment.timeout"`.
    fn jumo_identity() -> Option<&'static str> {
        None
    }

    /// Failure tag, if kind is `"failure"`, e.g. `"payment"`.
    fn jumo_tag() -> Option<&'static str> {
        None
    }

    /// Storage kind, if kind is `"storage"`:
    /// `"table"`, `"document"`, `"key_value"`, `"queue"`, `"object"`,
    /// `"search"`, `"graph"`, `"cache"`.
    fn jumo_storage_kind() -> Option<&'static str> {
        None
    }

    /// Storage durability, if kind is `"storage"`:
    /// `"transient"`, `"persistent"`, `"derived"`.
    fn jumo_durability() -> Option<&'static str> {
        None
    }

    /// Actor parent name, if kind is `"actor"`, e.g. `"User"`.
    fn jumo_parent() -> Option<&'static str> {
        None
    }

    /// Failure description text, if kind is `"failure"`.
    fn jumo_description() -> Option<&'static str> {
        None
    }

    /// Field names marked `#[jumo(unique)]`.
    fn jumo_unique_fields() -> &'static [&'static str] {
        &[]
    }
}
