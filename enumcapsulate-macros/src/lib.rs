use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

use crate::utils::tokenstream;

use self::enum_deriver::EnumDeriver;

mod enum_deriver;
mod utils;

#[proc_macro_derive(From)]
pub fn derive_from(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    tokenstream(|| {
        let deriver = EnumDeriver::try_from(input)?;
        deriver.derive_from()
    })
}

#[proc_macro_derive(FromVariant)]
pub fn derive_from_variant(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    tokenstream(|| {
        let deriver = EnumDeriver::try_from(input)?;
        deriver.derive_from_variant()
    })
}

#[proc_macro_derive(AsVariantRef)]
pub fn derive_as_variant_ref(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    tokenstream(|| {
        let deriver = EnumDeriver::try_from(input)?;
        deriver.derive_as_variant_ref()
    })
}

#[proc_macro_derive(AsVariantMut)]
pub fn derive_as_variant_mut(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    tokenstream(|| {
        let deriver = EnumDeriver::try_from(input)?;
        deriver.derive_as_variant_mut()
    })
}

#[proc_macro_derive(IntoVariant)]
pub fn derive_into_variant(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    tokenstream(|| {
        let deriver = EnumDeriver::try_from(input)?;
        deriver.derive_into_variant()
    })
}

#[proc_macro_derive(VariantDowncast)]
pub fn derive_variant_downcast(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    tokenstream(|| {
        let deriver = EnumDeriver::try_from(input)?;
        deriver.derive_variant_downcast()
    })
}
