use enumcapsulate::derive::IntoVariant;

pub struct VariantA;

#[derive(IntoVariant)]
pub enum Enum {
    VariantA(VariantA),
    VariantB(i32, u32),
}
