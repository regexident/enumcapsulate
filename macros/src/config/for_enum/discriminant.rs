use syn::meta::ParseNestedMeta;

use crate::attr::{NAME, REPR};

#[derive(Clone, Default)]
pub(crate) struct DiscriminantConfig {
    repr: Option<syn::Type>,
    ident: Option<syn::Ident>,
}

impl DiscriminantConfig {
    pub(crate) fn parse(
        &mut self,
        meta: &ParseNestedMeta,
        _item_enum: &syn::ItemEnum,
    ) -> Result<(), syn::Error> {
        meta.parse_nested_meta(|meta| {
            if meta.path.is_ident(REPR) {
                if self.repr.is_some() {
                    return Err(meta.error("repr already specified"));
                }

                self.repr = Some(meta.value()?.parse()?);
            } else if meta.path.is_ident(NAME) {
                if self.ident.is_some() {
                    return Err(meta.error("name already specified"));
                }

                self.ident = Some(meta.value()?.parse()?);
            } else {
                return Err(meta.error("unsupported discriminant attribute"));
            }

            Ok(())
        })
    }

    pub(crate) fn repr(&self) -> Option<&syn::Type> {
        self.repr.as_ref()
    }

    pub(crate) fn ident(&self) -> Option<&syn::Ident> {
        self.ident.as_ref()
    }
}
