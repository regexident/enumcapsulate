use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use crate::config::VariantFieldConfig;

pub(crate) mod attr {
    pub(crate) const EXCLUDE: &str = "exclude";
    pub(crate) const FIELD: &str = "field";
    pub(crate) const INDEX: &str = "index";
    pub(crate) const NAME: &str = "name";
    pub(crate) const NAMESPACE: &str = "enumcapsulate";
}

pub(crate) mod macro_name {
    pub(crate) const FROM: &str = "From";
    pub(crate) const TRY_INTO: &str = "TryInto";

    pub(crate) const FROM_VARIANT: &str = "FromVariant";
    pub(crate) const INTO_VARIANT: &str = "IntoVariant";

    pub(crate) const AS_VARIANT: &str = "AsVariant";
    pub(crate) const AS_VARIANT_REF: &str = "AsVariantRef";
    pub(crate) const AS_VARIANT_MUT: &str = "AsVariantMut";

    pub(crate) const VARIANT_DISCRIMINANT: &str = "VariantDiscriminant";
    pub(crate) const VARIANT_DOWNCAST: &str = "VariantDowncast";
}

pub(crate) fn position_of_selected_field(
    fields: &syn::Fields,
    config: Option<&VariantFieldConfig>,
) -> Result<Option<usize>, syn::Error> {
    let field_count = fields.len();

    if field_count > 1 && config.is_none() {
        return Err(syn::Error::new_spanned(
            fields,
            "multiple ambiguous fields in variant",
        ));
    }

    if let Some(field_config) = config {
        match field_config {
            VariantFieldConfig::Name(name) => {
                let position = fields
                    .iter()
                    .position(|field| {
                        field
                            .ident
                            .as_ref()
                            .map(|ident| ident == name)
                            .unwrap_or(false)
                    })
                    .expect("field name should have been rejected");

                return Ok(Some(position));
            }
            VariantFieldConfig::Index(index) => {
                assert!(
                    field_count > *index,
                    "field index should have been rejected"
                );
                return Ok(Some(*index));
            }
        }
    }

    match field_count {
        0 => Ok(None),
        1 => Ok(Some(0)),
        _ => Err(syn::Error::new_spanned(fields, "more than one field")),
    }
}

#[track_caller]
pub(crate) fn tokenstream<F>(f: F) -> TokenStream
where
    F: FnOnce() -> Result<TokenStream2, syn::Error>,
{
    match f() {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    }
    .into()
}
