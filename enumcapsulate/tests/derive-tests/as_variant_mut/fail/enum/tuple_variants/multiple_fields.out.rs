use enumcapsulate::derive::AsVariantMut;
pub struct VariantA;
pub enum Enum {
    VariantA(VariantA),
    VariantB(i32, u32),
}
