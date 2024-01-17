use enumcapsulate::derive::From;

pub struct Variant;

#[derive(From)]
pub enum Enum {
    VariantA(VariantA),
    VariantB {},
}
