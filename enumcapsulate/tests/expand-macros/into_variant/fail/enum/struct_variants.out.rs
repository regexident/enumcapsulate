use enumcapsulate::derive::IntoVariant;
pub struct VariantA;
pub enum Enum {
    VariantA(VariantA),
    VariantB {},
}
