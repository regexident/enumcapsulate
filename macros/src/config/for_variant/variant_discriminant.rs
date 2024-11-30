use crate::{attr::DISCRIMINANT, parse_enumcapsulate_attrs};

use super::DiscriminantConfig;

#[derive(Clone, Default)]
pub(crate) struct VariantDiscriminantDeriveVariantConfig {
    discriminant: Option<DiscriminantConfig>,
}

impl VariantDiscriminantDeriveVariantConfig {
    pub(crate) fn from_variant(variant: &syn::Variant) -> Result<Self, syn::Error> {
        let mut this = Self::default();

        parse_enumcapsulate_attrs(&variant.attrs, |meta| {
            if meta.path.is_ident(DISCRIMINANT) {
                let mut discriminant = this.discriminant.take().unwrap_or_default();

                discriminant.parse(&meta, variant)?;

                this.discriminant = Some(discriminant);
            }

            Ok(())
        })?;

        Ok(this)
    }

    pub fn expr(&self) -> Option<&syn::Expr> {
        self.discriminant
            .as_ref()
            .and_then(|discriminant| discriminant.expr())
    }

    pub fn ident(&self) -> Option<&syn::Ident> {
        self.discriminant
            .as_ref()
            .and_then(|discriminant| discriminant.ident())
    }
}
