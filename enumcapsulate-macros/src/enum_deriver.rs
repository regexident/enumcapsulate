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

impl EnumDeriver {
    pub fn derive_from_variant(&self) -> Result<TokenStream2, syn::Error> {
        let outer = &self.ident;
        let variants = utils::infos_per_newtype_variant(&self.variants);

        let impls = variants.into_iter().map(|variant_info| {
            let VariantInfo {
                ident: inner,
                inner_ty,
            } = variant_info;
            quote! {
                impl ::enumcapsulate::FromVariant<#inner_ty> for #outer {
                    fn from_variant(inner: #inner_ty) -> Self {
                        Self::#inner(inner)
                    }
                }
            }
        });

        Ok(quote! {
            #(#impls)*
        })
    }

    pub fn derive_as_variant_ref(&self) -> Result<TokenStream2, syn::Error> {
        let outer = &self.ident;
        let variants = utils::infos_per_newtype_variant(&self.variants);

        let impls = variants.into_iter().map(|variant_info| {
            let VariantInfo {
                ident: inner,
                inner_ty,
            } = variant_info;
            quote! {
                impl ::enumcapsulate::AsVariantRef<#inner_ty> for #outer {
                    fn as_variant_ref(&self) -> Option<&#inner_ty> {
                        match self {
                            #outer::#inner(variant) => Some(variant),
                            _ => None
                        }
                    }
                }
            }
        });

        Ok(quote! {
            #(#impls)*
        })
    }

    pub fn derive_as_variant_mut(&self) -> Result<TokenStream2, syn::Error> {
        let outer = &self.ident;
        let variants = utils::infos_per_newtype_variant(&self.variants);

        let impls = variants.into_iter().map(|variant_info| {
            let VariantInfo {
                ident: inner,
                inner_ty,
            } = variant_info;
            quote! {
                impl ::enumcapsulate::AsVariantMut<#inner_ty> for #outer {
                    fn as_variant_mut(&mut self) -> Option<&mut #inner_ty> {
                        match self {
                            #outer::#inner(variant) => Some(variant),
                            _ => None
                        }
                    }
                }
            }
        });

        Ok(quote! {
            #(#impls)*
        })
    }
}
