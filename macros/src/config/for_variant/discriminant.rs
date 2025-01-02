use syn::meta::ParseNestedMeta;

use crate::attr::{NAME, NESTED, VALUE};

#[derive(Clone, Default)]
pub(crate) struct DiscriminantConfig {
    value: Option<syn::Expr>,
    name: Option<syn::Ident>,
    nested: bool,
}

impl DiscriminantConfig {
    pub(crate) fn parse(
        &mut self,
        meta: &ParseNestedMeta,
        variant: &syn::Variant,
    ) -> Result<(), syn::Error> {
        meta.parse_nested_meta(|meta| {
            if meta.path.is_ident(VALUE) {
                if self.value.is_some() {
                    return Err(meta.error("`value = …` already specified"));
                } else if self.nested {
                    return Err(meta.error("conflicting with use of `nesting`"));
                }

                self.value = Some(meta.value()?.parse()?);
            } else if meta.path.is_ident(NAME) {
                if self.name.is_some() {
                    return Err(meta.error("`name = …` already specified"));
                }

                self.name = Some(meta.value()?.parse()?);
            } else if meta.path.is_ident(NESTED) {
                if matches!(variant.fields, syn::Fields::Unit) {
                    return Err(meta.error("no field found on variant"));
                } else if self.nested {
                    return Err(meta.error("`nested` already specified"));
                } else if self.value.is_some() {
                    return Err(meta.error("conflicting with use of `value = …`"));
                }

                self.nested = true;
            } else {
                return Err(meta.error("unsupported discriminant attribute"));
            }

            Ok(())
        })
    }

    pub(crate) fn expr(&self) -> Option<&syn::Expr> {
        self.value.as_ref()
    }

    pub(crate) fn ident(&self) -> Option<&syn::Ident> {
        self.name.as_ref()
    }

    pub(crate) fn nested(&self) -> bool {
        self.nested
    }
}
