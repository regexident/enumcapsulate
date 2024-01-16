use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, DataEnum, DeriveInput, Ident, Variant};

use crate::utils::{self, VariantInfo};

pub(crate) struct EnumDeriver {
    ident: Ident,
    variants: Punctuated<Variant, Comma>,
}

impl TryFrom<DeriveInput> for EnumDeriver {
    type Error = syn::Error;

    fn try_from(input: DeriveInput) -> Result<Self, Self::Error> {
        let DeriveInput { ident, data, .. } = input;

        let syn::Data::Enum(DataEnum { variants, .. }) = data else {
            return Err(syn::Error::new(
                ident.span(),
                "Only enums can use this derive",
            ));
        };

        Ok(Self { ident, variants })
    }
}
