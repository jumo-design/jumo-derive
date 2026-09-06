# Changelog

All notable changes to this project will be documented in this file.

## [0.1.3] — Unreleased

### Changed

- **Renamed from moju to jumo across the project**: workspace member dir `moju-derive-macros` → `jumo-derive-macros`; package names `jumo-derive` / `jumo-derive-macros`; derive macro `MoJu` → `Jumo` with `#[jumo(...)]` attribute; trait `MoJuItem` → `JumoItem` with `jumo_*` accessors. Breaking for consumers of `moju-derive`.
- **Fix generics and lifetimes in derive macro**: `split_for_impl()` is now called after all local bindings, ensuring correct generic and lifetime handling in the generated `impl` block.

### Removed

- **Removed `module` metadata field**: The `module` attribute was removed from `TypeAttr`, attribute parsing, the `JumoItem` trait, and both generated `impl` blocks. Module designation remains the responsibility of Jumo model files.

## [0.1.1] — 2025-06-01 (approximate)

### Changed

- Bumped version to 0.1.1.

### Removed

- Removed unused CI workflow files (`pages.yml`, `release.yml`).

## [0.1.0] — 2025-06-01 (approximate)

### Added

- Initial release of `jumo-derive`, a proc-macro crate for annotating Rust types with reviewed Jumo metadata.
- `#[derive(Jumo)]` proc-macro with `#[jumo(...)]` attribute support.
- Supported metadata kinds: `kind`, `domain`, `role`, `identity`, `tag`, `storage_kind`, `durability`, `parent`, `description`.
- Field-level `#[jumo(unique)]` annotation support.
- `JumoItem` trait with default method implementations.
- CI and release workflows.
- Test suite covering struct and enum derive patterns.

[0.1.3]: https://github.com/dayu-sec/jumo-derive/compare/v0.1.2...HEAD
[0.1.2]: https://github.com/dayu-sec/jumo-derive/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/dayu-sec/jumo-derive/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/dayu-sec/jumo-derive/releases/tag/v0.1.0
