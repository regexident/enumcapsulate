use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{Error, Field, Fields, Ident, Type, Variant};

pub(crate) fn single_field_of_variant(variant: &Variant) -> Result<&Field, Error> {
    let Fields::Unnamed(field) = &variant.fields else {
        return Err(Error::new(
            variant.ident.span(),
            "Only enums with tuple variants can use this derive",
        ));
    };

    if field.unnamed.len() != 1 {
        return Err(Error::new(
            variant.ident.span(),
            "Only enums with single-element tuple variants can use this derive",
        ));
    }

    Ok(field.unnamed.first().unwrap())
}

pub(crate) struct VariantInfo<'a> {
    pub ident: &'a Ident,
    pub inner_ty: &'a Type,
}

pub(crate) fn infos_per_newtype_variant<'a, I>(variants: I) -> Vec<VariantInfo<'a>>
where
    I: IntoIterator<Item = &'a Variant>,
{
    variants
        .into_iter()
        .map(|variant| {
            let field = single_field_of_variant(variant).unwrap();
            let inner_ty = &field.ty;
            let ident = &variant.ident;

            VariantInfo { ident, inner_ty }
        })
        .collect::<Vec<VariantInfo>>()
}

pub(crate) fn tokenstream<F>(f: F) -> TokenStream
where
    F: FnOnce() -> Result<TokenStream2, syn::Error>,
{
    match f() {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    }
    .into()
}
