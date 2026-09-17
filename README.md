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

Type-level keys, all with string-literal values:

- `kind` (required) — item kind, e.g. `"struct"`, `"state"`, `"event"`,
  `"message"`, `"failure"`, `"cap"`, `"actor"`, `"storage"`
- `domain` (required) — domain name, e.g. `"Business"`
- `id` — stable model item name, e.g. `"Business.Order.Root"`; exposed as
  `JumoItem::jumo_id()`, so tools can pair code with model items without
  guessing type names
- `module` — module name, e.g. `"Design.JumoCore"`
- message `role` — `"command"`, `"query"`, `"response"`
- failure `identity`, `tag`, `description`
- storage `storage_kind`, `durability`
- actor `parent`

Field-level:

- `#[jumo(unique)]` — marks a field as part of the item's unique key; exposed as
  `JumoItem::jumo_unique_fields()`

The mapping from each key onto its `JumoItem` accessor is documented on the trait.

Example:

```rust
use jumo_derive::Jumo;

#[derive(Jumo)]
#[jumo(
    id = "Business.Order.SubmitOrder",
    kind = "message",
    role = "command",
    domain = "Business",
    module = "Business.Order"
)]
pub struct SubmitOrder {
    #[jumo(unique)]
    pub id: String,
}
```

## Validation

Type-level `#[jumo(...)]` mistakes fail the build instead of silently producing
empty or partial metadata:

- an unknown key is an error
- values must be string literals
- `kind` and `domain` are required and must not be empty; a `#[derive(Jumo)]`
  with no `#[jumo(...)]` attribute at all is an error
- keys may be spread over several `#[jumo(...)]` attributes, and the last
  occurrence of a key wins

Field-level flags the macro does not interpret are deliberately tolerated:
`#[jumo(skip)]` marks a field the extractor ignores, and annotation must not
break a build over metadata the derive does not itself consume.

## Boundaries

Do not use type-level attributes to encode broad design structures such as flow steps, dataflow graph edges, interface routes, storage adapter providers, config sources, or generation profile choices. Those remain in Jumo model files.
