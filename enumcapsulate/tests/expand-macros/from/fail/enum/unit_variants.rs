use enumcapsulate::derive::From;

pub struct VariantA;

#[derive(From)]
pub enum Enum {
    VariantA(VariantA),
    VariantB,
}
