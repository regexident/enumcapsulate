use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{Error, Fields, Ident, Type, Variant};

pub(crate) struct FieldInfo<'a> {
    pub ident: Option<Ident>,
    pub ty: &'a Type,
}

#[derive(Default)]
pub(crate) struct VariantAttrs {
    pub exclude: bool,
}

pub(crate) struct VariantInfo<'a> {
    pub ident: Ident,
    pub attrs: VariantAttrs,
    pub fields: Vec<FieldInfo<'a>>,
}

const NAMESPACE_ATTR: &str = "enumcapsulate";
const EXCLUDE_ATTR: &str = "exclude";

fn ident(variant: &Variant) -> Result<Ident, Error> {
    Ok(variant.ident.clone())
}

fn attrs(variant: &Variant) -> Result<VariantAttrs, Error> {
    let mut attrs = VariantAttrs::default();

    for attr in &variant.attrs {
        if !attr.path().is_ident(NAMESPACE_ATTR) {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident(EXCLUDE_ATTR) {
                attrs.exclude = true;
                Ok(())
            } else {
                Err(meta.error("unsupported attribute"))
            }
        })?;
    }

    Ok(attrs)
}

fn fields(variant: &Variant) -> Result<Vec<FieldInfo>, Error> {
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

    Ok(fields)
}

pub(crate) fn variant_infos<'a, I>(variants: I) -> Result<Vec<VariantInfo<'a>>, Error>
where
    I: IntoIterator<Item = &'a Variant>,
{
    let mut info = vec![];

    for variant in variants {
        info.push(VariantInfo {
            ident: ident(variant)?,
            attrs: attrs(variant)?,
            fields: fields(variant)?,
        });
    }

    Ok(info)
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
