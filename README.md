# `enumcapsulate`

[![Crates.io](https://img.shields.io/crates/v/enumcapsulate)](https://crates.io/crates/enumcapsulate)
[![Crates.io](https://img.shields.io/crates/d/enumcapsulate)](https://crates.io/crates/enumcapsulate)
[![Crates.io](https://img.shields.io/crates/l/enumcapsulate)](https://crates.io/crates/enumcapsulate)
[![docs.rs](https://docs.rs/enumcapsulate/badge.svg)](https://docs.rs/enumcapsulate/)

Safe casting for newtype enums and their variants.

----

## Traits

The `enumcapsulate` crate exports the following traits:

| Traits                | Functionality                                                                                                                                  |
| --------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| `Encapsulate`         | Umbrella derive macro for `AsVariant`, `AsVariantMut`, `AsVariantRef`, `From`, `FromVariant`, `IntoVariant`, `TryInto`, and `VariantDowncast`. |
| `FromVariant`         | Creates an instance of `Self` from the unambiguous field type of one of its variants.                                                          |
| `IntoVariant`         | Returns the current variant's field, consuming `self`.                                                                                         |
| `AsVariant`           | Provides owned access to the current variant's field.                                                                                          |
| `AsVariantMut`        | Provides mutable borrowed access to the current variant's field.                                                                               |
| `AsVariantRef`        | Provides borrowed access to the current variant's field.                                                                                       |
| `VariantDowncast`     | Convenience umbrella trait utilizing `AsVariant`, `AsVariantRef`, `AsVariantMut`, and `IntoVariant`.                                           |
| `VariantDiscriminant` | Used to obtain an enum variant's discriminant.                                                                                                 |

## Derive macros

The `enumcapsulate` crate exports the following corresponding derive macros, if the `"derive"` feature is enabled (which is the default):

- `Encapsulate`
- `From`
- `TryInto`
- `FromVariant`
- `IntoVariant`
- `AsVariant`
- `AsVariantMut`
- `AsVariantRef`
- `VariantDowncast`
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
