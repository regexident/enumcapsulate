use syn::{parse::Parse, punctuated::Punctuated};

use crate::{attr, macro_name};

static RECOGNIZED_ENUM_LEVEL_MACROS: &[&str] = &[
    macro_name::FROM,
    macro_name::TRY_INTO,
    macro_name::FROM_VARIANT,
    macro_name::INTO_VARIANT,
    macro_name::AS_VARIANT,
    macro_name::AS_VARIANT_REF,
    macro_name::AS_VARIANT_MUT,
    macro_name::VARIANT_DISCRIMINANT,
    macro_name::VARIANT_DOWNCAST,
];

static RECOGNIZED_VARIANT_LEVEL_MACROS: &[&str] = &[
    macro_name::FROM,
    macro_name::TRY_INTO,
    macro_name::FROM_VARIANT,
    macro_name::INTO_VARIANT,
    macro_name::AS_VARIANT,
    macro_name::AS_VARIANT_REF,
    macro_name::AS_VARIANT_MUT,
];

#[derive(Clone, Eq, PartialEq, Debug)]
pub(crate) enum VariantFieldConfig {
    Name(String),
    Index(usize),
}

#[derive(Clone, Default, Debug)]
pub(crate) struct EnumConfig {
    // #[enumcapsulate(exclude(…))]
    pub exclude: Option<Vec<syn::Ident>>,
}

impl EnumConfig {
    pub fn is_excluded(&self, name: &str) -> bool {
        if let Some(excluded) = &self.exclude {
            return excluded.iter().any(|ident| ident == name);
        } else {
            false
        }
    }
}

#[derive(Clone, Default, Debug)]
pub(crate) struct VariantConfig {
    // #[enumcapsulate(exclude(…))]
    pub exclude: Option<Vec<syn::Ident>>,

    // #[enumcapsulate(include(…))]
    pub include: Option<Vec<syn::Ident>>,

    // #[enumcapsulate(field(… = …))]
    pub field: Option<VariantFieldConfig>,
}

impl VariantConfig {
    pub fn is_excluded(&self, name: &str, config: &EnumConfig) -> bool {
        if self.is_excluded_explicitly(name) {
            assert!(!self.is_included_explicitly(name));
            return true;
        }

        if self.is_included_explicitly(name) {
            return false;
        }

        config.is_excluded(name)
    }

    pub fn is_excluded_explicitly(&self, name: &str) -> bool {
        if let Some(excluded) = &self.exclude {
            if excluded.is_empty() {
                if let Some(included) = &self.include {
                    return !included.iter().any(|ident| ident == name);
                } else {
                    return true;
                }
            }

            excluded.iter().any(|ident| ident == name)
        } else {
            false
        }
    }

    pub fn is_included_explicitly(&self, name: &str) -> bool {
        if let Some(included) = &self.include {
            if included.is_empty() {
                if let Some(excluded) = &self.exclude {
                    return !excluded.iter().any(|ident| ident == name);
                } else {
                    return true;
                }
            }

            included.iter().any(|ident| ident == name)
        } else {
            false
        }
    }
}

pub(crate) fn config_for_enum_with_attrs(
    enum_attrs: &[syn::Attribute],
) -> Result<EnumConfig, syn::Error> {
    let mut config = EnumConfig::default();

    parse_enumcapsulate_attrs(enum_attrs, |meta| {
        if meta.path.is_ident(attr::EXCLUDE) {
            // #[enumcapsulate(exclude(…))]

            let mut exclude = config.exclude.take().unwrap_or_default();
            exclude.extend(macro_idents_for_enum(&meta)?.into_iter());
            config.exclude = Some(exclude);
        } else {
            return Err(meta.error("unrecognized attribute"));
        }

        Ok(())
    })?;

    Ok(config)
}

pub(crate) fn config_for_variant(variant: &syn::Variant) -> Result<VariantConfig, syn::Error> {
    let mut config = VariantConfig::default();

    let fields = match &variant.fields {
        syn::Fields::Named(fields) => fields.named.iter().collect(),
        syn::Fields::Unnamed(fields) => fields.unnamed.iter().collect(),
        syn::Fields::Unit => vec![],
    };

    parse_enumcapsulate_attrs(&variant.attrs, |meta| {
        if meta.path.is_ident(attr::EXCLUDE) {
            // #[enumcapsulate(exclude(…))]

            let mut exclude = config.exclude.take().unwrap_or_default();
            let conflicting = config.include.as_deref().unwrap_or(&[]);

            exclude.extend(macro_idents_for_variant(&meta, conflicting)?.into_iter());
            config.exclude = Some(exclude);
        } else if meta.path.is_ident(attr::INCLUDE) {
            // #[enumcapsulate(include(…))]

            let mut include = config.include.take().unwrap_or_default();
            let conflicting = config.exclude.as_deref().unwrap_or(&[]);

            include.extend(macro_idents_for_variant(&meta, conflicting)?.into_iter());
            config.include = Some(include);
        } else if meta.path.is_ident(attr::FIELD) {
            // #[enumcapsulate(field(…))]
            meta.parse_nested_meta(|meta| {
                if meta.path.is_ident(attr::NAME) {
                    // #[enumcapsulate(field(name = "…"))]

                    if !matches!(&variant.fields, syn::Fields::Named(_)) {
                        return Err(meta.error("no named fields in variant"));
                    }

                    let lit: syn::LitStr = meta.value()?.parse()?;
                    let name = lit.value();

                    let field_idents: Vec<_> = fields
                        .iter()
                        .filter_map(|&field| field.ident.as_ref())
                        .collect();

                    if field_idents.is_empty() {
                        return Err(meta.error("no named fields in variant"));
                    }

                    let field_exists = field_idents.into_iter().any(|ident| ident == &name);

                    if !field_exists {
                        return Err(meta.error("field not found in variant"));
                    }

                    config.field = Some(VariantFieldConfig::Name(name));

                    Ok(())
                } else if meta.path.is_ident(attr::INDEX) {
                    // #[enumcapsulate(field(index = …))]

                    if fields.is_empty() {
                        return Err(meta.error("no fields in variant"));
                    }

                    let lit: syn::LitInt = meta.value()?.parse()?;
                    let index = lit.base10_parse()?;

                    if fields.len() <= index {
                        return Err(meta.error("field index out of bounds"));
                    }

                    config.field = Some(VariantFieldConfig::Index(index));

                    Ok(())
                } else {
                    return Err(meta.error("unrecognized attribute"));
                }
            })?;
        } else {
            return Err(meta.error("unrecognized attribute"));
        }

        Ok(())
    })?;

    Ok(config)
}

pub(crate) fn parse_enumcapsulate_attrs(
    attrs: &[syn::Attribute],
    logic: impl FnMut(syn::meta::ParseNestedMeta) -> Result<(), syn::Error>,
) -> Result<(), syn::Error> {
    let mut logic = logic;

    for attr in attrs {
        if !attr.path().is_ident(attr::NAMESPACE) {
            continue;
        }

        // #[enumcapsulate(…)]
        attr.parse_nested_meta(&mut logic)?;
    }

    Ok(())
}

pub(crate) fn macro_idents_for_enum(
    meta: &syn::meta::ParseNestedMeta<'_>,
) -> Result<Vec<syn::Ident>, syn::Error> {
    let idents = parse_idents_from_meta_list(meta)?;

    let recognized = RECOGNIZED_ENUM_LEVEL_MACROS;
    ensure_only_recognized_ident_names(&idents, recognized)?;

    Ok(idents)
}

pub(crate) fn macro_idents_for_variant(
    meta: &syn::meta::ParseNestedMeta<'_>,
    conflict_list: &[syn::Ident],
) -> Result<Vec<syn::Ident>, syn::Error> {
    let idents = parse_idents_from_meta_list(meta)?;

    let recognized = RECOGNIZED_VARIANT_LEVEL_MACROS;
    ensure_only_recognized_ident_names(&idents, recognized)?;

    ensure_no_conflicting_idents(&idents, conflict_list)?;

    Ok(idents)
}

pub(crate) fn ensure_only_recognized_ident_names(
    idents: &[syn::Ident],
    recognized: &[&str],
) -> Result<(), syn::Error> {
    let mut error: Option<syn::Error> = None;

    let unrecognized = idents
        .iter()
        .filter(|&ident| !recognized.iter().any(|recognized| ident == recognized));

    for ident in unrecognized {
        let ident_err = syn::Error::new_spanned(ident, "unrecognized macro derive");
        if let Some(error) = error.as_mut() {
            error.combine(ident_err);
        } else {
            error = Some(ident_err)
        }
    }

    if let Some(err) = error {
        return Err(err);
    }

    Ok(())
}

pub(crate) fn ensure_no_conflicting_idents(
    idents: &[syn::Ident],
    conflicting: &[syn::Ident],
) -> Result<(), syn::Error> {
    let mut error: Option<syn::Error> = None;

    let conflicting = idents
        .iter()
        .filter(|&ident| conflicting.iter().any(|conflicting| ident == conflicting));

    for ident in conflicting {
        let ident_err = syn::Error::new_spanned(ident, "conflicting macro derive");
        if let Some(error) = error.as_mut() {
            error.combine(ident_err);
        } else {
            error = Some(ident_err)
        }
    }

    if let Some(err) = error {
        return Err(err);
    }

    Ok(())
}

pub(crate) fn parse_idents_from_meta_list(
    meta: &syn::meta::ParseNestedMeta<'_>,
) -> Result<Vec<syn::Ident>, syn::Error> {
    let mut idents = vec![];

    let lookahead = meta.input.lookahead1();
    if lookahead.peek(syn::token::Paren) {
        let content;
        syn::parenthesized!(content in meta.input);
        let punctuated: Punctuated<syn::Ident, syn::Token![,]> =
            content.parse_terminated(syn::Ident::parse, syn::Token![,])?;

        idents.extend(punctuated);
    }

    Ok(idents)
}

#[cfg(test)]
mod tests;
