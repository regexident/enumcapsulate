use syn::LitStr;

use crate::{
    attr::{DISCRIMINANT, EXCLUDE, FIELD},
    config::ExcludeConfig,
    parse_enumcapsulate_attrs,
};

use super::DiscriminantConfig;

#[derive(Clone, Eq, PartialEq, Debug)]
pub(crate) enum FieldSelector {
    Name(String),
    Index(usize),
}

#[derive(Clone, Default)]
pub(crate) struct VariantConfig {
    discriminant: Option<DiscriminantConfig>,
    exclude: Option<ExcludeConfig>,
    field: Option<FieldSelector>,
}

impl VariantConfig {
    pub(crate) fn from_variant(variant: &syn::Variant) -> Result<Self, syn::Error> {
        let mut this = Self::default();

        parse_enumcapsulate_attrs(&variant.attrs, |meta| {
            if meta.path.is_ident(DISCRIMINANT) {
                let mut discriminant = this.discriminant.take().unwrap_or_default();

                discriminant.parse(&meta, variant)?;

                this.discriminant = Some(discriminant);
            } else if meta.path.is_ident(EXCLUDE) {
                let mut exclude = this.exclude.take().unwrap_or_default();

                exclude.parse(&meta, true)?;

                this.exclude = Some(exclude);
            } else if meta.path.is_ident(FIELD) {
                if this.field.is_some() {
                    return Err(meta.error("field already specified"));
                }

                let value = meta.value()?;

                if value.peek(LitStr) {
                    let lit: syn::LitStr = value.parse()?;
                    this.field = Some(FieldSelector::Name(lit.value()));
                } else {
                    let lit: syn::LitInt = value.parse()?;
                    this.field = Some(FieldSelector::Index(lit.base10_parse()?));
                }
            }

            Ok(())
        })?;

        Ok(this)
    }

    pub(crate) fn position_of_selected_field(
        &self,
        fields: &syn::Fields,
    ) -> Result<usize, syn::Error> {
        if let Some(selector) = &self.field {
            match selector {
                FieldSelector::Name(name) => {
                    let position = fields
                        .iter()
                        .position(|field| {
                            field
                                .ident
                                .as_ref()
                                .map(|ident| ident == name)
                                .unwrap_or(false)
                        })
                        .expect("field name should have been rejected");

                    return Ok(position);
                }
                FieldSelector::Index(index) => {
                    if *index >= fields.len() {
                        return Err(syn::Error::new_spanned(fields, "field index out of bounds"));
                    }
                    return Ok(*index);
                }
            }
        };

        match fields.len() {
            0 => Err(syn::Error::new_spanned(fields, "no fields")),
            1 => Ok(0),
            _ => Err(syn::Error::new_spanned(
                fields,
                "multiple fields, please disambiguate via helper attribute",
            )),
        }
    }

    pub(crate) fn discriminant(&self) -> Option<&DiscriminantConfig> {
        self.discriminant.as_ref()
    }

    pub(crate) fn exclude(&self) -> Option<&ExcludeConfig> {
        self.exclude.as_ref()
    }
}
