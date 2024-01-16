#[cfg(feature = "macros")]
pub use enumcapsulate_macros as macros;

#[cfg(feature = "derive")]
pub mod derive {
    pub use super::macros::{AsVariantMut, AsVariantRef, From, FromVariant, IntoVariant, TryInto};
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

pub trait VariantDowncast {
    fn as_variant_downcast_ref<T>(&self) -> Option<&T>
    where
        Self: AsVariantRef<T>,
    {
        self.as_variant_ref()
    }

    fn as_variant_downcast_mut<T>(&mut self) -> Option<&mut T>
    where
        Self: AsVariantMut<T>,
    {
        self.as_variant_mut()
    }

    fn as_variant_downcast<T>(&self) -> Option<T>
    where
        T: Clone,
        Self: AsVariantRef<T>,
    {
        self.as_variant()
    }

    fn into_variant_downcast<T>(self) -> Result<T, Self>
    where
        Self: IntoVariant<T>,
    {
        self.into_variant()
    }
}
