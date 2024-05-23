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
    pub fn derive_from(&self) -> Result<TokenStream2, syn::Error> {
        let outer = &self.ident;
        let variants = utils::infos_per_newtype_variant(&self.variants);

        let impls = variants.into_iter().map(|variant_info| {
            let VariantInfo {
                ident: inner,
                inner_ty,
            } = variant_info;
            quote! {
                impl ::core::convert::From<#inner_ty> for #outer {
                    fn from(inner: #inner_ty) -> Self {
                        Self::#inner(inner)
                    }
                }
            }
        });

        Ok(quote! {
            #(#impls)*
        })
    }

    pub fn derive_try_into(&self) -> Result<TokenStream2, syn::Error> {
        let outer = &self.ident;
        let variants = utils::infos_per_newtype_variant(&self.variants);

        let impls = variants.into_iter().map(|variant_info| {
            let VariantInfo {
                ident: inner,
                inner_ty,
            } = variant_info;
            quote! {
                impl ::core::convert::TryFrom<#outer> for #inner_ty {
                    type Error = #outer;

                    fn try_from(outer: #outer) -> Result<Self, Self::Error> {
                        match outer {
                            #outer::#inner(inner) => Ok(inner),
                            err => Err(err)
                        }
                    }
                }
            }
        });

        Ok(quote! {
            #(#impls)*
        })
    }

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

    pub fn derive_into_variant(&self) -> Result<TokenStream2, syn::Error> {
        let outer = &self.ident;
        let variants = utils::infos_per_newtype_variant(&self.variants);

        let impls = variants.into_iter().map(|variant_info| {
            let VariantInfo {
                ident: inner,
                inner_ty,
            } = variant_info;
            quote! {
                impl ::enumcapsulate::IntoVariant<#inner_ty> for #outer {
                    fn into_variant(self) -> Result<#inner_ty, Self> {
                        match self {
                            #outer::#inner(variant) => Ok(variant),
                            err => Err(err)
                        }
                    }
                }
            }
        });

        Ok(quote! {
            #(#impls)*
        })
    }

    pub fn derive_variant_downcast(&self) -> Result<TokenStream2, syn::Error> {
        let outer = &self.ident;

        let tokens = quote! {
            impl ::enumcapsulate::VariantDowncast for #outer {}
        };

        Ok(tokens)
    }

    pub fn derive_is_variant(&self) -> Result<TokenStream2, syn::Error> {
        let outer = &self.ident;
        let variants = utils::infos_per_newtype_variant(&self.variants);

        let match_arms: Vec<_> = variants
            .into_iter()
            .map(|variant_info| {
                let VariantInfo {
                    ident: inner,
                    inner_ty: _,
                } = variant_info;

                quote! {
                    #outer::#inner(variant) => type_id_of_val(variant) == type_id,
                }
            })
            .collect();

        let method_body = if match_arms.is_empty() {
            quote! {
                unreachable!()
            }
        } else {
            quote! {
                use ::std::any::TypeId;

                #[inline]
                pub fn type_id_of_val<T: 'static + ?Sized>(_val: &T) -> TypeId {
                    TypeId::of::<T>()
                }

                let type_id = TypeId::of::<T>();

                match self {
                    #(#match_arms)*
                }
            }
        };

        Ok(quote! {
            impl ::enumcapsulate::IsVariant for #outer {
                fn is_variant<T>(&self) -> bool
                where
                    T: 'static + ?Sized
                {
                    #method_body
                }
            }
        })
    }

    pub fn derive_variant_discriminant(&self) -> Result<TokenStream2, syn::Error> {
        let outer = &self.ident;
        let variant_idents = utils::variant_idents(&self.variants);

        let discriminant_ident = quote::format_ident!("{outer}Discriminant");

        let discriminant_enum = quote! {
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
            pub enum #discriminant_ident {
                #(#variant_idents),*
            }
        };

        let match_arms: Vec<_> = self
            .variants
            .iter()
            .map(|variant| {
                let ident = &variant.ident;
                let pattern = match variant.fields {
                    syn::Fields::Named(_) => quote! {
                        #outer::#ident { .. }
                    },
                    syn::Fields::Unnamed(_) => quote! {
                        #outer::#ident(..)
                    },
                    syn::Fields::Unit => quote! {
                        #outer::#ident
                    },
                };
                quote! {
                    #pattern => #discriminant_ident::#ident,
                }
            })
            .collect();

        let method_body = if match_arms.is_empty() {
            quote! {
                unreachable!()
            }
        } else {
            quote! {
                match self {
                    #(#match_arms)*
                }
            }
        };

        Ok(quote! {
            #discriminant_enum

            impl ::enumcapsulate::VariantDiscriminant for #outer {
                type Discriminant = #discriminant_ident;

                fn variant_discriminant(&self) -> Self::Discriminant {
                    #method_body
                }
            }
        })
    }
}
