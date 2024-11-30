use crate::{attr::EXCLUDE, config::ExcludeConfig, parse_enumcapsulate_attrs};

#[derive(Clone, Default)]
pub(crate) struct EncapsulateDeriveEnumConfig {
    exclude: Option<ExcludeConfig>,
}

impl EncapsulateDeriveEnumConfig {
    pub fn is_included(&self, name: &str) -> bool {
        !self.is_excluded(name)
    }

    pub fn is_excluded(&self, name: &str) -> bool {
        self.exclude
            .as_ref()
            .map(|excluded| excluded.is_excluded(name))
            .unwrap_or(false)
    }
}

impl EncapsulateDeriveEnumConfig {
    pub(crate) fn from_enum(item_enum: &syn::ItemEnum) -> Result<Self, syn::Error> {
        let mut this = Self::default();

        parse_enumcapsulate_attrs(&item_enum.attrs, |meta| {
            if meta.path.is_ident(EXCLUDE) {
                let mut exclude = this.exclude.take().unwrap_or_default();

                exclude.parse(&meta, false)?;

                this.exclude = Some(exclude);
            }

            Ok(())
        })?;

        Ok(this)
    }
}
