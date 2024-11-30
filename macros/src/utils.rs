use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

pub(crate) mod attr {
    #![allow(dead_code)]

    pub(crate) const DISCRIMINANT: &str = "discriminant";
    pub(crate) const EXCLUDE: &str = "exclude";
    pub(crate) const FIELD: &str = "field";
    pub(crate) const NAME: &str = "name";
    pub(crate) const NAMESPACE: &str = "enumcapsulate";
    pub(crate) const REPR: &str = "repr";
    pub(crate) const VALUE: &str = "value";
}

pub(crate) mod macro_name {
    #![allow(dead_code)]

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
