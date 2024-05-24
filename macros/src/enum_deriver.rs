use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_quote_spanned, DataEnum, DeriveInput, Fields, Type, Variant};

use crate::utils::{self, FieldInfo, VariantInfo};

pub(crate) struct EnumDeriver {
    input: DeriveInput,
}

impl From<DeriveInput> for EnumDeriver {
    fn from(input: DeriveInput) -> Self {
        Self { input }
    }
}

impl EnumDeriver {
    fn variants(&self) -> Result<Vec<&Variant>, syn::Error> {
        let syn::Data::Enum(DataEnum { variants, .. }) = &self.input.data else {
            return Err(syn::Error::new(
                self.input.ident.span(),
                "Only enums can use this derive",
            ));
        };

        Ok(Vec::from_iter(variants.iter()))
    }

    pub fn derive_from(&self) -> Result<TokenStream2, syn::Error> {
        let outer = &self.input.ident;
        let outer_ty: Type = parse_quote_spanned! { outer.span() => #outer };

        let variants = self.variants()?;
        let variant_infos: Vec<VariantInfo> = utils::variant_infos(variants)?;

        let mut impls: Vec<TokenStream2> = vec![];

        for variant_info in variant_infos {
            let VariantInfo {
                ident: inner,
                attrs,
                fields,
            } = variant_info;

            if attrs.exclude.is_some() {
                continue;
            }

            let field_info = if let [field_info] = &fields[..] {
                Some(field_info)
            } else {
                None
            };

            let Some(FieldInfo {
                ident: inner_field,
                ty: inner_ty,
            }) = field_info
            else {
                continue;
            };

            let expression = match inner_field {
                Some(inner_field) => quote! { Self::#inner { #inner_field: inner} },
                None => quote! { Self::#inner(inner) },
            };

            impls.push(quote! {
                impl ::core::convert::From<#inner_ty> for #outer_ty {
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
        let outer = &self.input.ident;
        let outer_ty: Type = parse_quote_spanned! { outer.span() => #outer };

        let variants = self.variants()?;
        let variant_infos: Vec<VariantInfo> = utils::variant_infos(variants)?;

        let mut impls: Vec<TokenStream2> = vec![];

        for variant_info in variant_infos {
            let VariantInfo {
                ident: inner,
                attrs,
                fields,
            } = variant_info;

            if attrs.exclude.is_some() {
                continue;
            }

            let field_info = if let Some(include_info) = &attrs.include {
                let index = include_info.field.index;
                Some((index, &fields[index]))
            } else if let [field_info] = &fields[..] {
                Some((0, field_info))
            } else {
                None
            };

            let Some((
                field_index,
                FieldInfo {
                    ident: inner_field,
                    ty: inner_ty,
                },
            )) = field_info
            else {
                continue;
            };

            let pattern = match inner_field {
                Some(inner_field) => quote! {
                    #outer_ty::#inner { #inner_field: inner, .. }
                },
                None => {
                    let underscores = (0..field_index).map(|_| quote! { _, });
                    quote! {
                        #outer_ty::#inner(#(#underscores)* inner, ..)
                    }
                }
            };

            impls.push(quote! {
                impl ::core::convert::TryFrom<#outer_ty> for #inner_ty {
                    type Error = #outer_ty;

                    fn try_from(outer: #outer_ty) -> Result<Self, Self::Error> {
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
        let outer = &self.input.ident;
        let outer_ty: Type = parse_quote_spanned! { outer.span() => #outer };

        let variants = self.variants()?;
        let variant_infos: Vec<VariantInfo> = utils::variant_infos(variants)?;

        let mut impls: Vec<TokenStream2> = vec![];

        for variant_info in variant_infos {
            let VariantInfo {
                ident: inner,
                attrs,
                fields,
            } = variant_info;

            if attrs.exclude.is_some() {
                continue;
            }

            let field_info = if let [field_info] = &fields[..] {
                Some(field_info)
            } else {
                None
            };

            let Some(FieldInfo {
                ident: inner_field,
                ty: inner_ty,
            }) = field_info
            else {
                continue;
            };

            let expression = match inner_field {
                Some(inner_field) => quote! { Self::#inner { #inner_field: inner} },
                None => quote! { Self::#inner(inner) },
            };

            impls.push(quote! {
                impl ::enumcapsulate::FromVariant<#inner_ty> for #outer_ty {
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
        let outer = &self.input.ident;
        let outer_ty: Type = parse_quote_spanned! { outer.span() => #outer };

        let variants = self.variants()?;
        let variant_infos: Vec<VariantInfo> = utils::variant_infos(variants)?;

        let mut impls: Vec<TokenStream2> = vec![];

        for variant_info in variant_infos {
            let VariantInfo {
                ident: inner,
                attrs,
                fields,
            } = variant_info;

            if attrs.exclude.is_some() {
                continue;
            }

            let field_info = if let Some(include_info) = &attrs.include {
                let index = include_info.field.index;
                Some((index, &fields[index]))
            } else if let [field_info] = &fields[..] {
                Some((0, field_info))
            } else {
                None
            };

            let Some((
                field_index,
                FieldInfo {
                    ident: inner_field,
                    ty: inner_ty,
                },
            )) = field_info
            else {
                continue;
            };

            let pattern = match inner_field {
                Some(inner_field) => quote! {
                    #outer_ty::#inner { #inner_field: inner, .. }
                },
                None => {
                    let underscores = (0..field_index).map(|_| quote! { _, });
                    quote! {
                        #outer_ty::#inner(#(#underscores)* inner, ..)
                    }
                }
            };

            impls.push(quote! {
                impl ::enumcapsulate::AsVariantRef<#inner_ty> for #outer_ty {
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
        let outer = &self.input.ident;
        let outer_ty: Type = parse_quote_spanned! { outer.span() => #outer };

        let variants = self.variants()?;
        let variant_infos: Vec<VariantInfo> = utils::variant_infos(variants)?;

        let mut impls: Vec<TokenStream2> = vec![];

        for variant_info in variant_infos {
            let VariantInfo {
                ident: inner,
                attrs,
                fields,
            } = variant_info;

            if attrs.exclude.is_some() {
                continue;
            }

            let field_info = if let Some(include_info) = &attrs.include {
                let index = include_info.field.index;
                Some((index, &fields[index]))
            } else if let [field_info] = &fields[..] {
                Some((0, field_info))
            } else {
                None
            };

            let Some((
                field_index,
                FieldInfo {
                    ident: inner_field,
                    ty: inner_ty,
                },
            )) = field_info
            else {
                continue;
            };

            let pattern = match inner_field {
                Some(inner_field) => quote! {
                    #outer_ty::#inner { #inner_field: inner, .. }
                },
                None => {
                    let underscores = (0..field_index).map(|_| quote! { _, });
                    quote! {
                        #outer_ty::#inner(#(#underscores)* inner, ..)
                    }
                }
            };

            impls.push(quote! {
                impl ::enumcapsulate::AsVariantMut<#inner_ty> for #outer_ty {
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
        let outer = &self.input.ident;
        let outer_ty: Type = parse_quote_spanned! { outer.span() => #outer };

        let variants = self.variants()?;
        let variant_infos: Vec<VariantInfo> = utils::variant_infos(variants)?;

        let mut impls: Vec<TokenStream2> = vec![];

        for variant_info in variant_infos {
            let VariantInfo {
                ident: inner,
                attrs,
                fields,
            } = variant_info;

            if attrs.exclude.is_some() {
                continue;
            }

            let field_info = if let Some(include_info) = &attrs.include {
                let index = include_info.field.index;
                Some((index, &fields[index]))
            } else if let [field_info] = &fields[..] {
                Some((0, field_info))
            } else {
                None
            };

            let Some((
                field_index,
                FieldInfo {
                    ident: inner_field,
                    ty: inner_ty,
                },
            )) = field_info
            else {
                continue;
            };

            let pattern = match inner_field {
                Some(inner_field) => quote! {
                    #outer_ty::#inner { #inner_field: inner, .. }
                },
                None => {
                    let underscores = (0..field_index).map(|_| quote! { _, });
                    quote! {
                        #outer_ty::#inner(#(#underscores)* inner, ..)
                    }
                }
            };

            impls.push(quote! {
                impl ::enumcapsulate::IntoVariant<#inner_ty> for #outer_ty {
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
        let outer = &self.input.ident;
        let outer_ty: Type = parse_quote_spanned! { outer.span() => #outer };

        let tokens = quote! {
            impl ::enumcapsulate::VariantDowncast for #outer_ty {}
        };

        Ok(tokens)
    }

    pub fn derive_is_variant(&self) -> Result<TokenStream2, syn::Error> {
        let outer = &self.input.ident;
        let outer_ty: Type = parse_quote_spanned! { outer.span() => #outer };

        let variants = self.variants()?;
        let variant_infos: Vec<VariantInfo> = utils::variant_infos(variants)?;

        let mut match_arms: Vec<TokenStream2> = vec![];

        for variant_info in variant_infos {
            let VariantInfo {
                ident: inner,
                attrs,
                fields,
            } = variant_info;

            if attrs.exclude.is_some() {
                continue;
            }

            let field_info = if let Some(include_info) = &attrs.include {
                let index = include_info.field.index;
                Some((index, &fields[index]))
            } else if let [field_info] = &fields[..] {
                Some((0, field_info))
            } else {
                None
            };

            let Some((
                field_index,
                FieldInfo {
                    ident: inner_field,
                    ty: _,
                },
            )) = field_info
            else {
                continue;
            };

            let pattern = match inner_field {
                Some(inner_field) => quote! {
                    #outer_ty::#inner { #inner_field: inner, .. }
                },
                None => {
                    let underscores = (0..field_index).map(|_| quote! { _, });
                    quote! {
                        #outer_ty::#inner(#(#underscores)* inner, ..)
                    }
                }
            };

            match_arms.push(quote! {
                #pattern => type_id_of_val(inner) == type_id,
            });
        }

        Ok(quote! {
            impl ::enumcapsulate::IsVariant for #outer_ty {
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
        let outer = &self.input.ident;
        let outer_ty: Type = parse_quote_spanned! { outer.span() => #outer };

        let variants = self.variants()?;

        let discriminant_ident = quote::format_ident!("{outer}Discriminant");

        let mut discriminant_variants: Vec<TokenStream2> = vec![];

        for variant in &variants {
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

        for variant in variants {
            let inner = &variant.ident;

            let pattern = match &variant.fields {
                Fields::Named(_) => quote! {
                    #outer_ty::#inner { .. }
                },
                Fields::Unnamed(_) => quote! {
                    #outer_ty::#inner(..)
                },
                Fields::Unit => quote! {
                    #outer_ty::#inner
                },
            };

            match_arms.push(quote! {
                #pattern => #discriminant_ident::#inner,
            });
        }

        Ok(quote! {
            #discriminant_enum

            impl ::enumcapsulate::VariantDiscriminant for #outer_ty {
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
