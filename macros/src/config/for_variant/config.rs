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

impl Default for FieldSelector {
    fn default() -> Self {
        Self::Index(0)
    }
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

    pub(crate) fn selected_field<'a>(
        &self,
        fields: &'a syn::Fields,
    ) -> Result<Option<(&'a syn::Field, usize)>, syn::Error> {
        let default_field_selector = FieldSelector::default();
        let field_selector = self.field.as_ref().unwrap_or(&default_field_selector);

        if fields.is_empty() {
            return Ok(None);
        }

        match field_selector {
            FieldSelector::Name(name) => {
                let (index, field) = fields
                    .iter()
                    .enumerate()
                    .find(|(_, field)| {
                        field
                            .ident
                            .as_ref()
                            .map(|ident| ident == name)
                            .unwrap_or(false)
                    })
                    .expect("field name should have been rejected");

                Ok(Some((field, index)))
            }
            FieldSelector::Index(index) => {
                let field = fields
                    .iter()
                    .nth(*index)
                    .expect("field index should have been rejected");

                Ok(Some((field, *index)))
            }
        }
    }

    pub(crate) fn discriminant(&self) -> Option<&DiscriminantConfig> {
        self.discriminant.as_ref()
    }

    pub(crate) fn exclude(&self) -> Option<&ExcludeConfig> {
        self.exclude.as_ref()
    }

    #[allow(dead_code)]
    pub(crate) fn field(&self) -> Option<&FieldSelector> {
        self.field.as_ref()
    }
}
