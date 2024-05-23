use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{Fields, Ident, Type, Variant};

pub(crate) struct FieldInfo<'a> {
    pub ident: Option<Ident>,
    pub ty: &'a Type,
}

pub(crate) struct VariantInfo<'a> {
    pub ident: Ident,
    pub fields: Vec<FieldInfo<'a>>,
}

pub(crate) fn variant_infos<'a, I>(variants: I) -> Vec<VariantInfo<'a>>
where
    I: IntoIterator<Item = &'a Variant>,
{
    let mut info = vec![];

    for variant in variants {
        let ident = variant.ident.clone();
        let fields = match &variant.fields {
            Fields::Named(fields) => Vec::from_iter(fields.named.iter()),
            Fields::Unnamed(fields) => Vec::from_iter(fields.unnamed.iter()),
            Fields::Unit => vec![],
        }
        .into_iter()
        .map(|field| FieldInfo {
            ident: field.ident.clone(),
            ty: &field.ty,
        })
        .collect();
        info.push(VariantInfo { ident, fields });
    }

    info
}

#[track_caller]
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
