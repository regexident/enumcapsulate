use syn::meta::ParseNestedMeta;

use crate::attr::{NAME, VALUE};

#[derive(Clone, Default)]
pub(crate) struct DiscriminantConfig {
    expr: Option<syn::Expr>,
    ident: Option<syn::Ident>,
}

impl DiscriminantConfig {
    pub(crate) fn parse(
        &mut self,
        meta: &ParseNestedMeta,
        _variant: &syn::Variant,
    ) -> Result<(), syn::Error> {
        meta.parse_nested_meta(|meta| {
            if meta.path.is_ident(VALUE) {
                if self.expr.is_some() {
                    return Err(meta.error("value already specified"));
                }

                self.expr = Some(meta.value()?.parse()?);
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

    pub(crate) fn expr(&self) -> Option<&syn::Expr> {
        self.expr.as_ref()
    }

    pub(crate) fn ident(&self) -> Option<&syn::Ident> {
        self.ident.as_ref()
    }
}
