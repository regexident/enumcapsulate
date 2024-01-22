use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

use crate::utils::tokenstream;

use self::enum_deriver::EnumDeriver;

mod enum_deriver;
mod utils;

/// Derive macro generating an impl of the trait `From<T>`.
///
/// It generates an impl for each of the enum's
/// newtype variants, where `Inner` is the variant's field type
/// and `Outer` is the enclosing enum's type.
///
/// ```
/// struct Inner;
///
/// enum Outer {
///     Inner(Inner),
///     // ...
/// }
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
#[proc_macro_derive(From)]
pub fn derive_from(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    tokenstream(|| {
        let deriver = EnumDeriver::try_from(input)?;
        deriver.derive_from()
    })
}

/// Derive macro generating an impl of the trait `TryFrom<T>`.
///
/// It generates an impl for each of the enum's newtype variants,
/// where `Inner` is the variant's field type and `Outer`
/// is the enclosing enum's type.
///
/// ```
/// struct Inner;
///
/// enum Outer {
///     Inner(Inner),
///     // ...
/// }
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
#[proc_macro_derive(TryInto)]
pub fn derive_try_from(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    tokenstream(|| {
        let deriver = EnumDeriver::try_from(input)?;
        deriver.derive_try_into()
    })
}

/// Derive macro generating an impl of the trait `FromVariant<T>`.
///
/// It generates an impl for each of the enum's
/// newtype variants, where `Inner` is the variant's field type
/// and `Outer` is the enclosing enum's type.
///
/// ```
/// # use enumcapsulate::FromVariant;
/// #
/// # struct Inner;
///
/// enum Outer {
///     Inner(Inner),
///     // ...
/// }
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
#[proc_macro_derive(FromVariant)]
pub fn derive_from_variant(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    tokenstream(|| {
        let deriver = EnumDeriver::try_from(input)?;
        deriver.derive_from_variant()
    })
}

/// Derive macro generating an impl of the trait `FromVariant<T>`.
///
/// It generates an impl for each of the enum's
/// newtype variants, where `Inner` is the variant's field type
/// and `Outer` is the enclosing enum's type.
///
/// ```
/// # use enumcapsulate::AsVariantRef;
///
/// struct Inner;
///
/// enum Outer {
///     Inner(Inner),
///     // ...
/// }
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
#[proc_macro_derive(AsVariantRef)]
pub fn derive_as_variant_ref(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    tokenstream(|| {
        let deriver = EnumDeriver::try_from(input)?;
        deriver.derive_as_variant_ref()
    })
}

/// Derive macro generating an impl of the trait `From<T>`.
///
/// It generates an impl for each of the enum's
/// newtype variants, where `Inner` is the variant's field type
/// and `Outer` is the enclosing enum's type.
///
/// ```
/// # use enumcapsulate::AsVariantMut;
///
/// struct Inner;
///
/// enum Outer {
///     Inner(Inner),
///     // ...
/// }
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
#[proc_macro_derive(AsVariantMut)]
pub fn derive_as_variant_mut(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    tokenstream(|| {
        let deriver = EnumDeriver::try_from(input)?;
        deriver.derive_as_variant_mut()
    })
}

/// Derive macro generating an impl of the trait `AsVariant<T>`.
///
/// It generates an impl for each of the enum's
/// newtype variants, where `Inner` is the variant's field type
/// and `Outer` is the enclosing enum's type.
///
/// ```
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
/// impl AsVariant<Inner> for Outer {
///     fn as_variant(&self) -> Option<Inner> {
///          match self {
///             Outer::Inner(inner) => Some(inner.clone()),
///             _ => None,
///         }
///     }
/// }
///
/// // ...
/// ```
///
#[proc_macro_derive(AsVariant)]
pub fn derive_as_variant(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    tokenstream(|| {
        let deriver = EnumDeriver::try_from(input)?;

        let as_variant_ref = deriver.derive_as_variant_ref()?;
        let as_variant_mut = deriver.derive_as_variant_mut()?;

        Ok(quote::quote! {
            #as_variant_ref
            #as_variant_mut
        })
    })
}

/// Derive macro generating an impl of the trait `IntoVariant<T>`.
///
/// It generates an impl for each of the enum's
/// newtype variants, where `Inner` is the variant's field type
/// and `Outer` is the enclosing enum's type.
///
/// ```
/// # use enumcapsulate::IntoVariant;
///
/// struct Inner;
///
/// enum Outer {
///     Inner(Inner),
///     // ...
/// }
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
#[proc_macro_derive(IntoVariant)]
pub fn derive_into_variant(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    tokenstream(|| {
        let deriver = EnumDeriver::try_from(input)?;
        deriver.derive_into_variant()
    })
}

/// Derive macro generating an impl of the trait `VariantDowncast`.
///
/// ```
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
#[proc_macro_derive(VariantDowncast)]
pub fn derive_variant_downcast(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    tokenstream(|| {
        let deriver = EnumDeriver::try_from(input)?;
        deriver.derive_variant_downcast()
    })
}

/// Derive macro generating an impl of the trait `IsVariant`.
///
/// ```
/// struct Inner;
///
/// enum Outer {
///     Inner(Inner),
///     // ...
/// }
///
/// impl IsVariant for Outer {
///     fn is_variant<T>(&self) -> bool
///     where
///         T: 'static + ?Sized
///     {
///         match self {
///            Outer::Inner(inner) => /* ... */,
///            // ...
///        }
///     }
/// }
///
/// // ...
/// ```
///
#[proc_macro_derive(IsVariant)]
pub fn derive_is_variant(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    tokenstream(|| {
        let deriver = EnumDeriver::try_from(input)?;
        deriver.derive_is_variant()
    })
}

/// Derive macro generating an impl of the trait `VariantDiscriminant`.
///
/// ```
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
#[proc_macro_derive(VariantDiscriminant)]
pub fn derive_variant_discriminant(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    tokenstream(|| {
        let deriver = EnumDeriver::try_from(input)?;
        deriver.derive_variant_discriminant()
    })
}

/// Umbrella derive macro.
///
/// The following use of the `Encapsulate` umbrella derive macro:
///
/// ```rust
/// use enumcapsulate::derive::Encapsulate;
///
/// #[derive(Encapsulate)
/// enum Outer {
///     // ...
/// }
/// ```
///
/// is equivalent to the following:
///
/// ```
/// # use enumcapsulate::{
/// #     derive::{
/// #         From, TryInto, FromVariant, AsVariantRef, AsVariantMut, IntoVariant, VariantDowncast, IsVariant, VariantDiscriminant
/// #     },
/// # };
///
/// // ...
///
/// #[derive(From, TryInto, FromVariant, AsVariantRef, AsVariantMut, IntoVariant, VariantDowncast, IsVariant, VariantDiscriminant)]
/// enum Outer {
///     // ...
/// }
/// ```
#[proc_macro_derive(Encapsulate)]
pub fn derive_encapsulate(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    tokenstream(|| {
        let deriver = EnumDeriver::try_from(input)?;

        let from = deriver.derive_from()?;
        let try_into = deriver.derive_try_into()?;
        let from_variant = deriver.derive_from_variant()?;
        let as_variant_ref = deriver.derive_as_variant_ref()?;
        let as_variant_mut = deriver.derive_as_variant_mut()?;
        let into_variant = deriver.derive_into_variant()?;
        let variant_downcast = deriver.derive_variant_downcast()?;
        let is_variant = deriver.derive_is_variant()?;
        let variant_discriminant = deriver.derive_variant_discriminant()?;

        Ok(quote::quote! {
            #from
            #try_into
            #from_variant
            #as_variant_ref
            #as_variant_mut
            #into_variant
            #variant_downcast
            #is_variant
            #variant_discriminant
        })
    })
}
