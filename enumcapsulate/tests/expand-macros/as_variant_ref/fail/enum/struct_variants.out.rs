use enumcapsulate::derive::AsVariantRef;
pub struct VariantA;
pub enum Enum {
    VariantA(VariantA),
    VariantB {},
}
