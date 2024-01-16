#[cfg(feature = "macros")]
pub use enumcapsulate_macros as macros;

#[cfg(feature = "derive")]
pub mod derive {
    pub use super::macros::{AsVariantMut, AsVariantRef, FromVariant};
}

pub trait FromVariant<T> {
    fn from_variant(variant: T) -> Self;
}

pub trait AsVariantRef<T> {
    fn as_variant_ref(&self) -> Option<&T>;
}

pub trait AsVariantMut<T> {
    fn as_variant_mut(&mut self) -> Option<&mut T>;
}
