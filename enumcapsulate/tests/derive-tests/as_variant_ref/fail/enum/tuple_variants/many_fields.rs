use enumcapsulate::derive::AsVariantRef;

pub struct VariantA;

#[derive(AsVariantRef)]
pub enum Enum {
    VariantA(VariantB),
    VariantB(i32, u32),
}
