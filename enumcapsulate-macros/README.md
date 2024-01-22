# `enumcapsulate-macros`

[![Crates.io](https://img.shields.io/crates/v/enumcapsulate-macros)](https://crates.io/crates/enumcapsulate-macros)
[![Crates.io](https://img.shields.io/crates/d/enumcapsulate-macros)](https://crates.io/crates/enumcapsulate-macros)
[![Crates.io](https://img.shields.io/crates/l/enumcapsulate-macros)](https://crates.io/crates/enumcapsulate-macros)
[![docs.rs](https://docs.rs/enumcapsulate-macros/badge.svg)](https://docs.rs/enumcapsulate-macros/)

Derive macros for [enumcapsulate](https://crates.io/crates/enumcapsulate) crate.

----

## Macros

The `enumcapsulate-macros` proc-macro crate exports the following derive macros:

| Derive macro          | Functionality                                                                                                                                                                       |
| --------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `AsVariant`           | Umbrella derive macro for `AsVariantRef`, and `AsVariantMut`                                                                                                                        |
| `AsVariantMut`        | Derive impls for `enumcapsulate::AsVariantMut<T>` for each variant type `T`                                                                                                         |
| `AsVariantRef`        | Derive impls for `enumcapsulate::AsVariantRef<T>` for each variant type `T`                                                                                                         |
| `Encapsulate`         | Umbrella derive macro for `AsVariantMut`, `AsVariantRef`, `From`, `FromVariant`, `IntoVariant`, `IsVariant`, `TryInto`, `VariantDiscriminant`, and `VariantDowncast` `AsVariantMut` |
| `From`                | Derive impls for `core::convert::From<T>` for each variant type `T`                                                                                                                 |
| `FromVariant`         | Derive impls for `enumcapsulate::FromVariant<T>` for each variant type `T`                                                                                                          |
| `IntoVariant`         | Derive impls for `enumcapsulate::FromVariant<T>` for each variant type `T`                                                                                                          |
| `IsVariant`           | Derive impl for `enumcapsulate::IsVariant`                                                                                                                                          |
| `TryInto`             | Derive impls for `core::convert::TryInto<T>` for each variant type `T`                                                                                                              |
| `VariantDiscriminant` | Derive impl for `enumcapsulate::VariantDiscriminant`                                                                                                                                |

## Documentation

Please refer to the documentation on [docs.rs](https://docs.rs/enumcapsulate-macros).

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our [code of conduct](https://www.rust-lang.org/conduct.html),  
and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/regexident/enumcapsulate-macros/tags).

## License

This project is licensed under the [**MPL-2.0**](https://www.tldrlegal.com/l/mpl-2.0) – see the [LICENSE.md](LICENSE.md) file for details.
