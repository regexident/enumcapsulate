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

# Macro helper attributes

Most of the derive macros support helper attributes:

## `#[enumcapsulate(exclude)]`

The variant-based derive macros in this crate will perform derives for **every single-field variant** they find in the enum.
This can lead to undesired false positives where a variant like `ToBeExcluded` unintentionally
gets detected as variant of type `bool`.

Given an enum like this …

```rust
struct VariantA {
    // ...
}

struct VariantB {
    // ...
}

#[derive(FromVariant)]
enum Enum {
    VariantA(VariantA),
    VariantB(VariantB),
    ToBeExcluded { flagToBeExcluded: bool },
}
```

… the following implementations get derived from the code above:

```rust
impl FromVariant<VariantA> for Enum {
    // ...
}

impl FromVariant<VariantB> for Enum {
    // ...
}

// Notice how the derive picked up the `is_cool: bool` field
// as an inner variant and generated an impl for it:

impl FromVariant<bool> for Enum {
    // ...
}
```

Adding `#[enumcapsulate(exclude)]` to the undesired variant …

```rust
#[derive(FromVariant)]
enum Enum {
    // ...
    #[enumcapsulate(exclude)]
    ToBeExcluded { flag: bool },
}
```

… makes the undesired `impl FromVariant<bool> for Enum` get omitted.

## Helper attribute support

Check the matrix below for which derive macros support which helper attributes:

|                       | `#[enumcapsulate(exclude)]` |
| --------------------- | --------------------------- |
| `AsVariant`           | ✔ supported                 |
| `AsVariantMut`        | ✔ supported                 |
| `AsVariantRef`        | ✔ supported                 |
| `Encapsulate`         | ✔ supported                 |
| `From`                | ✔ supported                 |
| `FromVariant`         | ✔ supported                 |
| `IntoVariant`         | ✔ supported                 |
| `IsVariant`           | ✔ supported                 |
| `TryInto`             | ✔ supported                 |
| `VariantDiscriminant` | ✘ not supported             |

## Documentation

Please refer to the documentation on [docs.rs](https://docs.rs/enumcapsulate-macros).

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our [code of conduct](https://www.rust-lang.org/conduct.html),  
and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/regexident/enumcapsulate-macros/tags).

## License

This project is licensed under the [**MPL-2.0**](https://www.tldrlegal.com/l/mpl-2.0) – see the [LICENSE.md](LICENSE.md) file for details.
