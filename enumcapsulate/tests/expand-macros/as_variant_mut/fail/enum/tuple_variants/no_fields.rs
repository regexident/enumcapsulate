use enumcapsulate::derive::AsVariantMut;

pub struct VariantA;

#[derive(AsVariantMut)]
pub enum Enum {
    VariantA(VariantA),
    VariantB(),
}
