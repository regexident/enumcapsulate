use enumcapsulate::derive::From;
pub struct VariantA;
pub enum Enum {
    VariantA(VariantA),
    VariantB,
}
