use enumcapsulate::derive::AsVariant;
pub struct VariantA;
pub enum Enum {
    VariantA(VariantA),
    VariantB {},
}
