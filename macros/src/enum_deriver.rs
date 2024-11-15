use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_quote_spanned, visit::Visit as _, DataEnum, DeriveInput, Fields, Type, Variant};

use crate::{
    config_for_enum_with_attrs, config_for_variant, macro_name, position_of_selected_field,
    TypeVisitor,
};

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
        const DERIVE_MACRO_NAME: &str = macro_name::FROM;

        let derive_input = &self.input;
        let enum_ident = &derive_input.ident;

        let enum_config = config_for_enum_with_attrs(&self.input.attrs)?;

        if enum_config.is_excluded(DERIVE_MACRO_NAME) {
            return Ok(TokenStream2::default());
        }

        let outer = enum_ident;
        let outer_ty: Type = parse_quote_spanned! { outer.span() => #outer };

        let (impl_generics, type_generics, where_clause) = self.input.generics.split_for_impl();

        let mut impls: Vec<TokenStream2> = vec![];

        for variant in self.variants()? {
            let variant_ident = &variant.ident;
            let inner = variant_ident;

            let variant_config = config_for_variant(variant)?;

            if variant_config.is_excluded(DERIVE_MACRO_NAME, &enum_config) {
                continue;
            }

            let Some(selection_index) =
                position_of_selected_field(&variant.fields, variant_config.field.as_ref())?
            else {
                continue;
            };

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

        let derive_input = &self.input;
        let enum_ident = &derive_input.ident;

        let enum_config = config_for_enum_with_attrs(&self.input.attrs)?;

        let outer = enum_ident;
        let outer_ty: Type = parse_quote_spanned! { outer.span() => #outer };

        let (impl_generics, type_generics, where_clause) = self.input.generics.split_for_impl();

        let mut impls: Vec<TokenStream2> = vec![];

        for variant in self.variants()? {
            let variant_ident = &variant.ident;
            let inner = variant_ident;

            let variant_config = config_for_variant(variant)?;

            if variant_config.is_excluded(DERIVE_MACRO_NAME, &enum_config) {
                continue;
            }

            let Some(selection_index) =
                position_of_selected_field(&variant.fields, variant_config.field.as_ref())?
            else {
                continue;
            };

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

        let derive_input = &self.input;
        let enum_ident = &derive_input.ident;

        let enum_config = config_for_enum_with_attrs(&self.input.attrs)?;

        let outer = enum_ident;
        let outer_ty: Type = parse_quote_spanned! { outer.span() => #outer };

        let (impl_generics, type_generics, where_clause) = self.input.generics.split_for_impl();

        let mut impls: Vec<TokenStream2> = vec![];

        for variant in self.variants()? {
            let variant_ident = &variant.ident;
            let inner = variant_ident;

            let variant_config = config_for_variant(variant)?;

            if variant_config.is_excluded(DERIVE_MACRO_NAME, &enum_config) {
                continue;
            }

            let Some(selection_index) =
                position_of_selected_field(&variant.fields, variant_config.field.as_ref())?
            else {
                continue;
            };

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

        let derive_input = &self.input;
        let enum_ident = &derive_input.ident;

        let enum_config = config_for_enum_with_attrs(&self.input.attrs)?;

        let outer = enum_ident;
        let outer_ty: Type = parse_quote_spanned! { outer.span() => #outer };

        let (impl_generics, type_generics, where_clause) = self.input.generics.split_for_impl();

        let mut impls: Vec<TokenStream2> = vec![];

        for variant in self.variants()? {
            let variant_ident = &variant.ident;
            let inner = variant_ident;

            let variant_config = config_for_variant(variant)?;

            if variant_config.is_excluded(DERIVE_MACRO_NAME, &enum_config) {
                continue;
            }

            let Some(selection_index) =
                position_of_selected_field(&variant.fields, variant_config.field.as_ref())?
            else {
                continue;
            };

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

        let derive_input = &self.input;
        let enum_ident = &derive_input.ident;

        let enum_config = config_for_enum_with_attrs(&derive_input.attrs)?;

        let outer = enum_ident;
        let outer_ty: Type = parse_quote_spanned! { outer.span() => #outer };

        let (impl_generics, type_generics, where_clause) = self.input.generics.split_for_impl();

        let mut impls: Vec<TokenStream2> = vec![];

        for variant in self.variants()? {
            let variant_ident = &variant.ident;
            let inner = variant_ident;

            let variant_config = config_for_variant(variant)?;

            if variant_config.is_excluded(DERIVE_MACRO_NAME, &enum_config) {
                continue;
            }

            let Some(selection_index) =
                position_of_selected_field(&variant.fields, variant_config.field.as_ref())?
            else {
                continue;
            };

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

        let derive_input = &self.input;
        let enum_ident = &derive_input.ident;

        let enum_config = config_for_enum_with_attrs(&self.input.attrs)?;

        let outer = enum_ident;
        let outer_ty: Type = parse_quote_spanned! { outer.span() => #outer };

        let (impl_generics, type_generics, where_clause) = self.input.generics.split_for_impl();

        let mut impls: Vec<TokenStream2> = vec![];

        for variant in self.variants()? {
            let variant_ident = &variant.ident;
            let inner = variant_ident;

            let variant_config = config_for_variant(variant)?;

            if variant_config.is_excluded(DERIVE_MACRO_NAME, &enum_config) {
                continue;
            }

            let Some(selection_index) =
                position_of_selected_field(&variant.fields, variant_config.field.as_ref())?
            else {
                continue;
            };

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

        let derive_input = &self.input;
        let enum_ident = &derive_input.ident;

        let enum_config = config_for_enum_with_attrs(&self.input.attrs)?;

        let outer = enum_ident;
        let outer_ty: Type = parse_quote_spanned! { outer.span() => #outer };

        let (impl_generics, type_generics, where_clause) = self.input.generics.split_for_impl();

        let mut impls: Vec<TokenStream2> = vec![];

        for variant in self.variants()? {
            let variant_ident = &variant.ident;
            let inner = variant_ident;

            let variant_config = config_for_variant(variant)?;

            if variant_config.is_excluded(DERIVE_MACRO_NAME, &enum_config) {
                continue;
            }

            let Some(selection_index) =
                position_of_selected_field(&variant.fields, variant_config.field.as_ref())?
            else {
                continue;
            };

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
        const DERIVE_MACRO_NAME: &str = macro_name::VARIANT_DOWNCAST;

        let derive_input = &self.input;
        let enum_ident = &derive_input.ident;

        let enum_config = config_for_enum_with_attrs(&self.input.attrs)?;

        if enum_config.is_excluded(DERIVE_MACRO_NAME) {
            return Ok(TokenStream2::default());
        }

        let outer = enum_ident;
        let outer_ty: Type = parse_quote_spanned! { outer.span() => #outer };

        let (impl_generics, type_generics, where_clause) = self.input.generics.split_for_impl();

        let tokens = quote! {
            impl #impl_generics ::enumcapsulate::VariantDowncast for #outer_ty #type_generics #where_clause {}
        };

        Ok(tokens)
    }

    pub fn derive_variant_discriminant(&self) -> Result<TokenStream2, syn::Error> {
        const DERIVE_MACRO_NAME: &str = macro_name::VARIANT_DISCRIMINANT;

        let derive_input = &self.input;
        let enum_ident = &derive_input.ident;

        let enum_config = config_for_enum_with_attrs(&self.input.attrs)?;

        if enum_config.is_excluded(DERIVE_MACRO_NAME) {
            return Ok(TokenStream2::default());
        }

        let outer = enum_ident;
        let outer_ty: Type = parse_quote_spanned! { outer.span() => #outer };

        let (impl_generics, type_generics, where_clause) = self.input.generics.split_for_impl();

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
            let variant_ident = &variant.ident;
            let inner = variant_ident;

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

            impl #impl_generics ::enumcapsulate::VariantDiscriminant for #outer_ty #type_generics #where_clause {
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

    fn uses_generic_const_or_type(&self, ty: &syn::Type) -> bool {
        let mut visitor = TypeVisitor::new(&self.input.generics);

        visitor.visit_type(ty);

        visitor.uses_const_or_type_param()
    }
}
