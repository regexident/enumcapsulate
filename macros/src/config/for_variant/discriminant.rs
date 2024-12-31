use syn::meta::ParseNestedMeta;

use crate::attr::{NAME, NESTED, VALUE};

#[derive(Clone, Default)]
pub(crate) struct DiscriminantConfig {
    expr: Option<syn::Expr>,
    ident: Option<syn::Ident>,
    nested: Option<syn::Path>,
}

impl DiscriminantConfig {
    pub(crate) fn parse(
        &mut self,
        meta: &ParseNestedMeta,
        variant: &syn::Variant,
    ) -> Result<(), syn::Error> {
        meta.parse_nested_meta(|meta| {
            if meta.path.is_ident(VALUE) {
                if self.expr.is_some() {
                    return Err(meta.error("`value = …` already specified"));
                } else if self.nested.is_some() {
                    return Err(meta.error("conflicting with use of `nesting = …`"));
                }

                self.expr = Some(meta.value()?.parse()?);
            } else if meta.path.is_ident(NAME) {
                if self.ident.is_some() {
                    return Err(meta.error("`name = …` already specified"));
                }

                self.ident = Some(meta.value()?.parse()?);
            } else if meta.path.is_ident(NESTED) {
                if matches!(variant.fields, syn::Fields::Unit) {
                    return Err(meta.error("no field found on variant"));
                } else if self.nested.is_some() {
                    return Err(meta.error("`nested = …` already specified"));
                } else if self.expr.is_some() {
                    return Err(meta.error("conflicting with use of `value = …`"));
                }

                self.nested = Some(meta.value()?.parse()?);
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

    pub(crate) fn nested(&self) -> Option<&syn::Path> {
        self.nested.as_ref()
    }
}
