//! Traits and corresponding derive macros for enum-based encapsulation.

#[cfg(feature = "macros")]
pub use enumcapsulate_macros::*;

/// Creates an instance of `Self` from the unambiguous field type of one of its variants.
pub trait FromVariant<T> {
    /// Converts to this type from the variant's type.
    fn from_variant(variant: T) -> Self;
}

/// Provides owned access to the current variant's field.
pub trait AsVariant<T> {
    /// Returns a the inner value as `T` if it is of type `T`, or `None` if it isn’t.
    fn as_variant(&self) -> Option<T>;
}

/// Provides borrowed access to the current variant's field.
pub trait AsVariantRef<T> {
    /// Returns a the inner value as `&T` if it is of type `T`, or `None` if it isn’t.
    fn as_variant_ref(&self) -> Option<&T>;
}

/// Provides mutable borrowed access to the current variant's field.
pub trait AsVariantMut<T> {
    /// Returns a the inner value as `&mut T` if it is of type `T`, or `None` if it isn’t.
    fn as_variant_mut(&mut self) -> Option<&mut T>;
}

/// Returns the current variant's field, consuming `self`.
pub trait IntoVariant<T>: Sized {
    /// Returns the inner value if it is of type `T`, or `Err(self)` if it isn’t.
    fn into_variant(self) -> Result<T, Self>;
}

/// Convenience umbrella trait used to do a cheap
///
/// - reference-to-reference
/// - mutable-to-mutable reference
/// - reference-to-value
/// - value-to-value
///
/// conversion between an outer enum's and its inner variant's type.
pub trait VariantDowncast {
    /// Returns some reference to the inner value if it is of type `T`, or `None`` if it isn’t.
    fn as_variant_downcast_ref<T>(&self) -> Option<&T>
    where
        Self: AsVariantRef<T>,
    {
        self.as_variant_ref()
    }

    /// Returns some mutable reference to the inner value if it is of type `T`, or `None` if it isn’t.
    fn as_variant_downcast_mut<T>(&mut self) -> Option<&mut T>
    where
        Self: AsVariantMut<T>,
    {
        self.as_variant_mut()
    }

    /// Returns a copy of the inner value if it is of type `T`, or `None`` if it isn’t.
    fn as_variant_downcast<T>(&self) -> Option<T>
    where
        T: Clone,
        Self: AsVariant<T>,
    {
        self.as_variant()
    }

    /// Converts this type into the variant's type.
    fn into_variant_downcast<T>(self) -> Result<T, Self>
    where
        Self: IntoVariant<T>,
    {
        self.into_variant()
    }
}

/// Used to check type of an enum's inner variant's type.
pub trait IsVariant {
    /// Returns `true` if `T` matches the variant's type, otherwise `false`.
    fn is_variant<T>(&self) -> bool
    where
        T: 'static + ?Sized;
}

/// Used to obtain an enum variant's discriminant.
pub trait VariantDiscriminant {
    type Discriminant: Eq;

    /// Returns the variant's discriminant.
    fn variant_discriminant(&self) -> Self::Discriminant;
}
