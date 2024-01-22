use enumcapsulate::derive::AsVariantRef;
pub struct VariantA;
pub enum Enum {
    VariantA(VariantB),
    VariantB(i32, u32),
}
