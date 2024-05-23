use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, DataEnum, DeriveInput, Fields, Ident, Variant};

use crate::utils::{self, FieldInfo, VariantInfo};

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
        let variant_infos: Vec<VariantInfo> = utils::variant_infos(&self.variants);

        let mut impls: Vec<TokenStream2> = vec![];

        for variant_info in variant_infos {
            let VariantInfo {
                ident: inner,
                fields,
            } = variant_info;

            let [field_info] = &fields[..] else {
                continue;
            };

            let FieldInfo {
                ident: inner_field,
                ty: inner_ty,
            } = field_info;

            let expression = match inner_field {
                Some(inner_field) => quote! { Self::#inner { #inner_field: inner} },
                None => quote! { Self::#inner(inner) },
            };

            impls.push(quote! {
                impl ::core::convert::From<#inner_ty> for #outer {
                    fn from(inner: #inner_ty) -> Self {
                        #expression
                    }
                }
            });
        }

        Ok(quote! {
            #(#impls)*
        })
    }

    pub fn derive_try_into(&self) -> Result<TokenStream2, syn::Error> {
        let outer = &self.ident;
        let variant_infos: Vec<VariantInfo> = utils::variant_infos(&self.variants);

        let mut impls: Vec<TokenStream2> = vec![];

        for variant_info in variant_infos {
            let VariantInfo {
                ident: inner,
                fields,
            } = variant_info;

            let [field_info] = &fields[..] else {
                continue;
            };

            let FieldInfo {
                ident: inner_field,
                ty: inner_ty,
            } = field_info;

            let pattern = match inner_field {
                Some(inner_field) => quote! {
                    #outer::#inner { #inner_field: inner }
                },
                None => quote! {
                    #outer::#inner(inner)
                },
            };

            impls.push(quote! {
                impl ::core::convert::TryFrom<#outer> for #inner_ty {
                    type Error = #outer;

                    fn try_from(outer: #outer) -> Result<Self, Self::Error> {
                        match outer {
                            #pattern => Ok(inner),
                            err => Err(err)
                        }
                    }
                }
            });
        }

        Ok(quote! {
            #(#impls)*
        })
    }

    pub fn derive_from_variant(&self) -> Result<TokenStream2, syn::Error> {
        let outer = &self.ident;
        let variant_infos: Vec<VariantInfo> = utils::variant_infos(&self.variants);

        let mut impls: Vec<TokenStream2> = vec![];

        for variant_info in variant_infos {
            let VariantInfo {
                ident: inner,
                fields,
            } = variant_info;

            let [field_info] = &fields[..] else {
                continue;
            };

            let FieldInfo {
                ident: inner_field,
                ty: inner_ty,
            } = field_info;

            let expression = match inner_field {
                Some(inner_field) => quote! { Self::#inner { #inner_field: inner} },
                None => quote! { Self::#inner(inner) },
            };

            impls.push(quote! {
                impl ::enumcapsulate::FromVariant<#inner_ty> for #outer {
                    fn from_variant(inner: #inner_ty) -> Self {
                        #expression
                    }
                }
            });
        }

        Ok(quote! {
            #(#impls)*
        })
    }

    pub fn derive_as_variant_ref(&self) -> Result<TokenStream2, syn::Error> {
        let outer = &self.ident;
        let variant_infos: Vec<VariantInfo> = utils::variant_infos(&self.variants);

        let mut impls: Vec<TokenStream2> = vec![];

        for variant_info in variant_infos {
            let VariantInfo {
                ident: inner,
                fields,
            } = variant_info;

            let [field_info] = &fields[..] else {
                continue;
            };

            let FieldInfo {
                ident: inner_field,
                ty: inner_ty,
            } = field_info;

            let pattern = match inner_field {
                Some(inner_field) => quote! {
                    #outer::#inner { #inner_field: inner }
                },
                None => quote! {
                    #outer::#inner(inner)
                },
            };

            impls.push(quote! {
                impl ::enumcapsulate::AsVariantRef<#inner_ty> for #outer {
                    fn as_variant_ref(&self) -> Option<&#inner_ty> {
                        match self {
                            #pattern => Some(inner),
                            _ => None
                        }
                    }
                }
            });
        }

        Ok(quote! {
            #(#impls)*
        })
    }

    pub fn derive_as_variant_mut(&self) -> Result<TokenStream2, syn::Error> {
        let outer = &self.ident;
        let variant_infos: Vec<VariantInfo> = utils::variant_infos(&self.variants);

        let mut impls: Vec<TokenStream2> = vec![];

        for variant_info in variant_infos {
            let VariantInfo {
                ident: inner,
                fields,
            } = variant_info;

            let [field_info] = &fields[..] else {
                continue;
            };

            let FieldInfo {
                ident: inner_field,
                ty: inner_ty,
            } = field_info;

            let pattern = match inner_field {
                Some(inner_field) => quote! {
                    #outer::#inner { #inner_field: inner }
                },
                None => quote! {
                    #outer::#inner(inner)
                },
            };

            impls.push(quote! {
                impl ::enumcapsulate::AsVariantMut<#inner_ty> for #outer {
                    fn as_variant_mut(&mut self) -> Option<&mut #inner_ty> {
                        match self {
                            #pattern => Some(inner),
                            _ => None
                        }
                    }
                }
            });
        }

        Ok(quote! {
            #(#impls)*
        })
    }

    pub fn derive_into_variant(&self) -> Result<TokenStream2, syn::Error> {
        let outer = &self.ident;
        let variant_infos: Vec<VariantInfo> = utils::variant_infos(&self.variants);

        let mut impls: Vec<TokenStream2> = vec![];

        for variant_info in variant_infos {
            let VariantInfo {
                ident: inner,
                fields,
            } = variant_info;

            let [field_info] = &fields[..] else {
                continue;
            };

            let FieldInfo {
                ident: inner_field,
                ty: inner_ty,
            } = field_info;

            let pattern = match inner_field {
                Some(inner_field) => quote! {
                    #outer::#inner { #inner_field: inner }
                },
                None => quote! {
                    #outer::#inner(inner)
                },
            };

            impls.push(quote! {
                impl ::enumcapsulate::IntoVariant<#inner_ty> for #outer {
                    fn into_variant(self) -> Result<#inner_ty, Self> {
                        match self {
                            #pattern => Ok(inner),
                            err => Err(err)
                        }
                    }
                }
            });
        }

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
        let variant_infos: Vec<VariantInfo> = utils::variant_infos(&self.variants);

        let mut match_arms: Vec<TokenStream2> = vec![];

        for variant_info in variant_infos {
            let VariantInfo {
                ident: inner,
                fields,
            } = variant_info;

            let [field_info] = &fields[..] else {
                continue;
            };

            let FieldInfo {
                ident: inner_field,
                ty: _,
            } = field_info;

            let pattern = match inner_field {
                Some(inner_field) => quote! {
                    #outer::#inner { #inner_field: inner }
                },
                None => quote! {
                    #outer::#inner(inner)
                },
            };

            match_arms.push(quote! {
                #pattern => type_id_of_val(inner) == type_id,
            });
        }

        Ok(quote! {
            impl ::enumcapsulate::IsVariant for #outer {
                fn is_variant<T>(&self) -> bool
                where
                    T: 'static + ?Sized
                {
                    use ::std::any::TypeId;

                    #[inline]
                    pub fn type_id_of_val<T: 'static + ?Sized>(_val: &T) -> TypeId {
                        TypeId::of::<T>()
                    }

                    let type_id = TypeId::of::<T>();

                    match self {
                        #(#match_arms)*
                        _ => false,
                    }
                }
            }
        })
    }

    pub fn derive_variant_discriminant(&self) -> Result<TokenStream2, syn::Error> {
        let outer = &self.ident;
        let discriminant_ident = quote::format_ident!("{outer}Discriminant");

        let mut discriminant_variants: Vec<TokenStream2> = vec![];

        for variant in &self.variants {
            let variant_ident = &variant.ident;

            discriminant_variants.push(quote! {
                #variant_ident,
            });
        }

        let discriminant_enum = quote! {
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
            pub enum #discriminant_ident {
                #(#discriminant_variants)*
            }
        };

        let mut match_arms: Vec<TokenStream2> = vec![];

        for variant in &self.variants {
            let inner = &variant.ident;

            let pattern = match &variant.fields {
                Fields::Named(_) => quote! {
                    #outer::#inner { .. }
                },
                Fields::Unnamed(_) => quote! {
                    #outer::#inner(..)
                },
                Fields::Unit => quote! {
                    #outer::#inner
                },
            };

            match_arms.push(quote! {
                #pattern => #discriminant_ident::#inner,
            });
        }

        Ok(quote! {
            #discriminant_enum

            impl ::enumcapsulate::VariantDiscriminant for #outer {
                type Discriminant = #discriminant_ident;

                fn variant_discriminant(&self) -> Self::Discriminant {
                    match self {
                        #(#match_arms)*
                        _ => unreachable!(),
                    }
                }
            }
        })
    }
}
