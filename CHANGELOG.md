# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added

- **`kind` is validated at compile time.** `#[jumo(kind = "...")]` used to accept
  any string, so a typo such as `kind = "sturct"` compiled and only surfaced when
  a tool read `jumo_kind()`. The accepted values are the `.mju` item kinds
  (`struct`, `state`, `enum`, `variant`, `event`, `message`, `failure`, `cap`,
  `actor`, `module`, `interface`, `storage`, `command`, `dataflow`, `lifecycle`,
  `layer`, `dependency_rule`, `decision`, `failure_policy`, `flow`, `verify`,
  `target`), and the error message lists all of them.

### Fixed

- `JumoItem::jumo_kind()` documented only 20 of the accepted kinds. `variant`
  and `enum` were missing although both are in use across the workspace.
  `enum` is a code-only spelling: the draft generator projects it back onto
  `state`, so it carries no information a `.mju` model could round-trip.

## [0.1.6] — 2026-09-17

### Changed

- Both crates now share a single version number: it is declared once under
  `[workspace.package]` and inherited by `jumo-derive` and
  `jumo-derive-macros`, together with `edition`, `license`, and `repository`.
  A release can no longer publish one crate and skip the other.
- `JumoItem` impls for enums are emitted through the same path as structs and
  unions, removing a duplicated copy of the impl block. No behaviour change.
- `syn` no longer enables the unused `full` and `extra-traits` features, which
  trims the dependency build for consumers.

### Fixed

- The README documented only 9 of the 11 supported type-level keys; `id` and
  `module` were missing. A new `Validation` section records the compile-time
  checks introduced in 0.1.4.

## [0.1.5] — 2026-09-17

### Changed

- The release workflow skips `jumo-derive-macros` when that version is already
  on crates.io, instead of failing the job. Previously it aborted there and
  `jumo-derive` itself was silently skipped, so a release could look published
  on GitHub while nothing reached crates.io.

### Fixed

- `repository` now points at the real upstream, `jumo-design/jumo-derive`. The
  previous value referenced a repository that does not exist.

## [0.1.4] — 2026-09-17

### Added

- **`id` attribute**: `#[jumo(id = "Business.Order.Root")]` implements the new
  `JumoItem::jumo_id()`. This gives tools a stable pointer from a Rust type to
  its model item, so the pairing no longer depends on guessing type names.
  Without `id`, `jumo_id()` returns `None` and tools fall back to name matching.

### Fixed

- **Type-level `#[jumo(...)]` mistakes are now compile errors** instead of
  silently emitting wrong metadata. The parser used to discard its own error
  (`let _ = list.parse_nested_meta(...)`), and because parsing stops at the
  first error, every key after the offending one was dropped as well:
  `#[jumo(kind = "struct", bogus = "x", domain = "Business")]` compiled and
  reported `jumo_domain() == ""`. Now reported: unknown keys, non-string values,
  a missing or empty `kind`/`domain`, and a `#[derive(Jumo)]` with no
  `#[jumo(...)]` attribute at all.
- Uninterpreted field-level flags (such as `#[jumo(skip)]`) remain tolerated:
  the derive macro must not fail a build over metadata it does not consume.
- CI now runs `cargo test`, `clippy`, and doc tests with `--workspace`, so the
  `jumo-derive-macros` unit tests are actually executed.

## [0.1.3] — 2026-09-06

### Changed

- **Renamed from moju to jumo**: workspace member `moju-derive-macros` →
  `jumo-derive-macros`; packages `jumo-derive` / `jumo-derive-macros`; derive
  macro `MoJu` → `Jumo` with `#[jumo(...)]`; trait `MoJuItem` → `JumoItem` with
  `jumo_*` accessors. Breaking for consumers of `moju-derive`.
- CI cleanup: dropped jobs referencing scripts that no longer exist.

## [0.1.2] — 2026-06-29

### Added

- Apache-2.0 license, and crates.io package metadata.

### Fixed

- **Generics and lifetimes in the derive macro**: `split_for_impl()` is applied
  after all local bindings, so the generated `impl` block is correct.
- Publish `jumo-derive-macros` before `jumo-derive`.

## [0.1.1] — 2026-05-29

### Changed

- Bumped version to 0.1.1.

### Removed

- Removed unused CI workflow files (`pages.yml`, `release.yml`).

## [0.1.0] — 2026-05-29

### Added

- Initial release of `jumo-derive`, a proc-macro crate for annotating Rust types
  with reviewed Jumo metadata.
- `#[derive(Jumo)]` proc-macro with `#[jumo(...)]` attribute support.
- Type metadata: `kind`, `domain`, `module`, `role`, `identity`, `tag`,
  `storage_kind`, `durability`, `parent`, `description`.
- Field-level `#[jumo(unique)]` annotation support.
- `JumoItem` trait with default implementations, except for `jumo_kind()` and
  `jumo_domain()`.
- CI and release workflows, and a test suite covering struct and enum patterns.

[Unreleased]: https://github.com/jumo-design/jumo-derive/compare/v0.1.6...HEAD
[0.1.6]: https://github.com/jumo-design/jumo-derive/compare/v0.1.5...v0.1.6
[0.1.5]: https://github.com/jumo-design/jumo-derive/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/jumo-design/jumo-derive/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/jumo-design/jumo-derive/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/jumo-design/jumo-derive/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/jumo-design/jumo-derive/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/jumo-design/jumo-derive/releases/tag/v0.1.0
