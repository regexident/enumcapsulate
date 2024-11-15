# `enumcapsulate`

[![Crates.io](https://img.shields.io/crates/v/enumcapsulate)](https://crates.io/crates/enumcapsulate)
[![Crates.io](https://img.shields.io/crates/d/enumcapsulate)](https://crates.io/crates/enumcapsulate)
[![Crates.io](https://img.shields.io/crates/l/enumcapsulate)](https://crates.io/crates/enumcapsulate)
[![docs.rs](https://docs.rs/enumcapsulate/badge.svg)](https://docs.rs/enumcapsulate/)

Safe casting for newtype enums and their variants.

----

## Traits

The `enumcapsulate` crate exports the following traits:

| Traits                | Functionality                                                                                                                                                                       |
| --------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `AsVariant`           | Umbrella derive macro for `AsVariantRef`, and `AsVariantMut`                                                                                                                        |
| `AsVariantMut`        | Used to do a cheap mutable-to-mutable reference conversion between an outer enum's and its inner variant's type                                                                     |
| `AsVariantRef`        | Used to do a cheap reference-to-reference reference conversion between an outer enum's and its inner variant's type                                                                 |
| `Encapsulate`         | Umbrella derive macro for `AsVariantMut`, `AsVariantRef`, `From`, `FromVariant`, `IntoVariant`, `IsVariant`, `TryInto`, `VariantDiscriminant`, and `VariantDowncast` `AsVariantMut` |
| `FromVariant`         | Used to do variant-to-enum conversions between an outer enum's and its inner variant's type                                                                                         |
| `IntoVariant`         | Used to do enum-to-variant conversions between an outer enum's and its inner variant's type                                                                                         |
| `IsVariant`           | Used to check type of an enum's inner variant's type                                                                                                                                |
| `VariantDiscriminant` | Used to obtain an enum variant's discriminant                                                                                                                                       |
| `VariantDowncast`     | Convenience umbrella trait utilizing `AsVariantRef`, `AsVariantMut`, and `IntoVariant`                                                                                              |

## Derive macros

The `enumcapsulate` crate exports the following corresponding derive macros, if the `"derive"` feature is enabled (which is the default):

- `AsVariant`
- `AsVariantMut`
- `AsVariantRef`
- `Encapsulate`
- `From`
- `FromVariant`
- `IntoVariant`
- `IsVariant`
- `TryInto`
- `VariantDiscriminant`

## Documentation

Please refer to the documentation on [docs.rs](https://docs.rs/enumcapsulate).

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our [code of conduct](https://www.rust-lang.org/conduct.html),  
and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/regexident/enumcapsulate/tags).

## License

This project is licensed under the [**MPL-2.0**](https://www.tldrlegal.com/l/mpl-2.0) – see the [LICENSE.md](LICENSE.md) file for details.
