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

#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub(crate) struct MacroSelectionConfig {
    pub idents: Vec<syn::Ident>,
}

impl MacroSelectionConfig {
    pub fn is_empty(&self) -> bool {
        self.idents.is_empty()
    }

    pub fn contains(&self, name: &str) -> bool {
        self.idents.iter().any(|ident| ident == name)
    }

    pub fn extend_idents(&mut self, iter: impl IntoIterator<Item = syn::Ident>) {
        self.idents.extend(iter);
    }
}

pub(crate) type EnumExcludeConfig = MacroSelectionConfig;
pub(crate) type VariantExcludeConfig = MacroSelectionConfig;

#[derive(Clone, Default)]
pub(crate) struct EncapsulateEnumConfig {
    // #[enumcapsulate(exclude(…))]
    pub exclude: Option<EnumExcludeConfig>,
}

impl EncapsulateEnumConfig {
    pub fn is_included(&self, name: &str) -> bool {
        !self.is_excluded(name)
    }

    pub fn is_excluded(&self, name: &str) -> bool {
        self.exclude
            .as_ref()
            .map(|excluded| excluded.contains(name))
            .unwrap_or(false)
    }
}

#[derive(Clone, Default)]
pub(crate) struct EnumConfig {}

#[derive(Clone, Default)]
pub(crate) struct VariantConfig {
    // #[enumcapsulate(exclude(…))]
    pub exclude: Option<VariantExcludeConfig>,

    // #[enumcapsulate(field(…))]
    pub field: Option<VariantFieldConfig>,
}

impl VariantConfig {
    #[allow(dead_code)]
    pub fn is_included(&self, name: &str) -> bool {
        !self.is_excluded(name)
    }

    pub fn is_excluded(&self, name: &str) -> bool {
        let Some(excluded) = &self.exclude else {
            return false;
        };

        if excluded.is_empty() {
            true
        } else {
            excluded.contains(name)
        }
    }
}

pub(crate) fn encapsulate_config_for_enum(
    enum_item: &syn::ItemEnum,
) -> Result<EncapsulateEnumConfig, syn::Error> {
    let mut config = EncapsulateEnumConfig::default();

    parse_enumcapsulate_attrs(&enum_item.attrs, |meta| {
        if meta.path.is_ident(attr::EXCLUDE) {
            // #[enumcapsulate(exclude(…))]

            let mut exclude = config.exclude.take().unwrap_or_default();

            let idents = macro_selection_config_for_enum(&meta)?.idents;

            if idents.is_empty() {
                return Err(meta.error("expected list"));
            }

            exclude.extend_idents(idents);

            config.exclude = Some(exclude);
        } else {
            return Err(meta.error("unrecognized attribute"));
        }

        Ok(())
    })?;

    Ok(config)
}

pub(crate) fn config_for_enum(enum_item: &syn::ItemEnum) -> Result<EnumConfig, syn::Error> {
    let config = EnumConfig::default();

    parse_enumcapsulate_attrs(&enum_item.attrs, |meta| {
        if meta.path.is_ident(attr::EXCLUDE) {
            // #[enumcapsulate(exclude(…))]

            meta.parse_nested_meta(|_meta| {
                // Here we're not interested in any of the existing
                // sub-attributes, but we need to parse the list anyway.

                Ok(())
            })?;
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

            exclude.extend_idents(macro_selection_config_for_variant(&meta)?.idents);

            config.exclude = Some(exclude);
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

pub(crate) fn macro_selection_config_for_enum(
    meta: &syn::meta::ParseNestedMeta<'_>,
) -> Result<MacroSelectionConfig, syn::Error> {
    let idents = parse_idents_from_meta_list(meta)?;

    let recognized = RECOGNIZED_ENUM_LEVEL_MACROS;
    ensure_only_recognized_ident_names(&idents, recognized)?;

    Ok(MacroSelectionConfig { idents })
}

pub(crate) fn macro_selection_config_for_variant(
    meta: &syn::meta::ParseNestedMeta<'_>,
) -> Result<MacroSelectionConfig, syn::Error> {
    let idents = parse_idents_from_meta_list(meta)?;

    let recognized = RECOGNIZED_VARIANT_LEVEL_MACROS;
    ensure_only_recognized_ident_names(&idents, recognized)?;

    Ok(MacroSelectionConfig { idents })
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
