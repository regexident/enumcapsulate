#[cfg(feature = "macros")]
pub use enumcapsulate_macros as macros;

#[cfg(feature = "derive")]
pub mod derive {
    pub use super::macros::{AsVariantRef, FromVariant};
}

pub trait FromVariant<T> {
    fn from_variant(variant: T) -> Self;
}

pub trait AsVariantRef<T> {
    fn as_variant_ref(&self) -> Option<&T>;
}
