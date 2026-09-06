# jumo-derive

Rust annotations for carrying reviewed Jumo metadata in source code.

`jumo-derive` is used after a Jumo model has been reviewed and promoted into `jumo/`. It lets stable model facts travel with code so future `jumo-extract` runs can treat those annotations as high-confidence evidence.

## Role

```text
jumo/ reviewed model
  -> sync stable metadata into code annotations
  -> jumo-extract reads annotations as high-confidence facts
  -> drift report compares code, annotations, and jumo/
```

The reviewed `jumo/` model remains the source of truth. `#[jumo(...)]` attributes are mirrors and anchors, not a replacement for `domain.mju`, `binding.mju`, or `profile.mju`.

## Supported Metadata

Current `#[derive(Jumo)]` support covers local type metadata:

- `kind`
- `domain`
- message `role`
- failure `identity`, `tag`, `description`
- storage `storage_kind`, `durability`
- actor `parent`
- field-level `#[jumo(unique)]`

Example:

```rust
use jumo_derive::Jumo;

#[derive(Jumo)]
#[jumo(kind = "message", role = "command", domain = "Business")]
pub struct SubmitOrder {
    #[jumo(unique)]
    pub id: String,
}
```

## Boundaries

Do not use type-level attributes to encode broad design structures such as flow steps, dataflow graph edges, interface routes, storage adapter providers, config sources, or generation profile choices. Those remain in Jumo model files.
