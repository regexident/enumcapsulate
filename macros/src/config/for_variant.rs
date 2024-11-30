mod discriminant;
mod standard;
mod variant_discriminant;

pub(crate) use self::variant_discriminant::VariantDiscriminantDeriveVariantConfig;

use self::{discriminant::DiscriminantConfig, standard::VariantConfig};

pub(crate) type FromDeriveVariantConfig = VariantConfig;
pub(crate) type TryIntoDeriveVariantConfig = VariantConfig;
pub(crate) type FromVariantDeriveVariantConfig = VariantConfig;
pub(crate) type IntoVariantDeriveVariantConfig = VariantConfig;
pub(crate) type AsVariantDeriveVariantConfig = VariantConfig;
pub(crate) type AsVariantMutDeriveVariantConfig = VariantConfig;
pub(crate) type AsVariantRefDeriveVariantConfig = VariantConfig;
