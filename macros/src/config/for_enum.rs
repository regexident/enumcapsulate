mod discriminant;
mod encapsulate;
mod standard;
mod variant_discriminant;

pub(crate) use self::encapsulate::EncapsulateDeriveEnumConfig;
pub(crate) use self::variant_discriminant::VariantDiscriminantDeriveEnumConfig;

use self::{discriminant::DiscriminantConfig, standard::EnumConfig};

pub(crate) type FromDeriveEnumConfig = EnumConfig;
pub(crate) type TryIntoDeriveEnumConfig = EnumConfig;
pub(crate) type FromVariantDeriveEnumConfig = EnumConfig;
pub(crate) type IntoVariantDeriveEnumConfig = EnumConfig;
pub(crate) type AsVariantDeriveEnumConfig = EnumConfig;
pub(crate) type AsVariantMutDeriveEnumConfig = EnumConfig;
pub(crate) type AsVariantRefDeriveEnumConfig = EnumConfig;
pub(crate) type VariantDowncastDeriveEnumConfig = EnumConfig;
