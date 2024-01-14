#[cfg(feature = "derive")]
pub use enumcapsulate_macros as derive;

pub trait AsVariantRef<T> {
    fn as_variant_ref(&self) -> Option<&T>;
}

pub trait AsVariantMut<T> {
    fn as_variant_mut(&mut self) -> Option<&mut T>;
}

pub trait AsVariant<T> {
    fn as_variant(&self) -> Option<T>;
}

pub trait IntoVariant<T>: Sized {
    fn into_variant(self) -> Result<T, Self>;
}

pub trait Downcast {
    fn as_downcast_ref<T>(&self) -> Option<&T>
    where
        Self: AsVariantRef<T>;

    fn as_downcast_mut<T>(&mut self) -> Option<&mut T>
    where
        Self: AsVariantMut<T>;

    fn as_downcast<T>(&self) -> Option<T>
    where
        Self: AsVariant<T>;

    fn into_downcast<T>(self) -> Result<T, Self>
    where
        Self: IntoVariant<T>;
}
