# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Please make sure to add your changes to the appropriate categories:

- `Added`: for new functionality
- `Changed`: for changes in existing functionality
- `Deprecated`: for soon-to-be removed functionality
- `Removed`: for removed functionality
- `Fixed`: for fixed bugs
- `Performance`: for performance-relevant changes
- `Security`: for security-relevant changes
- `Other`: for everything else

## [Unreleased]

### Added

- n/a

### Changed

- Bumped MSRV from "1.74.0" to "1.78.0".
- Made the enum-level `#[enumcapsulate(exclude(…))]` helper macros only have an effect on derives, when orchestrated through the `Encapsulate` derive macro.
- Made `Encapsulate` derive macro no longer derive `VariantDiscriminant`.

### Deprecated

- n/a

### Removed

- n/a

### Fixed

- n/A

### Performance

- n/a

### Security

- n/a

### Other

- n/a

## [0.4.0] - 2024-11-18

### Added

- Added dedicated `trait AsVariant<T>` for `AsVariantRef<T>`'s removed `fn as_variant(&self) -> Option<T>`.
- Added dedicated `AsVariant` derive macro for `trait AsVariant<T>`.
- Added enum-level `#[enumcapsulate(exclude)]`/`#[enumcapsulate(exclude(…))]` helper attribute.
- Added optional selection list to variant-level `#[enumcapsulate(exclude(…))]` helper attribute.
- Added optional selection list to variant-level `#[enumcapsulate(include(…))]` helper attribute.
- Added derive support for generic enums (by force-excluding variants that use generic const/type parameters).

### Changed

- Removed `enumcapsulate::macros` and its redundant `enumcapsulate::derive` module in favor of re-exporting its items directly from `crate::*`, thus eliminating the need for individually importing trait + derive macro pairs:
  - Before: `use enumcapsulate::{derive::FromVariant, FromVariant};`
  - After: `use enumcapsulate::FromVariant;`
- Made `Encapsulate` derive macro derive also derive `AsVariant<T>`.
- Promoted `field` value of `include(field = …)` to its own top-level variant attribute with name/index variants:
  - `#[enumcapsulate(field(index = …))]`
  - `#[enumcapsulate(field(name = "…"))]`

### Removed

- Removed `fn as_variant(&self) -> Option<T>` default trait method from `AsVariantRef<T>` (see `Added` section for more info).
- Removed `IsVariant<T>` trait and corresponding `IsVariant` derive macro.

## [0.3.0] - 2024-05-24

### Added

- Added `#[enumcapsulate(exclude)]` derive macro helper attribute for exclude specific variants from getting derives.

### Changed

- Made derive macros more lenient by ignoring inapplicable variants (i.e. with fewer or more than one field), rather than bailing out.

## [0.2.2] - 2024-05-23

### Changed

- Updated dependencies:
  - `enumcapsulate-macros` from `0.1.0` to `0.2.1`
- The `VariantDiscriminant` derive macro now also supports enums with arbitrary variant shapes.

## [0.2.1] - 2024-05-05

### Changed

- Updated dependencies:
  - `proc-macro2` from `1.0.79` to `1.0.81`
  - `quote` from `1.0.35` to `1.0.36`
  - `syn` from `2.0.53` to `2.0.60`

## [0.2.0] - 2024-03-21

### Changed

- Updated dependencies:
  - `proc-macro2` from `1.0.76` to `1.0.79`
  - `syn` from `2.0.48` to `2.0.53`
- Bumped MSRV from `1.70.0` to `1.74.0`

## [0.1.0] - TBD

Initial release.
