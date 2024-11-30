use proc_macro::TokenStream;
use syn::parse_macro_input;

use crate::utils::tokenstream;

mod config;
mod enum_deriver;
mod type_visitor;
mod utils;

use self::{config::*, enum_deriver::*, type_visitor::*, utils::*};

/// Derive macro generating an impl of the trait `From<T>`.
///
/// It generates an impl for each of the enum's
/// newtype variants, where `Inner` is the variant's field type
/// and `Outer` is the enclosing enum's type.
///
/// ```ignore
/// struct Inner;
///
/// enum Outer {
///     Inner(Inner),
///     // ...
/// }
///
/// // The generated impls look something along these lines:
///
/// impl From<Inner> for Outer {
///     fn from(inner: Inner) -> Self {
///         Outer::Inner(inner)
///     }
/// }
///
/// // ...
/// ```
///
#[proc_macro_derive(From, attributes(enumcapsulate))]
pub fn derive_from(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as syn::ItemEnum);

    tokenstream(|| {
        let deriver = EnumDeriver::from(item);
        deriver.derive_from()
    })
}

/// Derive macro generating an impl of the trait `TryFrom<T>`.
///
/// It generates an impl for each of the enum's newtype variants,
/// where `Inner` is the variant's field type and `Outer`
/// is the enclosing enum's type.
///
/// ```ignore
/// struct Inner;
///
/// enum Outer {
///     Inner(Inner),
///     // ...
/// }
///
/// // The generated impls look something along these lines:
///
/// impl TryFrom<Outer> for Inner {
///     type Error = Outer;
///
///     fn try_from(outer: Outer) -> Result<Self, Self::Error> {
///         match outer {
///             Outer::Inner(inner) => Ok(inner),
///             err => Err(err),
///         }
///     }
/// }
///
/// // ...
/// ```
///
/// Note: Despite the derive macro's name it's actually `TryFrom<T>`
/// that's being derives, not `TryInto<T>`.
/// But since the macro is derived on the enum the latter feels
/// more appropriate as the derive's name.
#[proc_macro_derive(TryInto, attributes(enumcapsulate))]
pub fn derive_try_from(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as syn::ItemEnum);

    tokenstream(|| {
        let deriver = EnumDeriver::from(item);
        deriver.derive_try_into()
    })
}

/// Derive macro generating an impl of the trait `FromVariant<T>`.
///
/// It generates an impl for each of the enum's
/// newtype variants, where `Inner` is the variant's field type
/// and `Outer` is the enclosing enum's type.
///
/// ```ignore
/// # use enumcapsulate::FromVariant;
/// #
/// # struct Inner;
///
/// enum Outer {
///     Inner(Inner),
///     // ...
/// }
///
/// // The generated impls look something along these lines:
///
/// impl FromVariant<Inner> for Outer {
///     fn from_variant(inner: Inner) -> Self {
///         Outer::Inner(inner)
///     }
/// }
///
/// // ...
/// ```
///
#[proc_macro_derive(FromVariant, attributes(enumcapsulate))]
pub fn derive_from_variant(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as syn::ItemEnum);

    tokenstream(|| {
        let deriver = EnumDeriver::from(item);
        deriver.derive_from_variant()
    })
}

/// Derive macro generating an impl of the trait `FromVariant<T>`.
///
/// It generates an impl for each of the enum's
/// newtype variants, where `Inner` is the variant's field type
/// and `Outer` is the enclosing enum's type.
///
/// ```ignore
/// # use enumcapsulate::AsVariant;
///
/// #[derive(Clone)]
/// struct Inner;
///
/// enum Outer {
///     Inner(Inner),
///     // ...
/// }
///
/// // The generated impls look something along these lines:
///
/// impl AsVariant<Inner> for Outer where Inner: Clone {
///     fn as_variant(&self) -> Option<Inner> {
///         match self {
///             Outer::Inner(inner) => Some(inner.clone()),
///             _ => None,
///         }
///     }
/// }
///
/// // ...
/// ```
///
#[proc_macro_derive(AsVariant, attributes(enumcapsulate))]
pub fn derive_as_variant(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as syn::ItemEnum);

    tokenstream(|| {
        let deriver = EnumDeriver::from(item);
        deriver.derive_as_variant()
    })
}

/// Derive macro generating an impl of the trait `FromVariant<T>`.
///
/// It generates an impl for each of the enum's
/// newtype variants, where `Inner` is the variant's field type
/// and `Outer` is the enclosing enum's type.
///
/// ```ignore
/// # use enumcapsulate::AsVariantRef;
///
/// struct Inner;
///
/// enum Outer {
///     Inner(Inner),
///     // ...
/// }
///
/// // The generated impls look something along these lines:
///
/// impl AsVariantRef<Inner> for Outer {
///     fn as_variant_ref(&self) -> Option<&Inner> {
///         match self {
///             Outer::Inner(inner) => Some(inner),
///             _ => None,
///         }
///     }
/// }
///
/// // ...
/// ```
///
#[proc_macro_derive(AsVariantRef, attributes(enumcapsulate))]
pub fn derive_as_variant_ref(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as syn::ItemEnum);

    tokenstream(|| {
        let deriver = EnumDeriver::from(item);
        deriver.derive_as_variant_ref()
    })
}

/// Derive macro generating an impl of the trait `From<T>`.
///
/// It generates an impl for each of the enum's
/// newtype variants, where `Inner` is the variant's field type
/// and `Outer` is the enclosing enum's type.
///
/// ```ignore
/// # use enumcapsulate::AsVariantMut;
///
/// struct Inner;
///
/// enum Outer {
///     Inner(Inner),
///     // ...
/// }
///
/// // The generated impls look something along these lines:
///
/// impl AsVariantMut<Inner> for Outer {
///     fn as_variant_mut(&mut self) -> Option<&mut Inner> {
///         match self {
///             Outer::Inner(inner) => Some(inner),
///             _ => None,
///         }
///     }
/// }
///
/// // ...
/// ```
///
#[proc_macro_derive(AsVariantMut, attributes(enumcapsulate))]
pub fn derive_as_variant_mut(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as syn::ItemEnum);

    tokenstream(|| {
        let deriver = EnumDeriver::from(item);
        deriver.derive_as_variant_mut()
    })
}

/// Derive macro generating an impl of the trait `IntoVariant<T>`.
///
/// It generates an impl for each of the enum's
/// newtype variants, where `Inner` is the variant's field type
/// and `Outer` is the enclosing enum's type.
///
/// ```ignore
/// # use enumcapsulate::IntoVariant;
///
/// struct Inner;
///
/// enum Outer {
///     Inner(Inner),
///     // ...
/// }
///
/// // The generated impls look something along these lines:
///
/// impl IntoVariant<Inner> for Outer {
///     fn into_variant(self) -> Result<T, Self> {
///         match self {
///             Outer::Inner(inner) => Ok(inner),
///             err => Err(err),
///         }
///     }
/// }
///
/// // ...
/// ```
///
/// Note: Despite the derive macro's name it's actually `TryFrom<T>`
/// that's being derives, not `TryInto<T>`.
/// But since the macro is derived on the enum the latter feels
/// more appropriate as the derive's name.
#[proc_macro_derive(IntoVariant, attributes(enumcapsulate))]
pub fn derive_into_variant(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as syn::ItemEnum);

    tokenstream(|| {
        let deriver = EnumDeriver::from(item);
        deriver.derive_into_variant()
    })
}

/// Derive macro generating an impl of the trait `VariantDowncast`.
///
/// ```ignore
/// struct Inner;
///
/// enum Outer {
///     Inner(Inner),
///     // ...
/// }
///
/// // The generated impl looks something along these lines:
///
/// impl VariantDowncast for Outer {}
/// ```
///
#[proc_macro_derive(VariantDowncast, attributes(enumcapsulate))]
pub fn derive_variant_downcast(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as syn::ItemEnum);

    tokenstream(|| {
        let deriver = EnumDeriver::from(item);
        deriver.derive_variant_downcast()
    })
}

/// Derive macro generating an impl of the trait `VariantDiscriminant`.
///
/// ```ignore
/// struct Inner;
///
/// enum Outer {
///     Inner(Inner),
///     // ...
/// }
///
/// // The generated discriminant type looks something along these lines:
///
/// #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
/// pub enum OuterDiscriminant {
///     Inner,
///     // ...
/// }
///
/// // The generated impl looks something along these lines:
///
/// impl VariantDiscriminant for Outer {
///     type Discriminant = OuterDiscriminant;
///
///     fn variant_discriminant(&self) -> Self::Discriminant {
///         match self {
///            Outer::Inner(_) => OuterDiscriminant::Inner,
///            // ...
///        }
///     }
/// }
/// ```
///
#[proc_macro_derive(VariantDiscriminant, attributes(enumcapsulate))]
pub fn derive_variant_discriminant(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as syn::ItemEnum);

    tokenstream(|| {
        let deriver = EnumDeriver::from(item);
        deriver.derive_variant_discriminant()
    })
}

/// Umbrella derive macro.
///
/// The following use of the `Encapsulate` umbrella derive macro:
///
/// ```ignore
/// use enumcapsulate::Encapsulate;
///
/// #[derive(Encapsulate)
/// enum Enum {
///     // ...
/// }
/// ```
///
/// is equivalent to the following:
///
/// ```ignore
/// // ...
///
/// #[derive(
///     From,
///     TryInto,
///     FromVariant,
///     IntoVariant,
///     AsVariant,
///     AsVariantMut,
///     AsVariantRef,
///     VariantDowncast,
/// )]
/// enum Enum {
///     // ...
/// }
/// ```
///
/// If you wish to opt out of a select few of `Encapsulate`'s trait derives,
/// then you can do so by use of an `#[enumcapsulate(exclude(…))]` attribute
/// on the enum itself, such as if you wanted to exclude `From` and `TryInto`:
///
/// ```rust
/// #[derive(Encapsulate)]
/// #[enumcapsulate(exclude(From, TryInto))]
/// enum Enum {
///     // ...
/// }
/// ```
#[proc_macro_derive(Encapsulate, attributes(enumcapsulate))]
pub fn derive_encapsulate(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as syn::ItemEnum);

    tokenstream(|| {
        let deriver = EnumDeriver::from(item);
        deriver.derive_encapsulate()
    })
}
