use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_quote_spanned, visit::Visit as _, Fields, Type, Variant};

use crate::*;

pub(crate) struct EnumDeriver {
    item: syn::ItemEnum,
}

impl From<syn::ItemEnum> for EnumDeriver {
    fn from(item: syn::ItemEnum) -> Self {
        Self { item }
    }
}

impl EnumDeriver {
    fn variants(&self) -> Vec<&Variant> {
        self.item.variants.iter().collect()
    }

    pub fn derive_from(&self) -> Result<TokenStream2, syn::Error> {
        const DERIVE_MACRO_NAME: &str = macro_name::FROM;

        let enum_ident = &self.item.ident;

        let _enum_config = EnumConfig::from_enum(&self.item)?;

        let outer = enum_ident;
        let outer_ty: Type = parse_quote_spanned! { outer.span() => #outer };

        let (impl_generics, type_generics, where_clause) = self.item.generics.split_for_impl();

        let mut impls: Vec<TokenStream2> = vec![];

        for variant in self.variants() {
            let variant_ident = &variant.ident;
            let inner = variant_ident;

            let variant_config = VariantConfig::from_variant(variant)?;

            if let Some(config) = variant_config.exclude() {
                if config.is_excluded(DERIVE_MACRO_NAME) {
                    continue;
                }
            }

            if variant.fields.is_empty() {
                continue;
            }

            let selection_index = variant_config.position_of_selected_field(&variant.fields)?;

            let fields: Vec<_> = variant.fields.iter().collect();
            let inner_field = fields[selection_index];
            let inner_ty = &inner_field.ty;

            if self.uses_generic_const_or_type(inner_ty) {
                continue;
            }

            let field_expressions: Vec<_> = fields
                .iter()
                .enumerate()
                .map(|(field_index, &field)| {
                    let expr = if field_index == selection_index {
                        quote! { inner }
                    } else {
                        quote! { Default::default() }
                    };
                    match &field.ident {
                        Some(field) => quote! { #field: #expr },
                        None => quote! { #expr },
                    }
                })
                .collect();

            let expression = match &variant.fields {
                Fields::Named(_) => {
                    quote! { Self::#inner { #(#field_expressions),* } }
                }
                Fields::Unnamed(_) => {
                    quote! { Self::#inner(#(#field_expressions),*) }
                }
                Fields::Unit => continue,
            };

            impls.push(quote! {
                impl #impl_generics ::core::convert::From<#inner_ty> for #outer_ty #type_generics #where_clause {
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
        const DERIVE_MACRO_NAME: &str = macro_name::TRY_INTO;

        let enum_ident = &self.item.ident;

        let _enum_config = EnumConfig::from_enum(&self.item)?;

        let outer = enum_ident;
        let outer_ty: Type = parse_quote_spanned! { outer.span() => #outer };

        let (impl_generics, type_generics, where_clause) = self.item.generics.split_for_impl();

        let mut impls: Vec<TokenStream2> = vec![];

        for variant in self.variants() {
            let variant_ident = &variant.ident;
            let inner = variant_ident;

            let variant_config = VariantConfig::from_variant(variant)?;

            if let Some(config) = variant_config.exclude() {
                if config.is_excluded(DERIVE_MACRO_NAME) {
                    continue;
                }
            }

            if variant.fields.is_empty() {
                continue;
            }

            let selection_index = variant_config.position_of_selected_field(&variant.fields)?;

            let fields: Vec<_> = variant.fields.iter().collect();
            let inner_field = fields[selection_index];
            let inner_ident = inner_field.ident.as_ref();
            let inner_ty = &inner_field.ty;

            if self.uses_generic_const_or_type(inner_ty) {
                continue;
            }

            let pattern = match &variant.fields {
                Fields::Named(_) => {
                    let field = inner_ident;
                    quote! { #outer_ty::#inner { #field: inner, .. } }
                }
                Fields::Unnamed(_) => {
                    let underscores = (0..selection_index).map(|_| {
                        quote! { _, }
                    });
                    quote! { #outer_ty::#inner(#(#underscores)* inner, ..) }
                }
                Fields::Unit => continue,
            };

            impls.push(quote! {
                impl #impl_generics ::core::convert::TryFrom<#outer_ty #type_generics> for #inner_ty #where_clause {
                    type Error = #outer_ty #type_generics;

                    fn try_from(outer: #outer_ty #type_generics) -> Result<Self, Self::Error> {
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
        const DERIVE_MACRO_NAME: &str = macro_name::FROM_VARIANT;

        let enum_ident = &self.item.ident;

        let _enum_config = EnumConfig::from_enum(&self.item)?;

        let outer = enum_ident;
        let outer_ty: Type = parse_quote_spanned! { outer.span() => #outer };

        let (impl_generics, type_generics, where_clause) = self.item.generics.split_for_impl();

        let mut impls: Vec<TokenStream2> = vec![];

        for variant in self.variants() {
            let variant_ident = &variant.ident;
            let inner = variant_ident;

            let variant_config = VariantConfig::from_variant(variant)?;

            if let Some(config) = variant_config.exclude() {
                if config.is_excluded(DERIVE_MACRO_NAME) {
                    continue;
                }
            }

            if variant.fields.is_empty() {
                continue;
            }

            let selection_index = variant_config.position_of_selected_field(&variant.fields)?;

            let fields: Vec<_> = variant.fields.iter().collect();
            let inner_field = fields[selection_index];
            let inner_ty = &inner_field.ty;

            if self.uses_generic_const_or_type(inner_ty) {
                continue;
            }

            let field_expressions: Vec<_> = fields
                .iter()
                .enumerate()
                .map(|(field_index, &field)| {
                    let expr = if field_index == selection_index {
                        quote! { inner }
                    } else {
                        quote! { Default::default() }
                    };
                    match &field.ident {
                        Some(field) => quote! { #field: #expr },
                        None => quote! { #expr },
                    }
                })
                .collect();

            let expression = match &variant.fields {
                Fields::Named(_) => {
                    quote! { Self::#inner { #(#field_expressions),* } }
                }
                Fields::Unnamed(_) => {
                    quote! { Self::#inner(#(#field_expressions),*) }
                }
                Fields::Unit => continue,
            };

            impls.push(quote! {
                impl #impl_generics ::enumcapsulate::FromVariant<#inner_ty> for #outer_ty #type_generics #where_clause {
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

    pub fn derive_as_variant(&self) -> Result<TokenStream2, syn::Error> {
        const DERIVE_MACRO_NAME: &str = macro_name::AS_VARIANT;

        let enum_ident = &self.item.ident;

        let _enum_config = EnumConfig::from_enum(&self.item)?;

        let outer = enum_ident;
        let outer_ty: Type = parse_quote_spanned! { outer.span() => #outer };

        let (impl_generics, type_generics, where_clause) = self.item.generics.split_for_impl();

        let mut impls: Vec<TokenStream2> = vec![];

        for variant in self.variants() {
            let variant_ident = &variant.ident;
            let inner = variant_ident;

            let variant_config = VariantConfig::from_variant(variant)?;

            if let Some(config) = variant_config.exclude() {
                if config.is_excluded(DERIVE_MACRO_NAME) {
                    continue;
                }
            }

            if variant.fields.is_empty() {
                continue;
            }

            let selection_index = variant_config.position_of_selected_field(&variant.fields)?;

            let fields: Vec<_> = variant.fields.iter().collect();
            let inner_field = fields[selection_index];
            let inner_ident = inner_field.ident.as_ref();
            let inner_ty = &inner_field.ty;

            if self.uses_generic_const_or_type(inner_ty) {
                continue;
            }

            let pattern = match &variant.fields {
                Fields::Named(_) => {
                    let field = inner_ident;
                    quote! { #outer_ty::#inner { #field: inner, .. } }
                }
                Fields::Unnamed(_) => {
                    let underscores = (0..selection_index).map(|_| {
                        quote! { _, }
                    });
                    quote! { #outer_ty::#inner(#(#underscores)* inner, ..) }
                }
                Fields::Unit => continue,
            };

            let where_clause = match where_clause {
                Some(where_clause) => quote! { #where_clause #inner_ty: Clone },
                None => quote! { where #inner_ty: Clone },
            };

            impls.push(quote! {
                impl #impl_generics ::enumcapsulate::AsVariant<#inner_ty> for #outer_ty #type_generics #where_clause {
                    fn as_variant(&self) -> Option<#inner_ty> {
                        match self {
                            #pattern => Some(inner.clone()),
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

    pub fn derive_as_variant_ref(&self) -> Result<TokenStream2, syn::Error> {
        const DERIVE_MACRO_NAME: &str = macro_name::AS_VARIANT_REF;

        let enum_ident = &self.item.ident;

        let _enum_config = EnumConfig::from_enum(&self.item)?;

        let outer = enum_ident;
        let outer_ty: Type = parse_quote_spanned! { outer.span() => #outer };

        let (impl_generics, type_generics, where_clause) = self.item.generics.split_for_impl();

        let mut impls: Vec<TokenStream2> = vec![];

        for variant in self.variants() {
            let variant_ident = &variant.ident;
            let inner = variant_ident;

            let variant_config = VariantConfig::from_variant(variant)?;

            if let Some(config) = variant_config.exclude() {
                if config.is_excluded(DERIVE_MACRO_NAME) {
                    continue;
                }
            }

            if variant.fields.is_empty() {
                continue;
            }

            let selection_index = variant_config.position_of_selected_field(&variant.fields)?;

            let fields: Vec<_> = variant.fields.iter().collect();
            let inner_field = fields[selection_index];
            let inner_ident = inner_field.ident.as_ref();
            let inner_ty = &inner_field.ty;

            if self.uses_generic_const_or_type(inner_ty) {
                continue;
            }

            let pattern = match &variant.fields {
                Fields::Named(_) => {
                    let field = inner_ident;
                    quote! { #outer_ty::#inner { #field: inner, .. } }
                }
                Fields::Unnamed(_) => {
                    let underscores = (0..selection_index).map(|_| {
                        quote! { _, }
                    });
                    quote! { #outer_ty::#inner(#(#underscores)* inner, ..) }
                }
                Fields::Unit => continue,
            };

            impls.push(quote! {
                impl #impl_generics ::enumcapsulate::AsVariantRef<#inner_ty> for #outer_ty #type_generics #where_clause {
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
        const DERIVE_MACRO_NAME: &str = macro_name::AS_VARIANT_MUT;

        let enum_ident = &self.item.ident;

        let _enum_config = EnumConfig::from_enum(&self.item)?;

        let outer = enum_ident;
        let outer_ty: Type = parse_quote_spanned! { outer.span() => #outer };

        let (impl_generics, type_generics, where_clause) = self.item.generics.split_for_impl();

        let mut impls: Vec<TokenStream2> = vec![];

        for variant in self.variants() {
            let variant_ident = &variant.ident;
            let inner = variant_ident;

            let variant_config = VariantConfig::from_variant(variant)?;

            if let Some(config) = variant_config.exclude() {
                if config.is_excluded(DERIVE_MACRO_NAME) {
                    continue;
                }
            }

            if variant.fields.is_empty() {
                continue;
            }

            let selection_index = variant_config.position_of_selected_field(&variant.fields)?;

            let fields: Vec<_> = variant.fields.iter().collect();
            let inner_field = fields[selection_index];
            let inner_ident = inner_field.ident.as_ref();
            let inner_ty = &inner_field.ty;

            if self.uses_generic_const_or_type(inner_ty) {
                continue;
            }

            let pattern = match &variant.fields {
                Fields::Named(_) => {
                    let field = inner_ident;
                    quote! { #outer_ty::#inner { #field: inner, .. } }
                }
                Fields::Unnamed(_) => {
                    let underscores = (0..selection_index).map(|_| {
                        quote! { _, }
                    });
                    quote! { #outer_ty::#inner(#(#underscores)* inner, ..) }
                }
                Fields::Unit => continue,
            };

            impls.push(quote! {
                impl #impl_generics ::enumcapsulate::AsVariantMut<#inner_ty> for #outer_ty #type_generics #where_clause {
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
        const DERIVE_MACRO_NAME: &str = macro_name::INTO_VARIANT;

        let enum_ident = &self.item.ident;

        let _enum_config = EnumConfig::from_enum(&self.item)?;

        let outer = enum_ident;
        let outer_ty: Type = parse_quote_spanned! { outer.span() => #outer };

        let (impl_generics, type_generics, where_clause) = self.item.generics.split_for_impl();

        let mut impls: Vec<TokenStream2> = vec![];

        for variant in self.variants() {
            let variant_ident = &variant.ident;
            let inner = variant_ident;

            let variant_config = VariantConfig::from_variant(variant)?;

            if let Some(config) = variant_config.exclude() {
                if config.is_excluded(DERIVE_MACRO_NAME) {
                    continue;
                }
            }

            if variant.fields.is_empty() {
                continue;
            }

            let selection_index = variant_config.position_of_selected_field(&variant.fields)?;

            let fields: Vec<_> = variant.fields.iter().collect();
            let inner_field = fields[selection_index];
            let inner_ident = inner_field.ident.as_ref();
            let inner_ty = &inner_field.ty;

            if self.uses_generic_const_or_type(inner_ty) {
                continue;
            }

            let pattern = match &variant.fields {
                Fields::Named(_) => {
                    let field = inner_ident;
                    quote! { #outer_ty::#inner { #field: inner, .. } }
                }
                Fields::Unnamed(_) => {
                    let underscores = (0..selection_index).map(|_| {
                        quote! { _, }
                    });
                    quote! { #outer_ty::#inner(#(#underscores)* inner, ..) }
                }
                Fields::Unit => continue,
            };

            impls.push(quote! {
                impl #impl_generics ::enumcapsulate::IntoVariant<#inner_ty> for #outer_ty #type_generics #where_clause {
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
        let enum_ident = &self.item.ident;

        let _enum_config = EnumConfig::from_enum(&self.item)?;

        let outer = enum_ident;
        let outer_ty: Type = parse_quote_spanned! { outer.span() => #outer };

        let (impl_generics, type_generics, where_clause) = self.item.generics.split_for_impl();

        let tokens = quote! {
            impl #impl_generics ::enumcapsulate::VariantDowncast for #outer_ty #type_generics #where_clause {}
        };

        Ok(tokens)
    }

    pub fn derive_variant_discriminant(&self) -> Result<TokenStream2, syn::Error> {
        let enum_ident = &self.item.ident;

        let enum_config = EnumConfig::from_enum(&self.item)?;

        let visibility = &self.item.vis;

        let mut discriminant_enum_ident = quote::format_ident!("{enum_ident}Discriminant");
        let mut repr_attr: Option<TokenStream2> = None;

        if let Some(discriminant_config) = enum_config.discriminant() {
            if let Some(ident) = discriminant_config.ident() {
                discriminant_enum_ident = ident.clone();
            }

            if let Some(ty) = discriminant_config.repr() {
                repr_attr = Some(quote! {
                    #[repr(#ty)]
                });
            }
        }

        let (impl_generics, type_generics, where_clause) = self.item.generics.split_for_impl();

        let variants = self.variants();

        let mut discriminant_variants: Vec<TokenStream2> = vec![];
        let mut match_arms: Vec<TokenStream2> = vec![];

        for variant in &variants {
            let variant_config = VariantConfig::from_variant(variant)?;

            let variant_ident: &syn::Ident = &variant.ident;

            let mut discriminant_variant_ident: syn::Ident = variant_ident.clone();
            let mut discriminant_variant_expr: Option<&syn::Expr> =
                variant.discriminant.as_ref().map(|(_, expr)| expr);

            if let Some(discriminant_config) = variant_config.discriminant() {
                if let Some(ident) = discriminant_config.ident() {
                    discriminant_variant_ident = ident.clone();
                }

                if let Some(expr) = discriminant_config.expr() {
                    discriminant_variant_expr = Some(expr);
                }
            }

            let variant_discriminant = discriminant_variant_expr.map(|expr| {
                quote! { = #expr }
            });

            discriminant_variants.push(quote! {
                #discriminant_variant_ident #variant_discriminant,
            });

            let pattern = match &variant.fields {
                Fields::Named(_) => quote! {
                    #enum_ident::#variant_ident { .. }
                },
                Fields::Unnamed(_) => quote! {
                    #enum_ident::#variant_ident(..)
                },
                Fields::Unit => quote! {
                    #enum_ident::#variant_ident
                },
            };

            match_arms.push(quote! {
                #pattern => #discriminant_enum_ident::#discriminant_variant_ident,
            });
        }

        let discriminant_enum = quote! {
            #repr_attr
            #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
            #visibility enum #discriminant_enum_ident {
                #(#discriminant_variants)*
            }
        };

        Ok(quote! {
            #discriminant_enum

            impl #impl_generics ::enumcapsulate::VariantDiscriminant for #enum_ident #type_generics #where_clause {
                type Discriminant = #discriminant_enum_ident;

                fn variant_discriminant(&self) -> Self::Discriminant {
                    match self {
                        #(#match_arms)*
                        _ => unreachable!(),
                    }
                }
            }
        })
    }

    pub fn derive_encapsulate(&self) -> Result<TokenStream2, syn::Error> {
        let enum_config = EnumConfig::from_enum(&self.item)?;

        let from = enum_config
            .is_included(macro_name::FROM)
            .then_some(self.derive_from()?);
        let try_into = enum_config
            .is_included(macro_name::TRY_INTO)
            .then_some(self.derive_try_into()?);
        let from_variant = enum_config
            .is_included(macro_name::FROM_VARIANT)
            .then_some(self.derive_from_variant()?);
        let as_variant = enum_config
            .is_included(macro_name::AS_VARIANT)
            .then_some(self.derive_as_variant()?);
        let as_variant_ref = enum_config
            .is_included(macro_name::AS_VARIANT_REF)
            .then_some(self.derive_as_variant_ref()?);
        let as_variant_mut = enum_config
            .is_included(macro_name::AS_VARIANT_MUT)
            .then_some(self.derive_as_variant_mut()?);
        let into_variant = enum_config
            .is_included(macro_name::INTO_VARIANT)
            .then_some(self.derive_into_variant()?);
        let variant_downcast = enum_config
            .is_included(macro_name::VARIANT_DOWNCAST)
            .then_some(self.derive_variant_downcast()?);

        Ok(quote::quote! {
            #from
            #try_into
            #from_variant
            #as_variant
            #as_variant_ref
            #as_variant_mut
            #into_variant
            #variant_downcast
        })
    }

    fn uses_generic_const_or_type(&self, ty: &syn::Type) -> bool {
        let mut visitor = TypeVisitor::new(&self.item.generics);

        visitor.visit_type(ty);

        visitor.uses_const_or_type_param()
    }
}
