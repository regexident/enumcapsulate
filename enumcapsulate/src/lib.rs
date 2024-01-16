#[cfg(feature = "macros")]
pub use enumcapsulate_macros as macros;

#[cfg(feature = "derive")]
pub mod derive {
    pub use super::macros::FromVariant;
}

pub trait FromVariant<T> {
    fn from_variant(variant: T) -> Self;
}
