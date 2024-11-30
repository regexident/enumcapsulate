use syn::{meta::ParseNestedMeta, parse::Parse, punctuated::Punctuated};

#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub(crate) struct ExcludeConfig {
    idents: Vec<syn::Ident>,
}

impl ExcludeConfig {
    pub(crate) fn parse(
        &mut self,
        meta: &ParseNestedMeta,
        accept_empty: bool,
    ) -> Result<(), syn::Error> {
        let input = meta.input;

        let lookahead = input.lookahead1();
        if lookahead.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);

            let punctuated: Punctuated<syn::Ident, syn::Token![,]> =
                content.parse_terminated(syn::Ident::parse, syn::Token![,])?;

            let idents = Vec::from_iter(punctuated);

            if idents.is_empty() {
                return Err(meta.error("expected non-empty list"));
            }

            let () = validate_derive_names(idents.iter())?;

            self.idents.extend(idents);
        } else if !accept_empty {
            return Err(meta.error("expected list"));
        }

        Ok(())
    }

    pub(crate) fn is_wildcard(&self) -> bool {
        self.idents.is_empty()
    }

    #[allow(dead_code)]
    pub(crate) fn is_included(&self, name: &str) -> bool {
        !self.is_excluded(name)
    }

    pub(crate) fn is_excluded(&self, name: &str) -> bool {
        if self.is_wildcard() {
            true
        } else {
            self.idents.iter().any(|ident| ident == name)
        }
    }

    #[allow(dead_code)]
    pub(crate) fn iter(&self) -> impl Iterator<Item = &syn::Ident> {
        self.idents.iter()
    }

    #[allow(dead_code)]
    pub(crate) fn into_iter(self) -> impl Iterator<Item = syn::Ident> {
        self.idents.into_iter()
    }
}

fn validate_derive_names<'a>(
    idents: impl Iterator<Item = &'a syn::Ident>,
) -> Result<(), syn::Error> {
    use crate::macro_name::*;

    const DERIVE_NAMES: &[&str] = &[
        FROM,
        TRY_INTO,
        FROM_VARIANT,
        INTO_VARIANT,
        AS_VARIANT,
        AS_VARIANT_REF,
        AS_VARIANT_MUT,
        VARIANT_DOWNCAST,
    ];

    let mut error: Option<syn::Error> = None;

    let unrecognized = idents.filter(|&ident| !DERIVE_NAMES.iter().any(|name| ident == name));

    for ident in unrecognized {
        let ident_err = syn::Error::new_spanned(ident, "unrecognized identifier");
        if let Some(error) = error.as_mut() {
            error.combine(ident_err);
        } else {
            error = Some(ident_err)
        }
    }

    if let Some(err) = error {
        return Err(err);
    }

    Ok(())
}
