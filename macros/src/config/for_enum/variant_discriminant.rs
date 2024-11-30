use crate::{attr::DISCRIMINANT, parse_enumcapsulate_attrs};

use super::DiscriminantConfig;

#[derive(Clone, Default)]
pub(crate) struct VariantDiscriminantDeriveEnumConfig {
    discriminant: Option<DiscriminantConfig>,
}

impl VariantDiscriminantDeriveEnumConfig {
    pub(crate) fn from_enum(item_enum: &syn::ItemEnum) -> Result<Self, syn::Error> {
        let mut this = Self::default();

        parse_enumcapsulate_attrs(&item_enum.attrs, |meta| {
            if meta.path.is_ident(DISCRIMINANT) {
                let mut discriminant = this.discriminant.take().unwrap_or_default();

                discriminant.parse(&meta, item_enum)?;

                this.discriminant = Some(discriminant);
            }

            Ok(())
        })?;

        Ok(this)
    }

    pub fn repr(&self) -> Option<&syn::Type> {
        self.discriminant
            .as_ref()
            .and_then(|discriminant| discriminant.repr())
    }

    pub fn ident(&self) -> Option<&syn::Ident> {
        self.discriminant
            .as_ref()
            .and_then(|discriminant| discriminant.ident())
    }
}
