use enumcapsulate::derive::AsVariantRef;

pub struct VariantA;

#[derive(AsVariantRef)]
pub enum Enum {
    VariantA(VariantA),
    VariantB {},
}
