use crate::{
    attr::{DISCRIMINANT, EXCLUDE},
    config::ExcludeConfig,
    parse_enumcapsulate_attrs,
};

use super::DiscriminantConfig;

#[derive(Clone, Default)]
pub(crate) struct EnumConfig {
    exclude: Option<ExcludeConfig>,
    discriminant: Option<DiscriminantConfig>,
}

impl EnumConfig {
    pub fn is_included(&self, name: &str) -> bool {
        !self.is_excluded(name)
    }

    pub fn is_excluded(&self, name: &str) -> bool {
        self.exclude
            .as_ref()
            .map(|excluded| excluded.is_excluded(name))
            .unwrap_or(false)
    }

    pub(crate) fn discriminant(&self) -> Option<&DiscriminantConfig> {
        self.discriminant.as_ref()
    }
}

impl EnumConfig {
    pub(crate) fn from_enum(item_enum: &syn::ItemEnum) -> Result<Self, syn::Error> {
        let mut this = Self::default();

        parse_enumcapsulate_attrs(&item_enum.attrs, |meta| {
            if meta.path.is_ident(DISCRIMINANT) {
                let mut discriminant = this.discriminant.take().unwrap_or_default();

                discriminant.parse(&meta, item_enum)?;

                this.discriminant = Some(discriminant);
            } else if meta.path.is_ident(EXCLUDE) {
                let mut exclude = this.exclude.take().unwrap_or_default();

                exclude.parse(&meta, false)?;

                this.exclude = Some(exclude);
            }

            Ok(())
        })?;

        Ok(this)
    }
}
