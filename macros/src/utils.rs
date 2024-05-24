use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{
    meta::ParseNestedMeta, parse::Parse, Error, Expr, Fields, Ident, Lit, Meta, MetaNameValue,
    Token, Type, Variant,
};

#[derive(Default)]
pub(crate) struct ExcludeAttr {}

#[derive(Default)]
pub(crate) struct IncludeFieldAttr {
    pub index: usize,
}

#[derive(Default)]
pub(crate) struct IncludeAttr {
    pub field: IncludeFieldAttr,
}

#[derive(Default)]
pub(crate) struct VariantAttrs {
    pub exclude: Option<ExcludeAttr>,
    pub include: Option<IncludeAttr>,
}

pub(crate) struct FieldInfo<'a> {
    pub ident: Option<Ident>,
    pub ty: &'a Type,
}

pub(crate) struct VariantInfo<'a> {
    pub ident: Ident,
    pub attrs: VariantAttrs,
    pub fields: Vec<FieldInfo<'a>>,
}

const NAMESPACE_ATTR: &str = "enumcapsulate";
const EXCLUDE_ATTR: &str = "exclude";
const INCLUDE_ATTR: &str = "include";
const FIELD_ATTR: &str = "field";

fn ident(variant: &Variant) -> Result<Ident, Error> {
    Ok(variant.ident.clone())
}

fn exclude_attr(_variant: &Variant, _meta: &ParseNestedMeta) -> Result<ExcludeAttr, Error> {
    Ok(ExcludeAttr {})
}

fn index_of_field_with_name(
    variant: &Variant,
    name: &str,
    meta: &MetaNameValue,
) -> Result<usize, Error> {
    match &variant.fields {
        Fields::Named(fields) => {
            if let Some(index) = fields
                .named
                .iter()
                .position(|field| field.ident.as_ref().unwrap() == name)
            {
                Ok(index)
            } else {
                Err(Error::new_spanned(
                    meta,
                    format!("no field named {name:?} on variant"),
                ))
            }
        }
        Fields::Unnamed(_fields) => Err(Error::new_spanned(
            meta,
            "name-based field selector (e.g. `field = \"name\"`) not supported on tuple variant",
        )),
        Fields::Unit => Err(Error::new_spanned(meta, "unit variant has no fields")),
    }
}

fn index_of_field_with_index(
    variant: &Variant,
    index: usize,
    meta: &MetaNameValue,
) -> Result<usize, Error> {
    match &variant.fields {
        Fields::Named(_fields) => Err(Error::new_spanned(
            meta,
            "index-based field selector (e.g. `field = 1`) not supported on struct variant",
        )),
        Fields::Unnamed(fields) => {
            let fields_len = fields.unnamed.len();
            if fields_len > index {
                Ok(index)
            } else {
                Err(Error::new_spanned(
                    meta,
                    format!("variant only has {fields_len} fields"),
                ))
            }
        }
        Fields::Unit => Err(Error::new_spanned(meta, "unit variant has no fields")),
    }
}

fn include_field_attr(variant: &Variant, meta: &MetaNameValue) -> Result<IncludeFieldAttr, Error> {
    match &meta.value {
        Expr::Lit(expr_lit) => match &expr_lit.lit {
            Lit::Str(lit) => {
                let index = index_of_field_with_name(variant, &lit.value(), meta)?;
                Ok(IncludeFieldAttr { index })
            }
            Lit::Int(lit) => {
                let index = index_of_field_with_index(variant, lit.base10_parse()?, meta)?;
                Ok(IncludeFieldAttr { index })
            }
            _ => Err(Error::new_spanned(
                meta,
                "expected number or string literal!",
            )),
        },
        _ => Err(Error::new_spanned(meta, "unsupported literal!")),
    }
}

fn include_attr(variant: &Variant, meta: &ParseNestedMeta) -> Result<IncludeAttr, Error> {
    let mut attr = IncludeAttr::default();
    let content;
    syn::parenthesized!(content in meta.input);

    let metas = content.parse_terminated(Meta::parse, Token![,])?;

    for meta in metas {
        match &meta {
            Meta::Path(_) => return Err(Error::new_spanned(meta, "not supported!")),
            Meta::List(_) => return Err(Error::new_spanned(meta, "not supported!")),
            Meta::NameValue(name_value) => {
                if name_value.path.is_ident(FIELD_ATTR) {
                    attr.field = include_field_attr(variant, name_value)?;
                } else {
                    return Err(Error::new_spanned(meta, "not supported!"));
                }
            }
        }
    }

    Ok(attr)
}

fn attrs(variant: &Variant) -> Result<VariantAttrs, Error> {
    let mut attrs = VariantAttrs::default();

    for attr in &variant.attrs {
        if !attr.path().is_ident(NAMESPACE_ATTR) {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident(EXCLUDE_ATTR) {
                attrs.exclude = Some(exclude_attr(variant, &meta)?);
                Ok(())
            } else if meta.path.is_ident(INCLUDE_ATTR) {
                attrs.include = Some(include_attr(variant, &meta)?);
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
