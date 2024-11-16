# `enumcapsulate-macros`

[![Crates.io](https://img.shields.io/crates/v/enumcapsulate-macros)](https://crates.io/crates/enumcapsulate-macros)
[![Crates.io](https://img.shields.io/crates/d/enumcapsulate-macros)](https://crates.io/crates/enumcapsulate-macros)
[![Crates.io](https://img.shields.io/crates/l/enumcapsulate-macros)](https://crates.io/crates/enumcapsulate-macros)
[![docs.rs](https://docs.rs/enumcapsulate-macros/badge.svg)](https://docs.rs/enumcapsulate-macros/)

Derive macros for [enumcapsulate](https://crates.io/crates/enumcapsulate) crate.

----

## Macros

The `enumcapsulate-macros` proc-macro crate exports the following derive macros:

| Derive macro          | Functionality                                                                                                                                                                     |
| --------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `AsVariant`           | Derives impls for `enumcapsulate::AsVariant<T>` for each variant type `T` where `T: Clone`                                                                                        |
| `AsVariantMut`        | Derives impls for `enumcapsulate::AsVariantMut<T>` for each variant type `T`                                                                                                      |
| `AsVariantRef`        | Derives impls for `enumcapsulate::AsVariantRef<T>` for each variant type `T`                                                                                                      |
| `Encapsulate`         | Umbrella derive macro for `AsVariant`, `AsVariantMut`, `AsVariantRef`, `From`, `FromVariant`, `IntoVariant`, `IsVariant`, `TryInto`, `VariantDiscriminant`, and `VariantDowncast` |
| `From`                | Derives impls for `core::convert::From<T>` for each variant type `T`                                                                                                              |
| `FromVariant`         | Derives impls for `enumcapsulate::FromVariant<T>` for each variant type `T`                                                                                                       |
| `IntoVariant`         | Derives impls for `enumcapsulate::FromVariant<T>` for each variant type `T`                                                                                                       |
| `IsVariant`           | Derives impl for `enumcapsulate::IsVariant`                                                                                                                                       |
| `TryInto`             | Derives impls for `core::convert::TryInto<T>` for each variant type `T`                                                                                                           |
| `VariantDiscriminant` | Derives impl for `enumcapsulate::VariantDiscriminant`                                                                                                                             |

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

## `#[enumcapsulate(include(field = …_)]`

The variant-based derive macros in this crate will skip derives for **any multi-field variant** they find in the enum.

This can lead to undesired false negatives where a variant like `ToBeIncluded` unintentionally
gets detected as variant of type `bool`.

For tuple variants the field can be specified by its index: `#[enumcapsulate(include(field = INDEX))]`
For struct variants the field can be specified by its name: `#[enumcapsulate(include(field = "NAME"))]`

Given an enum like this …

```rust
struct VariantA {
    // ...
}

struct VariantB {
    // ...
}

struct VariantC {
    // ...
}

#[derive(FromVariant)]
enum Enum {
    VariantA(VariantA),
    ToBeIncludedB(bool, VariantB),
    ToBeIncludedC { flag: bool, variant: VariantC },
}
```

… the following implementations get derived from the code above:

```rust
impl FromVariant<VariantA> for Enum {
    // ...
}

// Notice how the variants `ToBeIncludedB { … }` and `ToBeIncludedC { … }`
// produced no impls, due to having more than one field.
```

Adding `#[enumcapsulate(include(field = …))]` to the desired variant,
with the desired variant field specified …

```rust
#[derive(FromVariant)]
enum Enum {
    // ...
    #[enumcapsulate(include(field = 1))]
    ToBeIncludedB(bool, VariantB),
    #[enumcapsulate(include(field = "variant"))]
    ToBeIncludedC { flag: bool, variant: VariantC },
}
```

… makes the variants have their `impl FromVariant<…> for Enum` get derived as desired:

```rust
// ...

impl FromVariant<VariantB> for Enum {
    // ...
}

impl FromVariant<VariantC> for Enum {
    // ...
}
```

Note however that `#[derive(From)]` and `#[derive(FromVariant)]` (at the current time)
still won't generate impls for variants with more than one field.

## Helper attribute support

Check the matrix below for which derive macros support which helper attributes:

|                       | `#[enumcapsulate(exclude)]` | `#[enumcapsulate(include(field = …))]` |
| --------------------- | --------------------------- | -------------------------------------- |
| `AsVariant`           | ✔ supported                 | ✔ supported                            |
| `AsVariantMut`        | ✔ supported                 | ✔ supported                            |
| `AsVariantRef`        | ✔ supported                 | ✔ supported                            |
| `Encapsulate`         | ✔ supported                 | ✔ supported                            |
| `From`                | ✔ supported                 | ✘ not supported                        |
| `FromVariant`         | ✔ supported                 | ✘ not supported                        |
| `IntoVariant`         | ✔ supported                 | ✔ supported                            |
| `IsVariant`           | ✔ supported                 | ✔ supported                            |
| `TryInto`             | ✔ supported                 | ✔ supported                            |
| `VariantDiscriminant` | ✘ not supported             | ✘ not supported                        |

## Documentation

Please refer to the documentation on [docs.rs](https://docs.rs/enumcapsulate-macros).

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our [code of conduct](https://www.rust-lang.org/conduct.html),  
and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/regexident/enumcapsulate-macros/tags).

## License

This project is licensed under the [**MPL-2.0**](https://www.tldrlegal.com/l/mpl-2.0) – see the [LICENSE.md](LICENSE.md) file for details.
