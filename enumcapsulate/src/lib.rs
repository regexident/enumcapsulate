//! Traits and corresponding derive macros for enum-based encapsulation.

/// Macros.
#[cfg(feature = "macros")]
pub use enumcapsulate_macros as macros;

/// Derive macros.
#[cfg(feature = "derive")]
pub mod derive {
    pub use super::macros::{
        AsVariant, AsVariantMut, AsVariantRef, Encapsulate, From, FromVariant, IntoVariant,
        IsVariant, TryInto,
    };
}

/// Used to do variant-to-enum conversions
/// between an outer enum's and its inner variant's type.
pub trait FromVariant<T> {
    /// Converts to this type from the variant's type.
    fn from_variant(variant: T) -> Self;
}

/// Used to do a cheap reference-to-reference reference conversion
/// between an outer enum's and its inner variant's type.
pub trait AsVariantRef<T> {
    /// Returns some reference to the inner value if it is of type `T`, or `None`` if it isn’t.
    fn as_variant_ref(&self) -> Option<&T>;

    /// Returns a copy of the inner value if it is of type `T`, or `None`` if it isn’t.
    fn as_variant(&self) -> Option<T>
    where
        T: Clone,
    {
        self.as_variant_ref().cloned()
    }
}

/// Used to do a cheap mutable-to-mutable reference conversion
/// between an outer enum's and its inner variant's type.
pub trait AsVariantMut<T> {
    /// Returns some mutable reference to the inner value if it is of type `T`, or `None` if it isn’t.
    fn as_variant_mut(&mut self) -> Option<&mut T>;
}

/// Used to do enum-to-variant conversions
/// between an outer enum's and its inner variant's type.
pub trait IntoVariant<T>: Sized {
    /// Converts this type into the variant's type.
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
        Self: AsVariantRef<T>,
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
