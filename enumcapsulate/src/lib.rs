#[cfg(feature = "macros")]
pub use enumcapsulate_macros as macros;

#[cfg(feature = "derive")]
pub mod derive {
    pub use super::macros::{AsVariantMut, AsVariantRef, FromVariant, IntoVariant};
}

pub trait FromVariant<T> {
    fn from_variant(variant: T) -> Self;
}

pub trait AsVariantRef<T> {
    fn as_variant_ref(&self) -> Option<&T>;

    fn as_variant(&self) -> Option<T>
    where
        T: Clone,
    {
        self.as_variant_ref().cloned()
    }
}

pub trait AsVariantMut<T> {
    fn as_variant_mut(&mut self) -> Option<&mut T>;
}

pub trait IntoVariant<T>: Sized {
    fn into_variant(self) -> Result<T, Self>;
}
