use enumcapsulate::derive::AsVariant;

pub struct VariantA;

#[derive(AsVariant)]
pub enum Enum {
    VariantA(VariantA),
    VariantB(i32, u32),
}
