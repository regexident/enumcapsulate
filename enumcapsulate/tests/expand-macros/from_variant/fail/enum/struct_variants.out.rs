use enumcapsulate::derive::FromVariant;
pub struct VariantA;
pub enum Enum {
    VariantA(VariantA),
    VariantB {},
}
