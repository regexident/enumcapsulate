use enumcapsulate::derive::FromVariant;

pub struct VariantA;

#[derive(FromVariant)]
pub enum Enum {
    VariantA(VariantA),
    VariantB {},
}
